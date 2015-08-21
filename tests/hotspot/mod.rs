#![allow(non_snake_case)]

use assert;
use std::path::PathBuf;
use temperature::circuit::HotSpot;
use temperature::{Config, Simulator};

mod fixture;

const UNITS: usize = 2;

#[test]
fn next_0() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; UNITS];
    for i in 0..220 {
        let range = (i * UNITS)..((i + 1) * UNITS);
        simulator.next(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 0.1);
    }
    simulator.next(&[], &mut Q[..0]);
    simulator.next(&[], &mut Q[..0]);
    for i in 220..440 {
        let range = (i * UNITS)..((i + 1) * UNITS);
        simulator.next(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 0.1);
    }
}

#[test]
fn next_1() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; UNITS];
    for i in 0..440 {
        let range = (i * UNITS)..((i + 1) * UNITS);
        simulator.next(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 0.1);
    }
}

#[test]
fn next_2() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; 2 * UNITS];
    for i in 0..220 {
        let range = (2 * i * UNITS)..(2 * (i + 1) * UNITS);
        simulator.next(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 0.1);
    }
}

#[test]
fn next_440() {
    let mut simulator = setup("002");
    let mut Q = vec![0.0; 440 * UNITS];
    simulator.next(&fixture::P, &mut Q);
    assert::close(&Q, &fixture::Q[..], 0.1);
}

fn setup(name: &str) -> Simulator {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config")).unwrap();
    Simulator::new(&circuit, &Config::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/hotspot/fixtures").join(name);
    assert!(::std::fs::metadata(&path).is_ok());
    path
}
