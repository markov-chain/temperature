extern crate hotspot;

use matrix::format::{Compressed, Diagonal};
use std::path::Path;

use {Circuit, Result};

/// The HotSpot model.
pub struct HotSpot;

impl HotSpot {
    /// Construct a thermal circuit.
    pub fn new<F: AsRef<Path>, C: AsRef<Path>>(floorplan: F, config: C) -> Result<Circuit> {
        let hotspot::Circuit { cores, nodes, capacitance, conductance } = {
            ok!(hotspot::Circuit::new(floorplan, config))
        };
        Ok(Circuit {
            capacitance: capacitance,
            conductance: conductance,
            distribution: Compressed::from(Diagonal::from_vec((nodes, cores), vec![1.0; cores])),
            aggregation: Compressed::from(Diagonal::from_vec((cores, nodes), vec![1.0; cores])),
        })
    }
}
