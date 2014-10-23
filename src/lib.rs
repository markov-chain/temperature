//! Temperature analysis of electronic systems.
//!
//! The library provides an exponential-integrator-based solver for systems of
//! differential-algebraic equations modeling temperature of electronic systems.
//!
//! The initial thermal system is
//!
//! ```math
//!     dQex
//! C * --- + G * (Qex - Qamb) = M * P
//!     dt
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

#![feature(macro_rules)]

extern crate serialize;

extern crate hotspot;
extern crate matrix;

pub use config::Config;

mod config;

#[cfg(test)]
mod test;

#[allow(non_snake_case)]
#[allow(dead_code)]
struct System {
    nc: uint,
    nn: uint,
    D: Vec<f64>,
    U: Vec<f64>,
    L: Vec<f64>,
    E: Vec<f64>,
    F: Vec<f64>,
}

#[allow(dead_code)]
pub struct Analysis {
    config: Config,
    system: System,
}

impl Analysis {
    /// Sets up the anlysis set up according to the given configuration.
    #[allow(non_snake_case)]
    pub fn new(config: Config) -> Result<Analysis, String> {
        use hotspot::Circuit;
        use matrix::{multiply, sym_eig};

        let circuit = match Circuit::new(config.hotspot.floorplan.as_slice(),
                                         config.hotspot.config.as_slice(),
                                         config.hotspot.params.as_slice()) {
            Ok(circuit) => circuit,
            Err(_) => return Err("cannot construct a thermal circuit".to_string()),
        };

        let (nc, nn) = (circuit.cores, circuit.nodes);

        // NOTE: Reusing the memory.
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

        // NOTE: Reusing the memory.
        let mut U: Vec<f64> = unsafe { ::std::mem::transmute_copy(&A) };
        let mut L = Vec::from_elem(nn, 0.0);

        match sym_eig(A.as_ptr(), U.as_mut_ptr(), L.as_mut_ptr(), nn) {
            Err(_) => return Err("cannot perform the eigendecomposition".to_string()),
            _ => {},
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
        multiply(U.as_ptr(), temp.as_ptr(), E.as_mut_ptr(), nn, nn, nn);

        // Technically, temp = temp.slice(0, nn * nc).
        for i in range(0u, nn) {
            coef[i] = (coef[i] - 1.0) / L[i];
        }
        for i in range(0u, nn) {
            for j in range(0u, nc) {
                temp[j * nn + i] = coef[i] * U[i * nn + j] * D[j];
            }
        }

        let mut F = Vec::from_elem(nn * nc, 0.0);
        multiply(U.as_ptr(), temp.as_ptr(), F.as_mut_ptr(), nn, nn, nc);

        Ok(Analysis {
            config: config,
            system: System { nc: nc, nn: nn, D: D, L: L, U: U, E: E, F: F },
        })
    }

    /// Sets up the analysis according to the given configuration file.
    #[inline]
    pub fn load(path: Path) -> Result<Analysis, String> {
        Analysis::new(try!(Config::load(path)))
    }
}
