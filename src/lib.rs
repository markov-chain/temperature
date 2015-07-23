//! Temperature simulator.
//!
//! ## Model
//!
//! Temperature simulation is based on the well-known analogy between electrical
//! and thermal circuits. Given a system with `units` processing elements, an
//! equivalent thermal RC circuit with `nodes` thermal nodes is constructed. The
//! circuit is then used for modeling the thermal behavior of the system.
//! Concretely, the thermal behavior is described using the following system of
//! differential-algebraic equations:
//!
//! ```math
//!     dT
//! Cth -- + Gth (T - Tamb) = Mp P
//!     dt
//!
//! Q = Mq T
//! ```
//!
//! where
//!
//! * `Cth` is a `nodes × nodes` diagonal matrix of the thermal capacitance;
//!
//! * `Gth` is a `nodes × nodes` symmetric, positive-definite matrix of the
//!   thermal conductance;
//!
//! * `T` is a `nodes`-element vector of the temperature of the thermal nodes;
//!
//! * `Tamb` is a `nodes`-element vector of the ambient temperature;
//!
//! * `P` is a `units`-element vector of the power dissipation of the processing
//!   elements;
//!
//! * `Mp` is a `nodes × units` matrix that distributes the power dissipation of
//!   the processing elements onto the thermal nodes;
//!
//! * `Q` is a `spots`-element vector of the temperature of interest; and
//!
//! * `Mq` is a `spots × nodes` matrix that aggregates the temperature of the
//!   thermal nodes into the temperature of interest.
//!
//! ## Solution
//!
//! The original system is transformed into the following:
//!
//! ```math
//! dS
//! -- = A S + B P
//! dt
//!
//! Q = C S + Mq Tamb
//! ```
//!
//! where
//!
//! ```math
//! S = D^(-1) (T - Tamb),
//! A = -D Gth D,
//! B = D Mp,
//! C = Mq D, and
//! D = Cth^(-1/2).
//! ```
//!
//! The eigendecomposition of `A`, which is real and symmetric, is
//!
//! ```math
//! A = U diag(Λ) U^T.
//! ```
//!
//! For a short time interval `[0, Δt]`, the solution is obtained using the
//! following equation:
//!
//! ```math
//! S(t) = E S(0) + F P(0)
//! ```
//!
//! where
//!
//! ```math
//! E = exp(A Δt) = U diag(exp(λi Δt)) U^T and
//! F = A^(-1) (exp(A Δt) - I) B = U diag((exp(λi Δt) - 1) / λi) U^T B.
//! ```
//!
//! The solution makes use of the assumption that `Δt`, referred to as the time
//! step, is short enough so that the power dissipation does not change much
//! within `[0, Δt]`. In order to compute the temperature profile corresponding
//! for the whole time span of interest, the time span is split into small
//! subintervals, and the above equation is successively applied to each of
//! them.

#[cfg(test)]
extern crate assert;
extern crate matrix;

use std::{default, error, fmt, result};
use matrix::format::{Compressed, Diagonal};

/// A thermal circuit.
#[derive(Clone, Debug)]
pub struct Circuit {
    /// The thermal-capacitance matrix.
    pub capacitance: Diagonal<f64>,
    /// The thermal-conductance matrix.
    pub conductance: Compressed<f64>,
    /// The power-distribution matrix.
    pub distribution: Compressed<f64>,
    /// The temperature-aggregation matrix.
    pub aggregation: Compressed<f64>,
}

/// A configuration of temperature simulation.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The temperature of the ambience in Kelvin.
    pub ambience: f64,
    /// The time step of the simulator in seconds.
    pub time_step: f64,
}

/// An error.
#[derive(Clone, Debug)]
pub struct Error(String);

/// A result.
pub type Result<T> = result::Result<T, Error>;

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => return Err(::Error(error.to_string())),
        }
    );
);

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

impl default::Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            ambience: 318.15,
            time_step: 1e-3,
        }
    }
}

mod simulator;
pub mod circuit;

pub use simulator::Simulator;
