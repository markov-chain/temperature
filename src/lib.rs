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

#![feature(phase)]

extern crate matrix;

pub mod model;

#[cfg(test)] mod tests;

/// Temperature analysis.
pub struct Analysis {
    config: Config,
    system: System,
}

/// A thermal RC circuit.
pub struct Circuit {
    /// The number of active thermal nodes.
    pub cores: uint,
    /// The number of all thermal nodes.
    pub nodes: uint,
    /// An `nodes`-element vector of thermal capacitance.
    pub capacitance: Vec<f64>,
    /// An `nodes`-by-`nodes` matrix of thermal conductance.
    pub conductance: Vec<f64>,
}

/// A configuration of temperature analysis.
pub struct Config {
    /// The sampling interval of power and temperature profiles in seconds.
    pub time_step: f64,
    /// The temperature of the ambience in Kelvin.
    pub ambience: f64,
}

#[allow(non_snake_case)]
struct System {
    cores: uint,
    nodes: uint,
    #[allow(dead_code)] U: Vec<f64>,
    #[allow(dead_code)] L: Vec<f64>,
    D: Vec<f64>,
    E: Vec<f64>,
    F: Vec<f64>,
}

impl Analysis {
    /// Set up the analysis for a particular problem.
    #[allow(non_snake_case)]
    pub fn new(circuit: Circuit, config: Config) -> Result<Analysis, &'static str> {
        use std::num::Float;

        use matrix::multiply;
        use matrix::decomp::sym_eig;

        let (nc, nn) = (circuit.cores, circuit.nodes);

        let mut D = circuit.capacitance; // recycle
        for i in range(0u, nn) {
            D[i] = (1.0 / D[i]).sqrt();
        }

        let mut A = circuit.conductance; // recycle
        for i in range(0u, nn) {
            for j in range(0u, nn) {
                A[j * nn + i] = -1.0 * D[i] * D[j] * A[j * nn + i];
            }
        }

        let mut U = Vec::from_elem(nn * nn, 0.0);
        let mut L = Vec::from_elem(nn, 0.0);
        if sym_eig(A.as_slice(), U.as_mut_slice(), L.as_mut_slice(), nn).is_err() {
            return Err("cannot perform the eigendecomposition");
        }

        let dt = config.time_step;

        let mut coef = Vec::from_elem(nn, 0.0);
        let mut temp = A; // recycle

        for i in range(0u, nn) {
            coef[i] = (dt * L[i]).exp();
        }
        for i in range(0u, nn) {
            for j in range(0u, nn) {
                temp[j * nn + i] = coef[i] * U[i * nn + j];
            }
        }

        let mut E = Vec::from_elem(nn * nn, 0.0);
        multiply(U.as_slice(), temp.as_slice(), E.as_mut_slice(), nn, nn, nn);

        for i in range(0u, nn) {
            coef[i] = (coef[i] - 1.0) / L[i];
        }
        for i in range(0u, nn) {
            for j in range(0u, nc) {
                temp[j * nn + i] = coef[i] * U[i * nn + j] * D[j];
            }
        }

        let mut F = Vec::from_elem(nn * nc, 0.0);
        multiply(U.as_slice(), temp.as_slice(), F.as_mut_slice(), nn, nn, nc);

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
    pub fn compute_transient(&self, P: &[f64], Q: &mut [f64], S: &mut [f64], steps: uint) {
        use matrix::{multiply, multiply_add};
        use std::mem::transmute_copy;

        let (nc, nn) = (self.system.cores, self.system.nodes);

        let D = self.system.D.as_slice();
        let E = self.system.E.as_slice();
        let F = self.system.F.as_slice();

        multiply(F, P, S, nn, nc, steps);

        // In the loop below, we need to perform operations on certain slices
        // of `S` and overwrite them with new data. `multiply_add` allows the
        // third and fourth arguments (one of the inputs and the only output,
        // respectively) to overlap. So, let us be more efficient.
        let Z: &mut [f64] = unsafe { transmute_copy(&S) };

        for i in range(1u, steps) {
            let (j, k) = ((i - 1) * nn, i * nn);
            multiply_add(E, S.slice(j, k), S.slice(k, k + nn),
                         Z.slice_mut(k, k + nn), nn, nn, 1);
        }

        for i in range(0u, nc) {
            for j in range(0u, steps) {
                Q[nc * j + i] = D[i] * S[nn * j + i] + self.config.ambience;
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
