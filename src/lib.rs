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

extern crate serialize;

extern crate hotspot;
extern crate matrix;

pub use config::Config;

mod config;

#[cfg(test)]
mod test;
