extern crate test;

extern crate temper;

use temper::Analysis;

#[bench]
#[allow(non_snake_case)]
fn compute_transient(bench: &mut test::Bencher) {
    let analysis = load_fixture("032");

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

fn load_fixture(name: &str) -> Analysis {
    use std::default::Default;
    use temper::model::hotspot::new;

    let circuit = new(&find_fixture(format!("{}.flp", name).as_slice()),
                      &find_fixture("hotspot.config"), "").unwrap();

    Analysis::new(circuit, Default::default()).unwrap()
}

fn find_fixture(name: &str) -> Path {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    path
}
