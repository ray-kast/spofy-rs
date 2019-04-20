use hyper::Uri;
use std::fmt::Display;
use url::form_urlencoded;

// TODO: the scopes should really be a constant provided by spofy-core itself.

pub fn authcode_uri<
  I: AsRef<str>,
  R: AsRef<str>,
  S: AsRef<str>,
  C: Into<Option<CI>>,
  CI: IntoIterator,
>(
  id: I,
  redirect: R,
  state: S,
  scopes: C,
  dialog: bool,
) -> Uri
where
  CI::Item: Display,
{
  let mut serializer = form_urlencoded::Serializer::new(String::new());

  serializer.extend_pairs(&[
    ("client_id", id.as_ref()),
    ("response_type", "code"),
    ("redirect_uri", redirect.as_ref()),
    ("state", state.as_ref()),
  ]);

  if let Some(scopes) = scopes.into() {
    serializer.append_pair("scope", &itertools::join(scopes.into_iter(), " "));
  }

  if dialog {
    serializer.append_pair("show_dialog", "true");
  }

  let query = serializer.finish();

  format!("https://accounts.spotify.com/authorize?{}", query)
    .parse()
    .unwrap()
}
