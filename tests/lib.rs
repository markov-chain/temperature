#![feature(phase)]

#[phase(plugin)] extern crate assert;

mod fixture;
mod support;

#[test]
#[allow(non_snake_case)]
fn compute_transient() {
    let analysis = support::setup("002");

    let nc = 2;
    let nn = 4 * nc + 12;
    let ns = fixture::P.len() / nc;

    let mut Q = Vec::from_elem(nc * ns, 0.0);
    let mut S = Vec::from_elem(nn * ns, 0.0);

    analysis.compute_transient(fixture::P, Q.as_mut_slice(), S.as_mut_slice(), ns);

    assert_close!(Q, fixture::Q);
}
