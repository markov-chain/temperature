/// A configuration of the HotSpot model.
#[deriving(Decodable)]
pub struct HotSpot {
    /// The floorplan file of the platform to analyze.
    pub floorplan: String,
    /// A configuration file of HotSpot (`hotspot.config`).
    pub config: String,
    /// A line of parameters overwriting the parameters in the above file.
    pub params: String,
}

/// A configuration of the analysis.
#[deriving(Decodable)]
pub struct Config {
    /// The configuration of the HotSpot model.
    pub hotspot: HotSpot,
    /// The sampling interval in seconds. It is the time between two successive
    /// samples of power or temperature in power or temperature profiles,
    /// respectively. In the formulas given in the general description of the
    /// library, it is referred to as `Δt`.
    pub time_step: f64,
    /// The temperature of the ambience in Kelvin.
    pub ambience: f64,
}

impl Config {
    /// Reads a configuration from the given file.
    pub fn load(path: Path) -> Result<Config, String> {
        use serialize::json;
        use std::io::File;

        let content = match File::open(&path).read_to_string() {
            Ok(content) => content,
            Err(error) => return Err(error.to_string()),
        };
        match json::decode(content.as_slice()) {
            Ok(config) => Ok(config),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use super::super::test::find_fixture;

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
