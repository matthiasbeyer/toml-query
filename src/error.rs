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

        EmptyQueryError {
            description("the query is empty")
            display("The query on the TOML is empty")
        }
    }
}
