#![allow(non_snake_case)]

use random::{self, Source};
use std::default::Default;
use std::fs;
use std::path::PathBuf;
use test::Bencher;

use temperature::Simulator;
use temperature::circuit::HotSpot;

#[bench] fn next_0001(bench: &mut Bencher) { next(   1, bench); }
#[bench] fn next_0010(bench: &mut Bencher) { next(  10, bench); }
#[bench] fn next_0100(bench: &mut Bencher) { next( 100, bench); }
#[bench] fn next_1000(bench: &mut Bencher) { next(1000, bench); }

fn next(steps: usize, bench: &mut Bencher) {
    let units = 32;

    let mut simulator = setup("032");
    let P = random::default().iter().take(steps * units).collect::<Vec<_>>();
    let mut Q = vec![0.0; steps * units];

    bench.iter(|| simulator.next(&P, &mut Q));
}

fn setup(name: &str) -> Simulator {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Simulator::new(circuit.unwrap(), Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
