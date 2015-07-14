//! Temperature analysis of electronic systems.
//!
//! ## Model
//!
//! Temperature analysis is based on the well-known analogy between electrical
//! and thermal circuits. For an electronic system with `cores` processing
//! elements, an equivalent thermal circuit is constructed, which is composed of
//! `nodes` thermal nodes.
//!
//! The thermal behavior of the system is modeled using the following system of
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
//! * `Cth` is the thermal-capacitance matrix, which is a `nodes × nodes`
//!   diagonal matrix;
//!
//! * `Gth` is the thermal-conductance matrix, which is a `nodes × nodes`
//!   symmetric, positive-definite matrix;
//!
//! * `T` is the temperature of the thermal nodes, which is a `nodes`-element
//!   vector;
//!
//! * `Tamb` is the ambient temperature, which is a `nodes`-element vector;
//!
//! * `P` is the power dissipation of the processing elements, which is a
//!   `cores`-element vector;
//!
//! * `Mp` is the distribution matrix mapping the power dissipation of the
//!   processing elements onto the thermal nodes, which is a `nodes × cores`
//!   matrix;
//!
//! * `Q` is the temperature of interest, which is an `outputs`-element vector;
//!
//! * `Mq` is the aggregation matrix mapping the temperature of the thermal
//!   nodes onto the temperature of interest, which is an `outputs × nodes`
//!   matrix.
//!
//! ## Solution
//!
//! The system is transformed as follows:
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
//! The solution makes use of the assumption that `Δt` is short enough so that
//! the power dissipation does not change much within `[0, Δt]`. In order to
//! compute the temperature profile corresponding for the whole time span of
//! interest, the time span is split into small subintervals, and the above
//! equation is successively applied to each of them.

#[cfg(test)]
extern crate assert;
extern crate linear;
extern crate matrix;

use std::{error, fmt};

/// A thermal circuit.
pub struct Circuit {
    /// The number of processing elements.
    pub cores: usize,
    /// The number of thermal nodes.
    pub nodes: usize,
    /// The thermal capacitance matrix.
    pub capacitance: matrix::Diagonal<f64>,
    /// The thermal conductance matrix.
    pub conductance: matrix::Compressed<f64>,
}

/// A configuration of temperature analysis.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The temperature of the ambience in Celsius or Kelvin.
    pub ambience: f64,
    /// The sampling interval of power and temperature profiles in seconds.
    pub time_step: f64,
}

/// An error.
#[derive(Debug)]
pub struct Error(String);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

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

impl std::default::Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            ambience: 318.15,
            time_step: 1e-3,
        }
    }
}

mod analysis;
pub mod model;

pub use analysis::Analysis;
