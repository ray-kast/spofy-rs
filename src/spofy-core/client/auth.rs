/*************************************************************************
 *
 * spofy-rs - the Rust components of Spofy (auth.rs)
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
