// Copyright 2023 myujiku (https://github.com/myuujiku)

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Gun {
    name: String,
    profiles: Option<Vec<String>>,
    encryption: Option<GunEncryptionOptions>,
    vars: Option<HashMap<String, String>>,
}

// TODO
#[derive(Debug, Deserialize)]
pub struct GunEncryptionOptions {}
