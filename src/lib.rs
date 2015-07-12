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

use std::{error, fmt};

mod analysis;
pub mod model;

pub use analysis::Analysis;

/// A thermal RC circuit.
pub trait Circuit {
    /// The number of active thermal nodes.
    fn cores(&self) -> usize;
    /// The number of all thermal nodes.
    fn nodes(&self) -> usize;
    /// An `nodes`-element vector of thermal capacitance.
    fn capacitance(&self) -> Vec<f64>;
    /// An `nodes`-by-`nodes` matrix of thermal conductance.
    fn conductance(&self) -> Vec<f64>;
}

/// A configuration of temperature analysis.
pub struct Config {
    /// The sampling interval of power and temperature profiles in seconds.
    pub time_step: f64,
    /// The temperature of the ambience in Celsius or Kelvin.
    pub ambience: f64,
}

/// An error.
#[derive(Debug)]
pub struct Error(String);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.0
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
