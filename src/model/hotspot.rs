extern crate hotspot;

use matrix::{Compressed, Dense, Diagonal, Make, Shape};
use std::path::Path;

use {Circuit, Result};

/// The HotSpot model.
pub struct HotSpot;

impl HotSpot {
    /// Construct a thermal circuit.
    pub fn new<F: AsRef<Path>, C: AsRef<Path>>(floorplan: F, config: C) -> Result<Circuit> {
        let hotspot::Circuit { nodes, cores, capacitance, conductance }  = {
            ok!(hotspot::Circuit::new(floorplan, config))
        };
        Ok(Circuit {
            cores: cores,
            nodes: nodes,
            capacitance: Diagonal::make(capacitance, Shape::Square(nodes)),
            conductance: Compressed::from(Dense::make(conductance, Shape::Square(nodes))),
        })
    }
}
