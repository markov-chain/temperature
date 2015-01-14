extern crate temperature;

use self::temperature::Analysis;

pub fn setup(name: &str) -> Analysis {
    use std::default::Default;
    use self::temperature::model::hotspot::new;

    let circuit = new(&find_fixture(format!("{}.flp", name).as_slice()),
                      &find_fixture("hotspot.config"), "").unwrap();

    Analysis::new(circuit, Default::default()).unwrap()
}

fn find_fixture(name: &str) -> Path {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(&["fixtures", name]);
    assert!(path.exists());
    path
}
