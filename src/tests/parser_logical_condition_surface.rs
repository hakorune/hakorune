use crate::parser::NyashParser;

fn c197_source() -> &'static str {
    r#"
static box Main {
    main(args) {
        local a = 1
        local b = 2
        local c = 0

        if (
            a == 1
            && b == 2
            || c == 1
        ) {
            c = c + 1
        }

        loop (
            c < 3
            && a == 1
        ) {
            c = c + 1
        }

        return c
    }
}
"#
}

#[test]
fn c197_multiline_logical_conditions_parse_default_route() {
    crate::tests::helpers::env::with_env_var("NYASH_PARSER_TOKEN_CURSOR", "0", || {
        NyashParser::parse_from_string(c197_source()).expect("C197 logical condition parse");
    });
}

#[test]
fn c197_multiline_logical_conditions_parse_token_cursor_route() {
    crate::tests::helpers::env::with_env_var("NYASH_PARSER_TOKEN_CURSOR", "1", || {
        NyashParser::parse_from_string(c197_source()).expect("C197 cursor condition parse");
    });
}
