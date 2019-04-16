#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

mod interface;

use dotenv::dotenv;
use failure::Error;
use interface::{cli, config};
use std::process::exit;

// TODO: LICENSE YER DAMN CODE

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
  let args = cli::parse();

  println!("{:#?}", args);

  let _ = dotenv();

  let conf = config::read()?;

  println!("{:#?}", conf);

  Ok(())
}
