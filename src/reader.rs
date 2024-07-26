use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{config, error::*};

pub fn read(path: PathBuf) -> Result<String> {
    let config = config::Config::new();

    let mut path = Path::new(&config.template_path).join(path);
    path.set_extension("json.jb");

    fs::read_to_string(path).map_err(JsonBuilderError::IO)
}
