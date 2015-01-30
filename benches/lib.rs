#![feature(core, io, path, rand, test)]

extern crate test;

#[path="../tests/support.rs"]
mod support;

#[bench]
#[allow(non_snake_case)]
fn compute_transient(bench: &mut test::Bencher) {
    use std::iter::repeat;

    let analysis = support::setup("032");

    let nc = 32;
    let ns = 1000;
    let nn = 4 * nc + 12;

    let P = range(0, nc * ns).map(|_| std::rand::random()).collect::<Vec<f64>>();
    let mut Q = repeat(0.0).take(nc * ns).collect::<Vec<f64>>();
    let mut S = repeat(0.0).take(nn * ns).collect::<Vec<f64>>();

    bench.iter(|| {
        analysis.compute_transient(&P[], &mut Q[], &mut S[], ns);
    });
}
