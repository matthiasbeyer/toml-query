/// Error types

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        QueryParsingError(query: String) {
            description("parsing the query failed")
            display("Parsing the query '{}' failed", query)
        }
    }
}
