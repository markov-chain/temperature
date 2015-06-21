extern crate assert;
extern crate temperature;

mod fixtures;
mod support;

#[test]
#[allow(non_snake_case)]
fn compute_transient() {
    let analysis = support::setup("002");

    let nc = 2;
    let nn = 4 * nc + 12;
    let ns = fixtures::P.len() / nc;

    let mut Q = vec![0.0; nc * ns];
    let mut S = vec![0.0; nn * ns];

    analysis.compute_transient(&fixtures::P, &mut Q, &mut S, ns);

    assert::close(&Q, &fixtures::Q[..], 1e-12);
}
