#[phase(plugin)] extern crate assert;

mod fixture;

mod analysis {
    use super::super::Analysis;
    use super::find_fixture;
    use super::fixture;

    #[test]
    fn new() {
        let analysis = Analysis::load(find_fixture("002.json")).unwrap();
        let system = analysis.system;

        assert_eq!(system.cores, 2);
        assert_eq!(system.nodes, 4 * 2 + 12);
        assert_close!(system.D, fixture::D);
        assert_abs_close!(system.U, fixture::U);
        assert_close!(system.L, fixture::L);
        assert_close!(system.E, fixture::E);
        assert_close!(system.F, fixture::F);
    }

    #[test]
    #[allow(non_snake_case)]
    fn compute_transient() {
        let analysis = Analysis::load(find_fixture("002.json")).unwrap();

        let nc = 2;
        let nn = 4 * nc + 12;
        let ns = fixture::P.len() / nc;

        let mut Q = Vec::from_elem(nc * ns, 0.0);
        let mut S = Vec::from_elem(nn * ns, 0.0);

        analysis.compute_transient(fixture::P, Q.as_mut_slice(), S.as_mut_slice(), ns);

        assert_close!(Q, fixture::Q);
    }
}

mod config {
    use super::super::Config;
    use super::find_fixture;

    #[test]
    fn load() {
        let config = Config::load(find_fixture("002.json")).unwrap();

        assert_eq!(config.hotspot.floorplan.as_slice(),
                   find_fixture("002.flp").as_str().unwrap());
        assert_eq!(config.hotspot.config.as_slice(),
                   find_fixture("hotspot.config").as_str().unwrap());
        assert_eq!(config.hotspot.params.as_slice(), "");
        assert_eq!(config.time_step, 1e-3);
        assert_eq!(config.ambience as f32, 318.15);
    }
}

fn find_fixture(name: &str) -> Path {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    path
}
