//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use toml::Value;

pub fn name_of_val(val: &Value) -> &'static str {
    match *val {
        Value::Array(_) => "Array",
        Value::Boolean(_) => "Boolean",
        Value::Datetime(_) => "Datetime",
        Value::Float(_) => "Float",
        Value::Integer(_) => "Integer",
        Value::String(_) => "String",
        Value::Table(_) => "Table",
    }
}
