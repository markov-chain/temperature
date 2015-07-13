extern crate threed_ice;

use std::path::Path;

use {Circuit, Result};

/// The 3D-ICE model.
pub struct ThreeDICE;

impl ThreeDICE {
    /// Construct a thermal circuit.
    pub fn new<T: AsRef<Path>>(config: T) -> Result<Circuit> {
        let backend = ok!(threed_ice::System::new(config));
        Ok(Circuit {
            cores: {
                let mut count = 0;
                for element in &backend.stack.elements {
                    if let &threed_ice::StackElement::Die(ref die) = element {
                        count += die.floorplan.elements.len();
                    }
                }
                count
            },
            nodes: {
                let dimensions = &backend.stack.dimensions;
                dimensions.layers() * dimensions.rows() * dimensions.columns()
            },
            capacitance: ok!(backend.capacitance()),
            conductance: ok!(backend.conductance()),
        })
    }
}
