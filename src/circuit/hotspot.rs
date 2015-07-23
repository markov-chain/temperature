extern crate hotspot;

use matrix::format::{Compressed, Diagonal};
use std::path::Path;

use {Circuit, Result};

/// The HotSpot model.
pub struct HotSpot;

impl HotSpot {
    /// Construct a thermal circuit.
    pub fn new<F: AsRef<Path>, C: AsRef<Path>>(floorplan: F, config: C) -> Result<Circuit> {
        let hotspot::Circuit { units, nodes, capacitance, conductance } = {
            ok!(hotspot::Circuit::new(floorplan, config))
        };
        Ok(Circuit {
            capacitance: capacitance,
            conductance: conductance,
            distribution: Compressed::from(Diagonal::from_vec((nodes, units), vec![1.0; units])),
            aggregation: Compressed::from(Diagonal::from_vec((units, nodes), vec![1.0; units])),
        })
    }
}
