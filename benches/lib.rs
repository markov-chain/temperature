extern crate test;

extern crate temper;

#[bench]
#[allow(non_snake_case)]
fn compute_transient(bench: &mut test::Bencher) {
    use temper::Analysis;

    let analysis = Analysis::load(find_fixture("032.json")).unwrap();

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

fn find_fixture(name: &str) -> Path {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    path
}
