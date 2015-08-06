#![allow(non_snake_case)]

use assert;
use std::path::PathBuf;
use temperature::circuit::HotSpot;
use temperature::{Config, Simulator};

mod fixture;

const UNITS: usize = 2;

#[test]
fn step_0() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; UNITS];
    for i in 0..220 {
        let range = (i * UNITS)..((i + 1) * UNITS);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
    simulator.step(&[], &mut Q[..0]);
    simulator.step(&[], &mut Q[..0]);
    for i in 220..440 {
        let range = (i * UNITS)..((i + 1) * UNITS);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_1() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; UNITS];
    for i in 0..440 {
        let range = (i * UNITS)..((i + 1) * UNITS);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_2() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; 2 * UNITS];
    for i in 0..220 {
        let range = (2 * i * UNITS)..(2 * (i + 1) * UNITS);
        simulator.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_440() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; 440 * UNITS];
    simulator.step(&fixture::P, &mut Q);
    assert::close(&Q, &fixture::Q[..], 1e-12);
}

fn setup(name: &str) -> Simulator {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Simulator::new(&circuit.unwrap(), &Config::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/hotspot/fixtures").join(name);
    assert!(::std::fs::metadata(&path).is_ok());
    path
}
