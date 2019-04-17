#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

mod interface;

use dotenv::dotenv;
use failure::Error;
use futures::prelude::*;
use interface::{cli, config};
use spofy_core::client::Client;
use std::{
  io::{self, prelude::*},
  process::exit,
};

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

  let client = Client::new()?;

  tokio::run(
    client
      .get("/")
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
      .map_err(|e| println!("ERROR: {}", e)),
  );

  Ok(())
}
