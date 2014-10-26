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

#[test]
#[allow(non_snake_case)]
fn compute_transient() {
    let analysis = load_fixture("002");

    let nc = 2;
    let nn = 4 * nc + 12;
    let ns = fixture::P.len() / nc;

    let mut Q = Vec::from_elem(nc * ns, 0.0);
    let mut S = Vec::from_elem(nn * ns, 0.0);

    analysis.compute_transient(fixture::P, Q.as_mut_slice(), S.as_mut_slice(), ns);

    assert_close!(Q, fixture::Q);
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
