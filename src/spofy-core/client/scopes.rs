/*************************************************************************
 *
 * spofy-rs - the Rust components of Spofy (scopes.rs)
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

pub mod playlist {
  /// Write access to a user's private playlists
  pub static MODIFY_PRIVATE: &str = "playlist-modify-private";

  /// Write access to a user's public playlists
  pub static MODIFY_PUBLIC: &str = "playlist-modify-public";

  /// Include collaborative playlists when requesting a user's playlists
  pub static READ_COLLABORATIVE: &str = "playlist-read-collaborative";

  /// Read access to a user's private playlists
  pub static READ_PRIVATE: &str = "playlist-read-private";
}

pub mod user {
  /// Write access to a user's following list
  pub static MODIFY_FOLLOW: &str = "user-follow-modify";

  /// Write access to a user's "Your Music" library
  pub static MODIFY_LIBRARY: &str = "user-library-modify";

  /// Write access to a user's player state
  pub static MODIFY_PLAYBACK_STATE: &str = "user-modify-playback-state";

  /// Read access to a user's birthdate
  pub static READ_BIRTHDATE: &str = "user-read-birthdate";

  /// Read access to a user's currently playing track
  pub static READ_CURRENTLY_PLAYING: &str = "user-read-currently-playing";

  /// Read access to a user's email address
  pub static READ_EMAIL: &str = "user-read-email";

  /// Read access to a user's followers and following list
  pub static READ_FOLLOW: &str = "user-follow-read";

  /// Read access to a user's "Your Music" library
  pub static READ_LIBRARY: &str = "user-library-read";

  /// Read access to a user's player state
  pub static READ_PLAYBACK_STATE: &str = "user-read-playback-state";

  /// Read access to a user's subscription details
  pub static READ_PRIVATE: &str = "user-read-private";

  /// Read access to a user's recently played tracks
  pub static READ_RECENTLY_PLAYED: &str = "user-read-recently-played";

  /// Read access to a user's top artists and tracks
  pub static READ_TOP: &str = "user-top-read";
}

/// Remote control playback of Spotify. This scope is currently available to Spotify iOS and Android
/// App Remote SDKs.
pub static APP_REMOTE_CONTROL: &str = "app-remote-control";

/// Control playback of a Spotify track. This scope is currently available to Spotify Playback SDKs,
/// including the iOS SDK, Android SDK, and Web Playback SDK. The user must have a Spotify Premium
/// account.
pub static STREAMING: &str = "streaming";
