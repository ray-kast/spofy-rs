/********************************************************************
 *
 * spofy-cli - a command-line interface for Spofy (main.rs)
 * Copyright (C) 2019-2019 Ryan Schroeder
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *******************************************************************/

mod interface;

use dotenv::dotenv;
use failure::Error;
use futures::{future, prelude::*};
use directories::BaseDirs;
use interface::{cli, config};
use spofy_core::client::Client;
use std::{
  io::{self, prelude::*},
  process::exit,
};

fn main() {
  match run() {
    Ok(()) => {},
    Err(e) => {
      println!("ERROR: {}", e);
      exit(-1); // TODO: exit with a better exit code
    },
  }
}

fn run() -> Result<(), Error> {
  let _args = cli::parse(); // NB: do this first because it might exit early

  let _base_dirs = match BaseDirs::new() {
    Some(d) => d,
    None => panic!(), // TODO: deal with the fact that NoneError can't be converted to failure::Error
  };

  match dotenv() {
    Ok(_) => {},
    Err(ref e) if e.not_found() => {},
    Err(e) => writeln!(io::stderr(), "dotenv failed: {}", e).unwrap(),
  }

  let conf = config::read()?;
  let client = Client::new(format!("spofy-cli v{}", env!("CARGO_PKG_VERSION")))?;

  println!(
    "{:?}",
    spofy_core::client::auth::authcode_uri(
      &conf.auth.id,
      "http://rk1024.net/oauth",
      "frick",
      {
        use spofy_core::client::scopes::*;

        &[
          playlist::MODIFY_PRIVATE,
          playlist::MODIFY_PUBLIC,
          playlist::READ_PRIVATE,
          user::MODIFY_LIBRARY,
          user::READ_CURRENTLY_PLAYING,
          user::READ_LIBRARY,
          user::READ_PLAYBACK_STATE,
        ]
      },
      false
    )
  );

  tokio::run(future::lazy(move || {
    client
      .request(true, "GET", "", None)
      .and_then(|res| {
        println!("Response status: {}", res.status());

        res
          .into_body()
          .for_each(|chunk| {
            io::stdout()
              .write_all(&chunk)
              .and_then(|_| io::stdout().flush())
              .map_err(|e| panic!("{}", e))
          })
          .from_err()
      })
      .map_err(|e| println!("ERROR: {}", e))
  }));

  Ok(())
}
