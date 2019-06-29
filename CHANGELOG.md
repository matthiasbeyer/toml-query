# Changelog

This changelog was started with the 0.4.0 release, so there are no logs before
that version.

# 0.9.1

Bugfix release:

* Fix a panic in the resolver implementation when indexing arrays: Bounds checks
  were missing.
  Tests are also added for this.


# 0.9.0

* Dependencies were updated to toml 0.5.
* Sources were updated to use rust 2018
* A feature was added to read partial documents directly into a structure
  This feature was introduced as normal trait, but later a proc macro was added
  to easily derive it.


# 0.8.0

* Switch error handling to `failure`.

# 0.7.0

* API CHANGE: The `read`, `set`, `delete` and `insert` function took a
  `&String` before for the value path. This changed, they now want a `&str`.
  Existing Code _should_ work, as `String` derefs to `&str`.
* Dependencies "error-chain" and "regex" were updated (thanks Bruce Mitchener)
* Automatic de/serialization was added (See the new
  {read,insert,delete,set}_serialize functions)

# 0.6.0

* `TomlValueReadTypeExt` requires now `TomlValueReadExt`.
* Changed API semantics for the typed read API: The functions return
  `Result<Option<_>>` again, not only `Result<_>`

# 0.5.0

* Minimum required rust compiler version is now 1.20.0
* Dependencies were updated
* Added method for requesting a type directly from the TOML document:
  The method returns the requested type directly, or fails with
  `Err(_)` and appropriate message:
  `document.read_string(path) -> Result<String, Error>` (for example)

# 0.4.0

* Updated the `error-chain` dependency from `0.10` to `0.11`.

