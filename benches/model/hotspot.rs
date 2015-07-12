use random::{self, Source};
use std::default::Default;
use std::fs;
use std::path::PathBuf;
use test::Bencher;

use temperature::Analysis;
use temperature::model::HotSpot;

#[bench]
#[allow(non_snake_case)]
fn compute_transient(bench: &mut Bencher) {
    let analysis = setup("032");

    let nc = 32;
    let ns = 1000;
    let nn = 4 * nc + 12;

    let P = random::default().iter().take(nc * ns).collect::<Vec<_>>();
    let mut Q = vec![0.0; nc * ns];
    let mut S = vec![0.0; nn * ns];

    bench.iter(|| analysis.compute_transient(&P, &mut Q, &mut S, ns));
}

pub fn setup(name: &str) -> Analysis {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Analysis::new(circuit.unwrap(), Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/model/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
