//! The HotSpot model.

extern crate hotspot;

use std::io::Result;
use std::path::Path;

/// Construct a thermal RC circuit using the HotSpot model.
pub fn new(floorplan: &Path, config: &Path, params: &str) -> Result<::Circuit> {
    use std::mem::transmute;
    let circuit = try!(hotspot::Circuit::new(floorplan, config, &params));
    Ok(unsafe { transmute(circuit) })
}
