use std::result::Result as RResult;
use error::Error;

pub type Result<T> = RResult<T, Error>;
