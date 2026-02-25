//! Core method alias table (SSOT).

#[derive(Debug, Clone, Copy)]
pub struct CoreMethodAlias {
    pub alias: &'static str,
    pub canonical: &'static str,
}

const CORE_METHOD_ALIASES: &[CoreMethodAlias] = &[
    CoreMethodAlias {
        alias: "toUpperCase",
        canonical: "toUpper",
    },
    CoreMethodAlias {
        alias: "toLowerCase",
        canonical: "toLower",
    },
    CoreMethodAlias {
        alias: "find",
        canonical: "indexOf",
    },
];

pub fn canonical_method_name(method_name: &str) -> &str {
    CORE_METHOD_ALIASES
        .iter()
        .find(|alias| alias.alias == method_name)
        .map(|alias| alias.canonical)
        .unwrap_or(method_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_method_name_aliases() {
        assert_eq!(canonical_method_name("toUpperCase"), "toUpper");
        assert_eq!(canonical_method_name("toLowerCase"), "toLower");
        assert_eq!(canonical_method_name("find"), "indexOf");
        assert_eq!(canonical_method_name("indexOf"), "indexOf");
    }
}
