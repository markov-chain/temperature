use std::fs;
use std::path::PathBuf;

use temperature::Analysis;

pub fn setup(name: &str) -> Analysis {
    use std::default::Default;
    use temperature::model::hotspot::new;

    let circuit = new(&find_fixture(&format!("{}.flp", name)),
                      &find_fixture("hotspot.config"), "").unwrap();

    Analysis::new(circuit, Default::default()).unwrap()
}

fn find_fixture(name: &str) -> PathBuf {
    let path = PathBuf::from("tests").join("fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
