extern crate threed_ice;

use matrix::operation::Transpose;
use std::path::Path;

use {Circuit, Result};

/// The 3D-ICE model.
pub struct ThreeDICE;

impl ThreeDICE {
    /// Construct a thermal circuit.
    pub fn new<T: AsRef<Path>>(config: T) -> Result<Circuit> {
        let backend = ok!(threed_ice::System::new(config));
        let distribution = ok!(backend.distribution());
        let aggregation = distribution.transpose();
        Ok(Circuit {
            capacitance: ok!(backend.capacitance()),
            conductance: ok!(backend.conductance()),
            distribution: distribution,
            aggregation: aggregation,
        })
    }
}
