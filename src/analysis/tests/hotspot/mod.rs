use assert;
use std::default::Default;
use std::fs;
use std::path::PathBuf;

use analysis::Analysis;
use circuit::HotSpot;

mod fixture;

#[test]
fn new() {
    let analysis = setup("002");
    let system = analysis.system;

    assert_eq!(system.cores, 2);
    assert_eq!(system.nodes, 4 * 2 + 12);
    assert::close(&system.D, &fixture::D[..], 1e-13);
    assert::close(&system.E, &fixture::E[..], 1e-13);
    assert::close(&system.F, &fixture::F[..], 1e-13);
}

pub fn setup(name: &str) -> Analysis {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Analysis::new(&circuit.unwrap(), &Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
