extern crate threed_ice;

use std::path::Path;

use {Circuit, Result};

/// The 3D-ICE model.
pub struct ThreeDICE;

impl ThreeDICE {
    /// Construct a thermal circuit.
    pub fn new<T: AsRef<Path>>(config: T) -> Result<Circuit> {
        let backend = ok!(threed_ice::System::new(config));
        let distribution = ok!(backend.distribution());
        Ok(Circuit {
            cores: distribution.columns,
            nodes: distribution.rows,
            capacitance: ok!(backend.capacitance()),
            conductance: ok!(backend.conductance()),
            distribution: distribution,
        })
    }
}
