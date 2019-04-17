use failure::Error;
use futures::prelude::*;
use hyper::{
  client::HttpConnector,
  http::{Response, Uri},
  Body, Client as HyperClient,
};
use hyper_tls::HttpsConnector;
use std::sync::Arc;

// TODO: make a module-level error type

fn build_uri<P: AsRef<str>>(path: P) -> Result<Uri, Error> {
  Ok({
    let x = Uri::builder()
      .scheme("https")
      .authority("api.spotify.com")
      .path_and_query(&*format!("/v1/{}", path.as_ref()))
      .build()?;

    println!("x is {:?}", x);

    x
  })
}

pub struct Client {
  client: Arc<HyperClient<HttpsConnector<HttpConnector>, Body>>,
}

impl Client {
  pub fn new() -> Result<Self, Error> {
    let https = HttpsConnector::new(4)?;

    let client = HyperClient::builder().build::<_, Body>(https);

    Ok(Client {
      client: Arc::new(client),
    })
  }

  pub fn get<E: AsRef<str>>(
    &self,
    endpoint: E,
  ) -> impl Future<Item = Response<Body>, Error = Error> + 'static
  {
    let client = Arc::clone(&self.client);

    build_uri(endpoint)
      .into_future()
      .and_then(move |u| client.get(u).from_err())
  }
}
