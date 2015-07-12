//! Temperature analysis of electronic systems.
//!
//! ## Model
//!
//! Temperature analysis is based on the well-known analogy between electrical
//! and thermal circuits. For an electronic system of interest, an equivalent
//! thermal RC circuit is constructed. The circuit is composed of `nodes`
//! thermal nodes. A subset of `cores` (out of `nodes`) thermal nodes
//! corresponds to the power-dissipating elements of the electronic system and
//! is referred to as active.
//!
//! The thermal behavior of the electronic system is modeled using the following
//! system of differential-algebraic equations:
//!
//! ```math
//!     dQall
//! C * ----- + G * (Qall - Qamb) = M * P
//!      dt
//!
//! Q = M^T * Qall
//! ```
//!
//! where
//!
//! * `C` is an `nodes`-by-`nodes` diagonal matrix of thermal capacitance;
//!
//! * `G` is an `nodes`-by-`nodes` symmetric, positive-definite matrix of
//!   thermal conductance;
//!
//! * `Qall` is an `nodes`-element temperature vector of all thermal nodes;
//!
//! * `Q` is a `cores`-element temperature vector of the active thermal nodes;
//!
//! * `Qamb` is a `cores`-element temperature vector of the ambience;
//!
//! * `P` is a `cores`-element power vector of the active thermal nodes; and
//!
//! * `M` is an `nodes`-by-`cores` rectangular diagonal matrix whose diagonal
//!   elements equal to unity.
//!
//! ## Solution
//!
//! The original thermal system is transformed as follows:
//!
//! ```math
//! dS
//! -- = A * S + B * P
//! dt
//!
//! Q = B^T * S + Qamb
//! ```
//!
//! where
//!
//! ```math
//! S = D^(-1) * (Qall - Qamb),
//! A = -D * G * D,
//! B = D * M, and
//! D = C^(-1/2).
//! ```
//!
//! The eigendecomposition of `A`, which is real and symmetric, is
//!
//! ```math
//! A = U * diag(Λ) * U^T.
//! ```
//!
//! The solution of the system for a short time interval `[0, Δt]` is based on
//! the following equation:
//!
//! ```math
//! S(t) = E * S(0) + F * P(0)
//! ```
//!
//! where
//!
//! ```math
//! E = exp(A * Δt) = U * diag(exp(λi * Δt)) * U^T and
//! F = A^(-1) * (exp(A * Δt) - I) * B
//!   = U * diag((exp(λi * Δt) - 1) / λi) * U^T * B.
//! ```
//!
//! `Δt` is referred to as the time step. In order to find the temperature
//! profile corresponding to the whole time span of interest, the time span is
//! split into small intervals, and the above equation is successively applied
//! to each of these small intervals.

#[cfg(test)]
extern crate assert;

extern crate linear;

pub mod model;

#[cfg(test)]
mod tests;

/// Temperature analysis.
pub struct Analysis {
    config: Config,
    system: System,
}

/// A thermal RC circuit.
pub struct Circuit {
    /// The number of active thermal nodes.
    pub cores: usize,
    /// The number of all thermal nodes.
    pub nodes: usize,
    /// An `nodes`-element vector of thermal capacitance.
    pub capacitance: Vec<f64>,
    /// An `nodes`-by-`nodes` matrix of thermal conductance.
    pub conductance: Vec<f64>,
}

/// A configuration of temperature analysis.
pub struct Config {
    /// The sampling interval of power and temperature profiles in seconds.
    pub time_step: f64,
    /// The temperature of the ambience in Celsius or Kelvin.
    pub ambience: f64,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
struct System {
    cores: usize,
    nodes: usize,
    U: Vec<f64>,
    L: Vec<f64>,
    D: Vec<f64>,
    E: Vec<f64>,
    F: Vec<f64>,
}

impl Analysis {
    /// Set up the analysis for a particular problem.
    #[allow(non_snake_case)]
    pub fn new(circuit: Circuit, config: Config) -> Result<Analysis, &'static str> {
        use linear::multiply;
        use linear::symmetric_eigen;

        let (nc, nn) = (circuit.cores, circuit.nodes);

        let mut D = circuit.capacitance; // recycle
        for i in (0..nn) {
            D[i] = (1.0 / D[i]).sqrt();
        }

        let mut A = circuit.conductance; // recycle
        for i in (0..nn) {
            for j in (0..nn) {
                A[j * nn + i] = -1.0 * D[i] * D[j] * A[j * nn + i];
            }
        }

        let mut U = A; // recycle
        let mut L = vec![0.0; nn];
        match symmetric_eigen(&mut U, &mut L) {
            Err(_) => return Err("cannot perform the eigendecomposition"),
            _ => {},
        }

        let dt = config.time_step;

        let mut temp1 = vec![0.0; nn];
        let mut temp2 = vec![0.0; nn * nn];

        for i in (0..nn) {
            temp1[i] = (dt * L[i]).exp();
        }
        for i in (0..nn) {
            for j in (0..nn) {
                temp2[j * nn + i] = temp1[i] * U[i * nn + j];
            }
        }

        let mut E = vec![0.0; nn * nn];
        multiply(1.0, &U, &temp2, 1.0, &mut E, nn);

        for i in (0..nn) {
            temp1[i] = (temp1[i] - 1.0) / L[i];
        }
        for i in (0..nn) {
            for j in (0..nc) {
                temp2[j * nn + i] = temp1[i] * U[i * nn + j] * D[j];
            }
        }

        let mut F = vec![0.0; nn * nc];
        multiply(1.0, &U, &temp2[..(nn * nc)], 1.0, &mut F, nn);

        Ok(Analysis {
            config: config,
            system: System {
                cores: nc,
                nodes: nn,
                L: L,
                U: U,
                D: D,
                E: E,
                F: F,
            },
        })
    }

    /// Perform transient temperature analysis.
    ///
    /// ## Arguments
    ///
    /// * `P` is an input power profile given as a `cores`-by-`steps` matrix;
    ///
    /// * `Q` is the corresponding output temperature profile given as a
    ///   `cores`-by-`steps` matrix;
    ///
    /// * `S` is an `nodes`-by-`steps` matrix for the internal usage; and
    ///
    /// * `steps` is the number of time steps; see `time_step` in `Config`.
    ///
    /// The structure of the arguments allows one to avoid repetitive memory
    /// allocation if the analysis is to be performed several times.
    #[allow(non_snake_case)]
    pub fn compute_transient(&self, P: &[f64], Q: &mut [f64], S: &mut [f64], steps: usize) {
        use linear::multiply;
        use std::mem::transmute_copy;

        let (nc, nn) = (self.system.cores, self.system.nodes);

        let D = &self.system.D;
        let E = &self.system.E;
        let F = &self.system.F;

        multiply(1.0, F, P, 1.0, S, nn);

        // In the loop below, we need to perform operations on certain slices of
        // `S` and overwrite them with new data. Let us be efficient.
        let Z: &mut [f64] = unsafe { transmute_copy(&S) };

        for i in (1..steps) {
            let (j, k) = ((i - 1) * nn, i * nn);
            multiply(1.0, E, &S[j..k], 1.0, &mut Z[k..k + nn], nn);
        }

        let ambience = self.config.ambience;
        for i in (0..nc) {
            for j in (0..steps) {
                Q[nc * j + i] = D[i] * S[nn * j + i] + ambience;
            }
        }
    }
}

impl std::default::Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            time_step: 1e-3,
            ambience: 318.15,
        }
    }
}
