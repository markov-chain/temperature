use assert;
use std::default::Default;
use std::fs;
use std::path::PathBuf;

use temperature::Analysis;
use temperature::model::HotSpot;

mod fixture;

#[test]
#[allow(non_snake_case)]
fn compute_transient() {
    let analysis = setup("002");

    let nc = 2;
    let nn = 4 * nc + 12;
    let ns = fixture::P.len() / nc;

    let mut Q = vec![0.0; nc * ns];
    let mut S = vec![0.0; nn * ns];

    analysis.compute_transient(&fixture::P, &mut Q, &mut S, ns);

    assert::close(&Q, &fixture::Q[..], 1e-12);
}

pub fn setup(name: &str) -> Analysis {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Analysis::new(&circuit.unwrap(), &Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/model/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
