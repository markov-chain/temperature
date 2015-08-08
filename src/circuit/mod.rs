//! Thermal circuits.

#[cfg(feature = "hotspot")]
mod hotspot;

#[cfg(feature = "hotspot")]
pub use self::hotspot::HotSpot;

#[cfg(feature = "threed-ice")]
#[path = "3d-ice.rs"]
mod threed_ice;

#[cfg(feature = "threed-ice")]
pub use self::threed_ice::ThreeDICE;
