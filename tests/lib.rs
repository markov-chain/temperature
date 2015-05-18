extern crate assert;
extern crate temperature;

mod fixtures;
mod support;

#[test]
#[allow(non_snake_case)]
fn compute_transient() {
    use std::iter::repeat;

    let analysis = support::setup("002");

    let nc = 2;
    let nn = 4 * nc + 12;
    let ns = fixtures::P.len() / nc;

    let mut Q = repeat(0.0).take(nc * ns).collect::<Vec<_>>();
    let mut S = repeat(0.0).take(nn * ns).collect::<Vec<_>>();

    analysis.compute_transient(&fixtures::P, &mut Q, &mut S, ns);

    assert::within(&Q, &fixtures::Q, 1e-12);
}
