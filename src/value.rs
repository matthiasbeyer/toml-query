//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

/// # Value extension
///
/// Extension trait for the toml::Value type
///
use toml::Value;

use crate::delete::TomlValueDeleteExt;
use crate::error::Result;
use crate::insert::TomlValueInsertExt;
use crate::read::TomlValueReadExt;
use crate::set::TomlValueSetExt;

/// Conveniance trait over
///
///  * TomlValueReadExt
///  * TomlValueSetExt
///
/// for ease of use.
///
/// The very same goal can be achieved by importing each trait seperately.
pub trait TomlValueExt<'doc>:
    TomlValueReadExt<'doc> + TomlValueSetExt + TomlValueDeleteExt + TomlValueInsertExt
{
    //
    // READ functionality
    //

    /// See documentation of `TomlValueReadExt`
    #[inline]
    fn read_with_seperator(&'doc self, query: &str, sep: char) -> Result<Option<&'doc Value>> {
        TomlValueReadExt::read_with_seperator(self, query, sep)
    }

    /// See documentation of `TomlValueReadExt`
    #[inline]
    fn read_mut_with_seperator(
        &'doc mut self,
        query: &str,
        sep: char,
    ) -> Result<Option<&'doc mut Value>> {
        TomlValueReadExt::read_mut_with_seperator(self, query, sep)
    }

    /// See documentation of `TomlValueReadExt`
    #[inline]
    fn read(&'doc self, query: &str) -> Result<Option<&'doc Value>> {
        TomlValueReadExt::read_with_seperator(self, query, '.')
    }

    /// See documentation of `TomlValueReadExt`
    #[inline]
    fn read_mut(&'doc mut self, query: &str) -> Result<Option<&'doc mut Value>> {
        TomlValueReadExt::read_mut_with_seperator(self, query, '.')
    }

    //
    // SET functionality
    //

    /// See documentation of `TomlValueSetExt`
    #[inline]
    fn set_with_seperator(
        &mut self,
        query: &str,
        sep: char,
        value: Value,
    ) -> Result<Option<Value>> {
        TomlValueSetExt::set_with_seperator(self, query, sep, value)
    }

    /// See documentation of `TomlValueSetExt`
    #[inline]
    fn set(&mut self, query: &str, value: Value) -> Result<Option<Value>> {
        TomlValueSetExt::set_with_seperator(self, query, '.', value)
    }

    //
    // DELETE functionality
    //

    /// See documentation of `TomlValueDeleteExt`
    #[inline]
    fn delete_with_seperator(&mut self, query: &str, sep: char) -> Result<Option<Value>> {
        TomlValueDeleteExt::delete_with_seperator(self, query, sep)
    }

    /// See documentation of `TomlValueDeleteExt`
    #[inline]
    fn delete(&mut self, query: &str) -> Result<Option<Value>> {
        TomlValueDeleteExt::delete(self, query)
    }

    //
    // INSERT functionality
    //

    /// See documentation of `TomlValueInsertExt`
    #[inline]
    fn insert_with_seperator(
        &mut self,
        query: &str,
        sep: char,
        value: Value,
    ) -> Result<Option<Value>> {
        TomlValueInsertExt::insert_with_seperator(self, query, sep, value)
    }

    /// See documentation of `TomlValueInsertExt`
    #[inline]
    fn insert(&mut self, query: &str, value: Value) -> Result<Option<Value>> {
        TomlValueInsertExt::insert(self, query, value)
    }
}

impl<'doc> TomlValueExt<'doc> for Value {}
