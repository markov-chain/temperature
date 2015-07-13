extern crate hotspot;

use std::path::Path;

use {Circuit, Result};

/// The HotSpot model.
pub struct HotSpot;

impl HotSpot {
    /// Construct a thermal circuit.
    pub fn new<F: AsRef<Path>, C: AsRef<Path>>(floorplan: F, config: C) -> Result<Circuit> {
        let circuit = ok!(hotspot::Circuit::new(floorplan, config));
        Ok(Circuit {
            cores: circuit.cores,
            nodes: circuit.nodes,
            capacitance: circuit.capacitance,
            conductance: circuit.conductance,
        })
    }
}
