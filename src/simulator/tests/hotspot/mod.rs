use assert;
use std::default::Default;
use std::fs;
use std::path::PathBuf;

use circuit::HotSpot;
use simulator::Simulator;

mod fixture;

#[test]
fn new() {
    let simulator = setup("002");
    let system = simulator.system;

    assert_eq!(system.units, 2);
    assert_eq!(system.nodes, 4 * 2 + 12);
    assert_eq!(system.spots, 2);
    assert::close(&system.E.values, &fixture::E[..], 1e-13);
    assert::close(&system.F.values, &fixture::F[..], 1e-13);
}

pub fn setup(name: &str) -> Simulator {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Simulator::new(circuit.unwrap(), Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
