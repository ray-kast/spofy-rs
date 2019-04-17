use clap::{App, ArgMatches};

pub fn app<'a, 'b>() -> App<'a, 'b> {
  let app = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"));

  app
}

pub fn parse<'a>() -> ArgMatches<'a> { app().get_matches() }
