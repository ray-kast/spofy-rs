/*************************************************************************
 *
 * spofy-core - the core backend library for Spofy (mod.rs)
 * Copyright (C) 2019-2019 Ryan Schroeder
 *
 * spofy-core is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * spofy-core is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with spofy-core.  If not, see <https://www.gnu.org/licenses/>.
 *
 ************************************************************************/

pub mod auth;
mod error;
pub mod scopes;

use futures::prelude::*;
use hyper::{
  client::HttpConnector,
  http::{self, header, request, HttpTryFrom, Uri},
  Body,
};
use hyper_tls::HttpsConnector;
use std::{
  fmt::Display,
  sync::{Arc, Mutex},
};

pub use error::*;

pub type Request = http::Request<Body>;
pub type Response = http::Response<Body>;

pub type HyperClient = hyper::Client<HttpsConnector<HttpConnector>>;
pub type RcHyperClient = Arc<HyperClient>;

struct ClientCore {
  user_agent: String,
  auth_token: Mutex<Option<String>>,
  client: RcHyperClient,
}

type RcClientCore = Arc<ClientCore>;

pub struct Client(RcClientCore);

impl ClientCore {
  fn new<U: Display>(user_agent: U) -> Result<Self, ClientError> {
    let https = HttpsConnector::new(4)?;

    let client = hyper::Client::builder().build::<_, Body>(https);

    Ok(ClientCore {
      user_agent: format!("{} (spofy-core v{})", user_agent, env!("CARGO_PKG_VERSION")),
      auth_token: Mutex::new(None),
      client: Arc::new(client),
    })
  }

  fn put_token(&self, tok: String) { *self.auth_token.lock().unwrap() = Some(tok) }

  fn create_request(&self, authorize: bool) -> Result<request::Builder, ClientError> {
    let mut req = http::Request::builder();

    req.header(header::USER_AGENT, &*self.user_agent);

    if authorize {
      match &*self.auth_token.lock().unwrap() {
        Some(ref s) => {
          req.header(header::AUTHORIZATION, format!("bearer {}", s));
        },
        None => return Err(ClientError::NoAuthToken),
      }
    }

    Ok(req)
  }

  fn create_basic_request<M, P: AsRef<str>, B: Into<Option<Body>>>(
    &self,
    authorize: bool,
    method: M,
    path: P,
    body: B,
  ) -> Result<Request, ClientError>
  where
    http::Method: HttpTryFrom<M>,
  {
    Ok(
      self
        .create_request(authorize)?
        .method(method)
        .uri(build_uri(path)?)
        .body(match body.into() {
          Some(b) => b,
          None => Body::empty(),
        })?,
    )
  }

  fn request(&self, req: Request) -> impl Future<Item = Response, Error = ClientError> {
    println!("{:#?}", req);

    // TODO: handle ratelimiting

    self.client.request(req).from_err()
  }
}

impl Client {
  pub fn new<U: Display>(user_agent: U) -> Result<Self, ClientError> {
    Ok(Client(Arc::new(ClientCore::new(user_agent)?)))
  }

  pub fn put_token(&self, tok: String) { self.0.put_token(tok) }

  pub fn request<M, P: AsRef<str>, B: Into<Option<Body>>>(
    &self,
    authorize: bool,
    method: M,
    path: P,
    body: B,
  ) -> impl Future<Item = Response, Error = ClientError>
  where
    http::Method: HttpTryFrom<M>,
  {
    let core = Arc::clone(&self.0);

    self
      .0
      .create_basic_request(authorize, method, path, body)
      .into_future()
      .and_then(move |req| core.request(req))
      .map(|res| {
        println!("{:#?}", res);
        res
      })
  }
}

fn build_uri<P: AsRef<str>>(path: P) -> Result<Uri, ClientError> {
  Ok(
    Uri::builder()
      .scheme("https")
      .authority("api.spotify.com")
      .path_and_query(&*format!("/v1/{}", path.as_ref()))
      .build()?,
  )
}
