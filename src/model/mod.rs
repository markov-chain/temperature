//! Models for the construction of thermal circuits.

#[cfg(feature = "hotspot")]
mod hotspot;

#[cfg(feature = "hotspot")]
pub use self::hotspot::HotSpot;
