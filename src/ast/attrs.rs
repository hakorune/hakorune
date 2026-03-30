#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuneAttr {
    pub name: String,
    pub args: Vec<String>,
}

pub const RUNE_SUPPORTED_NAMES_MSG: &str =
    "Public|Internal|FfiSafe|Symbol|CallConv|ReturnsOwned|FreeWith|Ownership|Hint|Contract|IntrinsicCandidate";

impl RuneAttr {
    pub fn supported_names_msg() -> &'static str {
        RUNE_SUPPORTED_NAMES_MSG
    }

    pub fn supported_name(name: &str) -> bool {
        matches!(
            name,
            "Public"
                | "Internal"
                | "FfiSafe"
                | "Symbol"
                | "CallConv"
                | "ReturnsOwned"
                | "FreeWith"
                | "Ownership"
                | "Hint"
                | "Contract"
                | "IntrinsicCandidate"
        )
    }

    pub fn noarg_name(name: &str) -> bool {
        matches!(name, "Public" | "Internal" | "FfiSafe" | "ReturnsOwned")
    }

    pub fn single_arg_name(name: &str) -> bool {
        matches!(
            name,
            "Symbol"
                | "CallConv"
                | "FreeWith"
                | "Ownership"
                | "Hint"
                | "Contract"
                | "IntrinsicCandidate"
        )
    }

    pub fn value_contract_error(name: &str, args: &[String]) -> Option<String> {
        let arg0 = args.first().map(String::as_str).unwrap_or("");
        match name {
            "CallConv" if arg0 != "c" => Some(
                "[freeze:contract][parser/rune] CallConv(\"c\")".to_string(),
            ),
            "Ownership" if !matches!(arg0, "owned" | "borrowed" | "shared") => Some(
                "[freeze:contract][parser/rune] Ownership(owned|borrowed|shared)".to_string(),
            ),
            "Hint" if !matches!(arg0, "inline" | "noinline" | "hot" | "cold") => Some(
                "[freeze:contract][parser/rune] Hint(inline|noinline|hot|cold)".to_string(),
            ),
            "Contract"
                if !matches!(arg0, "pure" | "readonly" | "no_alloc" | "no_safepoint") =>
            {
                Some(
                    "[freeze:contract][parser/rune] Contract(pure|readonly|no_alloc|no_safepoint)"
                        .to_string(),
                )
            }
            "IntrinsicCandidate" if arg0.is_empty() => Some(
                "[freeze:contract][parser/rune] IntrinsicCandidate(\"symbol\") with non-empty symbol"
                    .to_string(),
            ),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeclarationAttrs {
    pub runes: Vec<RuneAttr>,
}

impl DeclarationAttrs {
    pub fn is_empty(&self) -> bool {
        self.runes.is_empty()
    }
}
