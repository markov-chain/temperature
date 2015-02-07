#![feature(core, io, path)]

#[macro_use]
extern crate assert;

extern crate temperature;

mod fixture;
mod support;

#[test]
#[allow(non_snake_case)]
fn compute_transient() {
    use std::iter::repeat;

    let analysis = support::setup("002");

    let nc = 2;
    let nn = 4 * nc + 12;
    let ns = fixture::P.len() / nc;

    let mut Q = repeat(0.0).take(nc * ns).collect::<Vec<_>>();
    let mut S = repeat(0.0).take(nn * ns).collect::<Vec<_>>();

    analysis.compute_transient(&fixture::P[], &mut Q[], &mut S[], ns);

    assert_close!(Q, fixture::Q);
}
