//! The HotSpot model.

extern crate hotspot;

use super::super::Circuit;

/// Constructs a thermal RC circuit using the HotSpot model.
pub fn new(floorplan: &Path, config: &Path, params: &str) -> Result<Circuit, &'static str> {
    use std::mem::transmute;
    let circuit = try!(hotspot::Circuit::new(floorplan, config, params.as_slice()));
    Ok(unsafe { transmute(circuit) })
}
