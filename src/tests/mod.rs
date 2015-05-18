use std::fs;
use std::path::PathBuf;

use assert;
use Analysis;

mod fixture;

#[test]
fn new() {
    let analysis = setup("002");
    let system = analysis.system;

    assert_eq!(system.cores, 2);
    assert_eq!(system.nodes, 4 * 2 + 12);
    assert::absolute_within(&system.U, &fixture::U, 1e-9);
    assert::within(&system.L, &fixture::L, 1e-10);
    assert::within(&system.D, &fixture::D, 1e-13);
    assert::within(&system.E, &fixture::E, 1e-13);
    assert::within(&system.F, &fixture::F, 1e-13);
}

fn setup(name: &str) -> Analysis {
    use std::default::Default;
    use super::model::hotspot::new;

    let circuit = new(&find_fixture(&format!("{}.flp", name)),
                      &find_fixture("hotspot.config"), "").unwrap();

    Analysis::new(circuit, Default::default()).unwrap()
}

fn find_fixture(name: &str) -> PathBuf {
    let path = PathBuf::from("tests").join("fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
