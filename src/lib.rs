//! Temperature analysis of electronic systems.
//!
//! The library provides an exponential-integrator-based solver for systems of
//! differential-algebraic equations modeling temperature of electronic systems.
//! The initial thermal system is
//!
//! ```math
//!     dQex
//! C * ---- + G * (Qex - Qamb) = M * P
//!      dt
//!
//! Q = M^T * Qex
//! ```
//!
//! where `C` and `G` are the thermal capacitance and conductance matrices,
//! respectively; `Qex` and `Q` are the temperature vectors of all thermal nodes
//! and those that correspond to the processing elements, respectively; `Qamb`
//! is the ambient temperature; `P` is the power vector of the processing
//! elements; and `M` is a rectangular diagonal matrix whose diagonal elements
//! equal to unity.
//!
//! The transformed system is
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
//! S = D^(-1) * (Qex - Qamb),
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
//! the following recurrence:
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

#![feature(phase)]

extern crate serialize;

extern crate hotspot;
extern crate matrix;

#[cfg(test)] mod test;

/// Temperature analysis.
pub struct Analysis {
    /// The configuration of the analysis.
    pub config: Config,
    /// The thermal system that the analysis is based on.
    pub system: System,
}

/// A configuration of the analysis.
#[deriving(Decodable)]
pub struct Config {
    /// The configuration of the HotSpot model.
    pub hotspot: HotSpot,
    /// The sampling interval in seconds. It is the time between two successive
    /// samples of power or temperature in power or temperature profiles,
    /// respectively. In the formulas given in the general description of the
    /// library, it is referred to as `Δt`.
    pub time_step: f64,
    /// The temperature of the ambience in Kelvin.
    pub ambience: f64,
}

/// A configuration of the HotSpot model.
#[deriving(Decodable)]
pub struct HotSpot {
    /// The floorplan file of the platform to analyze.
    pub floorplan: String,
    /// A configuration file of HotSpot (`hotspot.config`).
    pub config: String,
    /// A line of parameters overwriting the parameters in the above file.
    pub params: String,
}

/// A model of heat transfer in an electronic system.
#[allow(non_snake_case)]
pub struct System {
    /// The number of active thermal nodes (processing elements).
    pub cores: uint,
    /// The number of thermal nodes.
    pub nodes: uint,

    #[allow(dead_code)] U: Vec<f64>,
    #[allow(dead_code)] L: Vec<f64>,

    D: Vec<f64>,
    E: Vec<f64>,
    F: Vec<f64>,
}

impl Analysis {
    /// Sets up the anlysis set up according to the given configuration.
    #[allow(non_snake_case)]
    pub fn new(config: Config) -> Result<Analysis, &'static str> {
        use hotspot::Circuit;
        use matrix::multiply;
        use matrix::decomp::sym_eig;
        use std::mem::{forget, transmute_copy};

        let circuit = try!(Circuit::new(config.hotspot.floorplan.as_slice(),
                                        config.hotspot.config.as_slice(),
                                        config.hotspot.params.as_slice()));

        let (nc, nn) = (circuit.cores, circuit.nodes);

        // Reusing the memory allocated in `circuit`.
        let mut A = circuit.conductance;
        let mut D = circuit.capacitance;

        for i in range(0u, nn) {
            D[i] = (1.0 / D[i]).sqrt();
        }
        for i in range(0u, nn) {
            for j in range(0u, nn) {
                A[j * nn + i] = -1.0 * D[i] * D[j] * A[j * nn + i];
            }
        }

        // Reusing `A` to store `U` as `sym_eig` allows for this.
        let mut U: Vec<f64> = unsafe { transmute_copy(&A) };
        let mut L = Vec::from_elem(nn, 0.0);

        if sym_eig(A.as_slice(), U.as_mut_slice(), L.as_mut_slice(), nn).is_err() {
            return Err("cannot perform the eigendecomposition");
        }

        unsafe {
            // `A` is no longer needed, and `U` is to be moved out. Both,
            // however, point at the same chunk of memory. So, forget `A`!
            forget(A);
        }

        let dt = config.time_step;

        let mut coef = Vec::from_elem(nn, 0.0);
        let mut temp = Vec::from_elem(nn * nn, 0.0);

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

        // At this point, only `nn * nc` elements of `temp` are utilized.
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

    /// Sets up the analysis according to the given configuration file.
    #[inline]
    pub fn load(path: Path) -> Result<Analysis, &'static str> {
        Analysis::new(try!(Config::load(path)))
    }

    /// Performs transient temperature analysis.
    ///
    /// `P` is an input power profile given as an `nc`-by-`ns` matrix where `nc`
    /// is the number of cores, and `ns` is the number of time steps; see
    /// `time_step` in `Config`. `Q` is the corresponding output temperature
    /// profile, which is given as an `nc`-by-`ns` matrix. `S` is an optional
    /// `nn`-by-`ns` matrix, where `nn` is the number of thermal nodes, for the
    /// internal usage of the function to prevent repetitive memory allocation
    /// if the analysis is to be performed several times.
    #[allow(non_snake_case)]
    pub fn compute_transient(&self, P: &[f64], Q: &mut [f64], S: &mut [f64], ns: uint) {
        use matrix::{multiply, multiply_add};
        use std::mem::transmute_copy;

        let (nc, nn) = (self.system.cores, self.system.nodes);

        let D = self.system.D.as_slice();
        let E = self.system.E.as_slice();
        let F = self.system.F.as_slice();

        multiply(F, P, S, nn, nc, ns);

        // In the loop below, we need to perform operations on certain slices
        // of `S` and overwrite them with new data. `multiply_add` allows the
        // third and fourth arguments (one of the inputs and the only output,
        // respectively) to overlap. So, let us be more efficient.
        let Z: &mut [f64] = unsafe { transmute_copy(&S) };

        for i in range(1u, ns) {
            let (j, k) = ((i - 1) * nn, i * nn);
            multiply_add(E, S.slice(j, k), S.slice(k, k + nn), Z.slice_mut(k, k + nn), nn, nn, 1);
        }

        for i in range(0u, nc) {
            for j in range(0u, ns) {
                Q[nc * j + i] = D[i] * S[nn * j + i] + self.config.ambience;
            }
        }
    }
}

impl Config {
    /// Reads a configuration structure from a JSON file.
    pub fn load(path: Path) -> Result<Config, &'static str> {
        use serialize::json;
        use std::io::File;

        let content = match File::open(&path).read_to_string() {
            Ok(content) => content,
            Err(error) => return Err(error.desc),
        };
        match json::decode(content.as_slice()) {
            Ok(config) => Ok(config),
            Err(_) => Err("cannot parse the input file"),
        }
    }
}
