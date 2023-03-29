// Copyright 2023 myujiku (https://github.com/myuujiku)

use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

// Searches the file tree until a directory containing a 'gun.toml' is found.
pub fn get() -> io::Result<PathBuf> {
    let path = Path::new(".").canonicalize()?;
    let mut current: &Path = &path;

    loop {
        if current.join("gun.toml").is_file() {
            return Ok(current.to_path_buf());
        }

        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            return Err(Error::new(ErrorKind::Other, "Not a configun directory."));
        }
    }
}
