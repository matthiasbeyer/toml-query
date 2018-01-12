# Changelog

This changelog was started with the 0.4.0 release, so there are no logs before
that version.

# Next

* `TomlValueReadTypeExt` requires now `TomlValueReadExt`.

# 0.5.0

* Minimum required rust compiler version is now 1.20.0
* Dependencies were updated
* Added method for requesting a type directly from the TOML document:
  The method returns the requested type directly, or fails with
  `Err(_)` and appropriate message:
  `document.read_string(path) -> Result<String, Error>` (for example)

# 0.4.0

* Updated the `error-chain` dependency from `0.10` to `0.11`.

