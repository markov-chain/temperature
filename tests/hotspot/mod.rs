#![allow(non_snake_case)]

use assert;
use std::default::Default;
use std::fs;
use std::path::PathBuf;

use temperature::Analysis;
use temperature::circuit::HotSpot;

mod fixture;

#[test]
fn step_1() {
    let cores = 2;
    let mut analysis = setup("002");
    let mut Q = vec![0.0; cores];
    for i in 0..440 {
        let range = (i * cores)..((i + 1) * cores);
        analysis.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_2() {
    let cores = 2;
    let mut analysis = setup("002");
    let mut Q = vec![0.0; 2 * cores];
    for i in 0..220 {
        let range = (2 * i * cores)..(2 * (i + 1) * cores);
        analysis.step(&fixture::P[range.clone()], &mut Q);
        assert::close(&Q, &fixture::Q[range], 1e-12);
    }
}

#[test]
fn step_440() {
    let cores = 2;
    let mut analysis = setup("002");
    let mut Q = vec![0.0; 440 * cores];
    analysis.step(&fixture::P, &mut Q);
    assert::close(&Q, &fixture::Q[..], 1e-12);
}

pub fn setup(name: &str) -> Analysis {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Analysis::new(&circuit.unwrap(), &Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
