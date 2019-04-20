/********************************************************************
 *
 * spofy-rs - the Rust components of Spofy (config.rs)
 * Copyright (C) 2019-2019 Ryan Schroeder
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *******************************************************************/

use failure_derive::Fail;
use serde_derive::Deserialize;
use std::{
  borrow::Cow,
  env,
  fs::File,
  io::{self, prelude::*},
  path::{Path, PathBuf},
};

#[derive(Debug, Fail)]
pub enum ConfigError {
  #[fail(display = "failed to read environment variable {:?}: {}", 0, 1)]
  EnvVarError(&'static str, env::VarError),

  #[fail(display = "failed to read config file {:?}: {}", 0, 1)]
  IoError(PathBuf, io::Error),

  #[fail(display = "failed to parse config file {:?}: {}", 0, 1)]
  ParseError(PathBuf, toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct Config {
  pub auth: AuthConfig,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
  pub id: String,
  pub secret: String,
}

pub fn path<'a>() -> Result<Cow<'a, Path>, ConfigError> {
  let name = "SPOFY_CONFIG";

  match env::var(name) {
    Ok(s) => Ok(Cow::Owned(s.into())),
    Err(env::VarError::NotPresent) => Ok(Cow::Borrowed(Path::new("config.toml"))),
    Err(e) => Err(ConfigError::EnvVarError(name, e)),
  }
}

pub fn read() -> Result<Config, ConfigError> {
  let path = path()?;
  let mut file =
    File::open(&path).map_err(|e| ConfigError::IoError(path.clone().into_owned(), e))?;

  let mut str = String::new();

  file
    .read_to_string(&mut str)
    .map_err(|e| ConfigError::IoError(path.clone().into_owned(), e))?;

  toml::from_str(&str).map_err(|e| ConfigError::ParseError(path.clone().into_owned(), e))
}
