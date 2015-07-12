//! The [HotSpot][1] model.
//!
//! [1]: http://lava.cs.virginia.edu/HotSpot

extern crate hotspot;

use std::path::Path;

use {Circuit, Error, Result};

#[cfg(test)]
mod tests;

/// A thermal circuit based on HotSpot.
pub struct HotSpot {
    backend: hotspot::Circuit,
}

impl HotSpot {
    /// Construct a thermal circuit.
    pub fn new<F: AsRef<Path>, C: AsRef<Path>>(floorplan: F, config: C) -> Result<HotSpot> {
        Ok(HotSpot {
            backend: match hotspot::Circuit::new(floorplan, config) {
                Ok(backend) => backend,
                Err(error) => return Err(Error(error.to_string())),
            },
        })
    }
}

impl Circuit for HotSpot {
    #[inline]
    fn cores(&self) -> usize {
        self.backend.cores
    }

    #[inline]
    fn nodes(&self) -> usize {
        self.backend.nodes
    }

    #[inline]
    fn capacitance(&self) -> Vec<f64> {
        self.backend.capacitance.clone()
    }

    #[inline]
    fn conductance(&self) -> Vec<f64> {
        self.backend.conductance.clone()
    }
}
