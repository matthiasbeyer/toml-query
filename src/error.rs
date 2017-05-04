/// Error types

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {

        // Errors for tokenizer

        QueryParsingError(query: String) {
            description("parsing the query failed")
            display("Parsing the query '{}' failed", query)
        }

        EmptyQueryError {
            description("the query is empty")
            display("The query on the TOML is empty")
        }

        EmptyIdentifier {
            description("Query an empty identifier: ''")
            display("The passed query has an empty identifier")
        }

        ArrayAccessWithoutIndex {
            description("trying to access array without index")
            display("The passed query tries to access an array but does not specify the index")
        }

        ArrayAccessWithInvalidIndex {
            description("trying to pass an invalid index")
            display("The passed query tries to access an array but does not specify a valid index")
        }

        // Errors for Resolver

        IdentifierNotFoundInDocument(ident: String) {
            description("Identifier missing in document")
            display("The identfier '{}' is not present in the document", ident)
        }

    }
}
