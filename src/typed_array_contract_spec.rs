//! Source-owned Typed Array element contract vocabulary.
//!
//! This module classifies explicit `Array<T>` annotations only. Runtime
//! activation, state attachment, and mutation checks land in later 3499
//! series steps. `MirType`, homogeneous literals, and storage are not inputs.

pub const UNSUPPORTED_SPELLING_TAG: &str = "[type/typed_array_contract_unsupported_spelling]";
pub const UNSUPPORTED_ELEMENT_TAG: &str = "[type/typed_array_contract_unsupported_element]";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExactArrayElementType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
}

impl ExactArrayElementType {
    pub const fn source_name(self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
        }
    }

    fn parse(source: &str) -> Option<Self> {
        match source {
            "i8" => Some(Self::I8),
            "i16" => Some(Self::I16),
            "i32" => Some(Self::I32),
            "i64" => Some(Self::I64),
            "u8" => Some(Self::U8),
            "u16" => Some(Self::U16),
            "u32" => Some(Self::U32),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayElementContractSpec {
    pub element: ExactArrayElementType,
}

/// Classifies one annotation. `Ok(None)` means it is not a Typed Array source
/// site. Array-like but non-canonical spellings fail instead of falling back.
pub fn parse_annotation(
    declared_type_name: &str,
) -> Result<Option<ArrayElementContractSpec>, String> {
    let source = declared_type_name.trim();
    if source.starts_with("ArrayBox<") || source.ends_with("[]") {
        return Err(format!("{} type={source}", UNSUPPORTED_SPELLING_TAG));
    }
    let Some(rest) = source.strip_prefix("Array<") else {
        return Ok(None);
    };
    let Some(inner) = rest.strip_suffix('>') else {
        return Err(format!("{} type={source}", UNSUPPORTED_SPELLING_TAG));
    };
    let inner = inner.trim();
    if inner.is_empty() || inner.contains(['<', '>', ',']) {
        return Err(format!("{} type={source}", UNSUPPORTED_SPELLING_TAG));
    }
    let Some(element) = ExactArrayElementType::parse(inner) else {
        return Err(format!(
            "{} element={inner} type={source}",
            UNSUPPORTED_ELEMENT_TAG
        ));
    };
    Ok(Some(ArrayElementContractSpec { element }))
}

pub fn reject_constructor_type_arguments(
    class_name: &str,
    has_type_arguments: bool,
) -> Result<(), String> {
    if has_type_arguments && matches!(class_name, "Array" | "ArrayBox") {
        Err(format!(
            "{} spelling=constructor_type_arguments class={class_name}",
            UNSUPPORTED_SPELLING_TAG
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closes_seven_exact_numeric_element_spellings() {
        let accepted = ["i8", "i16", "i32", "i64", "u8", "u16", "u32"];
        for name in accepted {
            let spec = parse_annotation(&format!("Array<{name}>"))
                .unwrap()
                .expect("typed Array spec");
            assert_eq!(spec.element.source_name(), name);
        }
    }

    #[test]
    fn ordinary_annotations_and_unannotated_arrays_are_not_claims() {
        assert_eq!(parse_annotation("i64").unwrap(), None);
        assert_eq!(parse_annotation("ArrayBox").unwrap(), None);
    }

    #[test]
    fn rejects_deferred_elements_and_noncanonical_spellings() {
        for source in [
            "Array<u64>",
            "Array<usize>",
            "Array<String>",
            "Array<Array<u8>>",
            "ArrayBox<u8>",
            "u8[]",
        ] {
            assert!(parse_annotation(source).is_err(), "{source}");
        }
        assert!(reject_constructor_type_arguments("Array", true).is_err());
        assert!(reject_constructor_type_arguments("Point", true).is_ok());
    }
}
