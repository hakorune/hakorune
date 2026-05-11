pub fn finish_result(result: Result<(String, i32), String>) -> Option<i32> {
    match result {
        Ok((output, exit_code)) => {
            print!("{output}");
            Some(exit_code)
        }
        Err(message) => {
            eprintln!("{message}");
            Some(2)
        }
    }
}

pub fn read_labeled_file(
    diagnostic: &'static str,
    field_name: &'static str,
    path: &str,
) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|err| format!("{diagnostic}: {field_name}={path}: {err}"))
}

pub fn one_line_option_text(value: Option<&str>) -> String {
    value.unwrap_or("").replace(['\r', '\n'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line_option_text_strips_newlines() {
        assert_eq!(
            one_line_option_text(Some("line 1\nline 2\r\nline 3")),
            "line 1 line 2  line 3"
        );
    }

    #[test]
    fn one_line_option_text_formats_missing_value_as_empty() {
        assert_eq!(one_line_option_text(None), "");
    }
}
