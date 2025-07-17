use serde::{Deserialize, Serialize};
use std::path::Path;
#[derive(Serialize, Deserialize, Debug)]
pub struct PostProcessConfig {
    #[serde(default)]
    pub remove: Vec<String>,
    #[serde(default)]
    pub insert: Vec<String>,
    #[serde(default)]
    pub replace: Vec<String>,
    #[serde(default)]
    pub ignore_files: Vec<String>,
}

impl PostProcessConfig {
    pub fn parse_the_config(file: &str) -> PostProcessConfig {
        let pp = Path::new(file);
        let context =
            std::fs::read_to_string(pp).expect(&format!("cannot open the append file: {}", file));
        let pp_config: PostProcessConfig = serde_json::from_str(&context)
            .expect("[Error] json fileparser fail for postprocess config");
        pp_config
    }
}
