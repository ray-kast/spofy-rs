/*************************************************************************
 *
 * spofy-rs - the Rust components of Spofy (error.rs)
 * Copyright (C) 2019-2019 Ryan Schroeder
 *
 * spofy-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * spofy-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with spofy-rs.  If not, see <https://www.gnu.org/licenses/>.
 *
 ************************************************************************/

use self::ClientError::*;
use failure_derive::Fail;
use hyper::http;

#[derive(Fail, Debug)]
pub enum ClientError {
  #[fail(display = "authorization requested with missing OAuth token")]
  NoAuthToken,

  #[fail(display = "{}", 0)]
  HttpError(http::Error),

  #[fail(display = "{}", 0)]
  HyperError(hyper::Error),

  #[fail(display = "failed to create HttpsConnector: {}", 0)]
  HyperTlsError(hyper_tls::Error),
}

impl From<http::Error> for ClientError {
  fn from(err: http::Error) -> Self { HttpError(err) }
}

impl From<hyper::Error> for ClientError {
  fn from(err: hyper::Error) -> Self { HyperError(err) }
}

impl From<hyper_tls::Error> for ClientError {
  fn from(err: hyper_tls::Error) -> Self { HyperTlsError(err) }
}
