//! The HotSpot model.

extern crate hotspot;

use std::io::Result;
use std::path::Path;

use Circuit;

/// Construct a thermal RC circuit using the HotSpot model.
pub fn new(floorplan: &Path, config: &Path, params: &str) -> Result<Circuit> {
    let hotspot::Circuit { cores, nodes, capacitance, conductance } = {
        try!(hotspot::Circuit::new(floorplan, config, &params))
    };
    Ok(Circuit {
        cores: cores,
        nodes: nodes,
        capacitance: capacitance,
        conductance: conductance,
    })
}
