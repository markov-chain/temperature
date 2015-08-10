extern crate threed_ice;

use matrix::operation::Transpose;
use self::threed_ice::{AnalysisType, StackElement, System};
use std::path::Path;

use {Circuit, Result};

/// The 3D-ICE model.
pub struct ThreeDICE;

impl ThreeDICE {
    /// Construct a thermal circuit given a configuration file.
    pub fn new<T: AsRef<Path>>(config: T) -> Result<Circuit> {
        ThreeDICE::from(&ok!(System::new(config)))
    }

    /// Construct a thermal circuit given a system.
    pub fn from(system: &System) -> Result<Circuit> {
        if system.stack.elements.iter().any(|element| element == &StackElement::Channel) {
            raise!("microchannels are not supported");
        }
        if system.analysis.kind() != AnalysisType::Steady {
            raise!("the analysis type should be set to “steady”");
        }
        let distribution = ok!(system.distribution());
        let aggregation = distribution.transpose();
        Ok(Circuit {
            capacitance: ok!(system.capacitance()),
            conductance: ok!(system.conductance()),
            distribution: distribution,
            aggregation: aggregation,
        })
    }
}
