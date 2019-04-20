/********************************************************************
 *
 * spofy-cli - a command-line interface for Spofy (cli.rs)
 * Copyright (C) 2019-2019 Ryan Schroeder
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *******************************************************************/

use clap::{App, ArgMatches};

pub fn app<'a, 'b>() -> App<'a, 'b> {
  let app = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"));

  app
}

pub fn parse<'a>() -> ArgMatches<'a> { app().get_matches() }
