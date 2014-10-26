#[phase(plugin)] extern crate assert;

use super::Analysis;

mod fixture;

#[test]
fn new() {
    let analysis = load_fixture("002");
    let system = analysis.system;

    assert_eq!(system.cores, 2);
    assert_eq!(system.nodes, 4 * 2 + 12);
    assert_abs_close!(system.U, fixture::U);
    assert_close!(system.L, fixture::L);
    assert_close!(system.D, fixture::D);
    assert_close!(system.E, fixture::E);
    assert_close!(system.F, fixture::F);
}

fn load_fixture(name: &str) -> Analysis {
    use std::default::Default;
    use super::model::hotspot::new;

    let circuit = new(&find_fixture(format!("{}.flp", name).as_slice()),
                      &find_fixture("hotspot.config"), "").unwrap();

    Analysis::new(circuit, Default::default()).unwrap()
}

fn find_fixture(name: &str) -> Path {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    path
}
