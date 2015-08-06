#![allow(non_snake_case)]

use assert;
use matrix::format::{Conventional, Diagonal};
use std::path::PathBuf;
use temperature::circuit::ThreeDICE;
use temperature::{Config, Simulator};

mod fixture;

#[test]
fn step_20() {
    let units = 4;

    let circuit = ThreeDICE::new(find("004.stk")).unwrap();
    let distribution = Conventional::from(&circuit.distribution);
    let identity = Conventional::from(Diagonal::from_vec((16, 4), vec![1.0; units]));
    assert::close(&*distribution, &*identity, 1e-15);

    let mut simulator = Simulator::new(&circuit, &Config::default()).unwrap();
    let mut Q = vec![0.0; 20 * units];
    simulator.step(&fixture::P, &mut Q);
    assert::close(&Q, &fixture::Q[..], 1e-12);
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/threed_ice/fixtures").join(name);
    assert!(::std::fs::metadata(&path).is_ok());
    path
}
