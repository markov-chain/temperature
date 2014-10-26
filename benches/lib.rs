extern crate test;

#[path="../tests/support.rs"]
mod support;

#[bench]
#[allow(non_snake_case)]
fn compute_transient(bench: &mut test::Bencher) {
    let analysis = support::setup("032");

    let nc = 32;
    let ns = 1000;
    let nn = 4 * nc + 12;

    let P = range(0u, nc * ns).map(|_| std::rand::random()).collect::<Vec<_>>();
    let mut Q = Vec::from_elem(nc * ns, 0.0);
    let mut S = Vec::from_elem(nn * ns, 0.0);

    bench.iter(|| {
        analysis.compute_transient(P.as_slice(), Q.as_mut_slice(), S.as_mut_slice(), ns);
    });
}
