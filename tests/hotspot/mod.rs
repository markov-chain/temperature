#![allow(non_snake_case)]

use assert;
use std::default::Default;
use std::fs;
use std::path::PathBuf;

use temperature::Simulator;
use temperature::circuit::HotSpot;

mod fixture;

#[test]
fn step_0() {
    let units = 2;
    let mut simulator = setup("002");
    let mut Q = vec![0.0; units];
    for i in 0..220 {
        let range = (i * units)..((i + 1) * units);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
    simulator.step(&[], &mut Q[..0]);
    simulator.step(&[], &mut Q[..0]);
    for i in 220..440 {
        let range = (i * units)..((i + 1) * units);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_1() {
    let units = 2;
    let mut simulator = setup("002");
    let mut Q = vec![0.0; units];
    for i in 0..440 {
        let range = (i * units)..((i + 1) * units);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_2() {
    let units = 2;
    let mut simulator = setup("002");
    let mut Q = vec![0.0; 2 * units];
    for i in 0..220 {
        let range = (2 * i * units)..(2 * (i + 1) * units);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_440() {
    let units = 2;
    let mut simulator = setup("002");
    let mut Q = vec![0.0; 440 * units];
    simulator.step(&fixture::P, &mut Q);
    assert::close(&Q, &fixture::Q[..], 1e-12);
}

pub fn setup(name: &str) -> Simulator {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Simulator::new(&circuit.unwrap(), &Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
