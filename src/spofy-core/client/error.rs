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
