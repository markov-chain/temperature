use std::io::fs::PathExtensions;

pub fn find_fixture(name: &'static str) -> Path {
    let path = Path::new("fixtures").join(name);
    assert!(path.exists());
    path
}
