#![allow(non_snake_case)]

use assert;
use matrix::format::{Conventional, Diagonal};
use std::path::PathBuf;
use temperature::circuit::ThreeDICE;
use temperature::{Config, Simulator};

mod fixture;

const UNITS: usize = 4;

#[test]
fn ambience() {
    let mut simulator = setup("004.stk");
    let mut Q = vec![0.0; 42 * UNITS];
    simulator.step(&vec![0.0; 42 * UNITS], &mut Q);
    assert::close(&Q, &vec![318.15; 42 * UNITS], 0.0);
}

#[test]
fn distribution() {
    let circuit = ThreeDICE::new(find("004.stk")).unwrap();
    let distribution = Conventional::from(&circuit.distribution);
    let identity = Conventional::from(Diagonal::from_vec((4 * UNITS, UNITS), vec![1.0; UNITS]));
    assert::close(&*distribution, &*identity, 1e-15);
}

#[test]
fn step_20() {
    let mut simulator = setup("004.stk");
    let mut Q = vec![0.0; 20 * UNITS];
    simulator.step(&fixture::P, &mut Q);
    assert::close(&Q, &fixture::Q[..], 1e-12);
}

fn setup(name: &str) -> Simulator {
    let circuit = ThreeDICE::new(find(name)).unwrap();
    Simulator::new(&circuit, &Config::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/threed_ice/fixtures").join(name);
    assert!(::std::fs::metadata(&path).is_ok());
    path
}
