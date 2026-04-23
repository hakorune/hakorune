use super::StringBox;
use crate::box_trait::{BoolBox, IntegerBox, NyashBox};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringMethodId {
    Length,
    Substring,
    Concat,
    IndexOf,
    IndexOfFrom,
    Replace,
    Trim,
    LastIndexOf,
    LastIndexOfFrom,
    Contains,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringSurfaceEffect {
    Read,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringSurfaceReturn {
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringExposureState {
    pub runtime_impl: bool,
    pub vm_dispatch: bool,
    pub std_sugar: bool,
    pub smoke_pinned: bool,
}

impl StringExposureState {
    pub const STABLE: Self = Self {
        runtime_impl: true,
        vm_dispatch: true,
        std_sugar: true,
        smoke_pinned: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringMethodSpec {
    pub id: StringMethodId,
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
    pub arity: u8,
    pub slot: u16,
    pub effect: StringSurfaceEffect,
    pub returns: StringSurfaceReturn,
    pub exposure: StringExposureState,
}

impl StringMethodSpec {
    pub fn matches_name(&self, name: &str) -> bool {
        self.canonical == name || self.aliases.iter().any(|alias| *alias == name)
    }

    pub fn matches_signature(&self, name: &str, arity: usize) -> bool {
        self.arity as usize == arity && self.matches_name(name)
    }
}

pub const STRING_SURFACE_METHODS: &[StringMethodSpec] = &[
    StringMethodSpec {
        id: StringMethodId::Length,
        canonical: "length",
        aliases: &["len", "size"],
        arity: 0,
        slot: 300,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::Substring,
        canonical: "substring",
        aliases: &["substr"],
        arity: 2,
        slot: 301,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::Concat,
        canonical: "concat",
        aliases: &[],
        arity: 1,
        slot: 302,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::IndexOf,
        canonical: "indexOf",
        aliases: &["find"],
        arity: 1,
        slot: 303,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::IndexOfFrom,
        canonical: "indexOf",
        aliases: &["find"],
        arity: 2,
        slot: 303,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::Replace,
        canonical: "replace",
        aliases: &[],
        arity: 2,
        slot: 304,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::Trim,
        canonical: "trim",
        aliases: &[],
        arity: 0,
        slot: 305,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::LastIndexOf,
        canonical: "lastIndexOf",
        aliases: &[],
        arity: 1,
        slot: 308,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::LastIndexOfFrom,
        canonical: "lastIndexOf",
        aliases: &[],
        arity: 2,
        slot: 308,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
    StringMethodSpec {
        id: StringMethodId::Contains,
        canonical: "contains",
        aliases: &[],
        arity: 1,
        slot: 309,
        effect: StringSurfaceEffect::Read,
        returns: StringSurfaceReturn::Value,
        exposure: StringExposureState::STABLE,
    },
];

impl StringMethodId {
    pub fn spec(self) -> &'static StringMethodSpec {
        STRING_SURFACE_METHODS
            .iter()
            .find(|spec| spec.id == self)
            .expect("StringMethodSpec missing for StringMethodId")
    }

    pub fn canonical_name(self) -> &'static str {
        self.spec().canonical
    }

    pub fn aliases(self) -> &'static [&'static str] {
        self.spec().aliases
    }

    pub fn arity(self) -> usize {
        self.spec().arity as usize
    }

    pub fn slot(self) -> u16 {
        self.spec().slot
    }

    pub fn effect(self) -> StringSurfaceEffect {
        self.spec().effect
    }

    pub fn returns(self) -> StringSurfaceReturn {
        self.spec().returns
    }

    pub fn from_name(name: &str) -> Option<Self> {
        STRING_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_name(name))
            .map(|spec| spec.id)
    }

    pub fn from_name_and_arity(name: &str, arity: usize) -> Option<Self> {
        STRING_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_signature(name, arity))
            .map(|spec| spec.id)
    }

    pub fn from_slot_and_arity(slot: u16, arity: usize) -> Option<Self> {
        STRING_SURFACE_METHODS
            .iter()
            .find(|spec| spec.slot == slot && spec.arity as usize == arity)
            .map(|spec| spec.id)
    }
}

pub enum StringSurfaceInvokeResult {
    Value(Box<dyn NyashBox>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringSurfaceInvokeError {
    pub method: StringMethodId,
    pub expected: usize,
    pub actual: usize,
}

impl fmt::Display for StringSurfaceInvokeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StringBox.{} expects {} args, got {}",
            self.method.canonical_name(),
            self.expected,
            self.actual
        )
    }
}

fn arg_text(value: &dyn NyashBox) -> String {
    value
        .as_str_fast()
        .map(str::to_owned)
        .unwrap_or_else(|| value.to_string_box().value)
}

fn arg_i64(value: &dyn NyashBox) -> i64 {
    value.as_i64_fast().unwrap_or_else(|| {
        value
            .to_string_box()
            .value
            .trim()
            .parse::<i64>()
            .unwrap_or(0)
    })
}

impl StringBox {
    pub fn invoke_surface(
        &self,
        method: StringMethodId,
        args: Vec<Box<dyn NyashBox>>,
    ) -> Result<StringSurfaceInvokeResult, StringSurfaceInvokeError> {
        let expected = method.arity();
        let actual = args.len();
        if actual != expected {
            return Err(StringSurfaceInvokeError {
                method,
                expected,
                actual,
            });
        }

        let mut args = args.into_iter();
        let result = match method {
            StringMethodId::Length => StringSurfaceInvokeResult::Value(self.length()),
            StringMethodId::Substring => {
                let start = arg_i64(
                    args.next()
                        .expect("validated StringBox.substring start")
                        .as_ref(),
                );
                let end = arg_i64(
                    args.next()
                        .expect("validated StringBox.substring end")
                        .as_ref(),
                );
                let mode = crate::boxes::string_ops::index_mode_from_env();
                let substring =
                    crate::boxes::string_ops::substring(&self.value, start, Some(end), mode);
                StringSurfaceInvokeResult::Value(Box::new(StringBox::new(substring)))
            }
            StringMethodId::Concat => {
                let rhs = arg_text(
                    args.next()
                        .expect("validated StringBox.concat rhs")
                        .as_ref(),
                );
                StringSurfaceInvokeResult::Value(Box::new(StringBox::new(format!(
                    "{}{}",
                    self.value, rhs
                ))))
            }
            StringMethodId::IndexOf => {
                let needle = arg_text(
                    args.next()
                        .expect("validated StringBox.indexOf needle")
                        .as_ref(),
                );
                let mode = crate::boxes::string_ops::index_mode_from_env();
                let idx = crate::boxes::string_ops::index_of(&self.value, &needle, None, mode);
                StringSurfaceInvokeResult::Value(Box::new(IntegerBox::new(idx)))
            }
            StringMethodId::IndexOfFrom => {
                let needle = arg_text(
                    args.next()
                        .expect("validated StringBox.indexOf needle")
                        .as_ref(),
                );
                let start = arg_i64(
                    args.next()
                        .expect("validated StringBox.indexOf start")
                        .as_ref(),
                );
                let mode = crate::boxes::string_ops::index_mode_from_env();
                let idx =
                    crate::boxes::string_ops::index_of(&self.value, &needle, Some(start), mode);
                StringSurfaceInvokeResult::Value(Box::new(IntegerBox::new(idx)))
            }
            StringMethodId::Replace => {
                let old = arg_text(
                    args.next()
                        .expect("validated StringBox.replace old")
                        .as_ref(),
                );
                let new = arg_text(
                    args.next()
                        .expect("validated StringBox.replace new")
                        .as_ref(),
                );
                StringSurfaceInvokeResult::Value(Box::new(StringBox::new(
                    self.value.replace(&old, &new),
                )))
            }
            StringMethodId::Trim => StringSurfaceInvokeResult::Value(self.trim()),
            StringMethodId::LastIndexOf => {
                let needle = arg_text(
                    args.next()
                        .expect("validated StringBox.lastIndexOf needle")
                        .as_ref(),
                );
                let mode = crate::boxes::string_ops::index_mode_from_env();
                let idx = crate::boxes::string_ops::last_index_of(&self.value, &needle, mode);
                StringSurfaceInvokeResult::Value(Box::new(IntegerBox::new(idx)))
            }
            StringMethodId::LastIndexOfFrom => {
                let needle = arg_text(
                    args.next()
                        .expect("validated StringBox.lastIndexOf needle")
                        .as_ref(),
                );
                let start = arg_i64(
                    args.next()
                        .expect("validated StringBox.lastIndexOf start")
                        .as_ref(),
                );
                let mode = crate::boxes::string_ops::index_mode_from_env();
                let idx = crate::boxes::string_ops::last_index_of_from(
                    &self.value,
                    &needle,
                    Some(start),
                    mode,
                );
                StringSurfaceInvokeResult::Value(Box::new(IntegerBox::new(idx)))
            }
            StringMethodId::Contains => {
                let needle = arg_text(
                    args.next()
                        .expect("validated StringBox.contains needle")
                        .as_ref(),
                );
                StringSurfaceInvokeResult::Value(Box::new(BoolBox::new(
                    self.value.contains(&needle),
                )))
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_surface_catalog_normalizes_aliases() {
        assert_eq!(
            StringMethodId::from_name("length"),
            Some(StringMethodId::Length)
        );
        assert_eq!(
            StringMethodId::from_name("len"),
            Some(StringMethodId::Length)
        );
        assert_eq!(
            StringMethodId::from_name("size"),
            Some(StringMethodId::Length)
        );
        assert_eq!(
            StringMethodId::from_name_and_arity("find", 2),
            Some(StringMethodId::IndexOfFrom)
        );
        assert_eq!(
            StringMethodId::from_slot_and_arity(303, 1),
            Some(StringMethodId::IndexOf)
        );
        assert_eq!(
            StringMethodId::from_slot_and_arity(303, 2),
            Some(StringMethodId::IndexOfFrom)
        );
        assert_eq!(
            StringMethodId::from_name_and_arity("lastIndexOf", 2),
            Some(StringMethodId::LastIndexOfFrom)
        );
        assert_eq!(
            StringMethodId::from_slot_and_arity(308, 2),
            Some(StringMethodId::LastIndexOfFrom)
        );
    }

    #[test]
    fn invoke_surface_routes_string_aliases_and_values() {
        let text = StringBox::new("banana");

        let length = text
            .invoke_surface(StringMethodId::from_name("size").unwrap(), vec![])
            .unwrap();
        match length {
            StringSurfaceInvokeResult::Value(value) => {
                assert_eq!(value.to_string_box().value, "6");
            }
        }

        let sub = text
            .invoke_surface(
                StringMethodId::Substring,
                vec![
                    Box::new(IntegerBox::new(1)) as Box<dyn NyashBox>,
                    Box::new(IntegerBox::new(4)) as Box<dyn NyashBox>,
                ],
            )
            .unwrap();
        match sub {
            StringSurfaceInvokeResult::Value(value) => {
                assert_eq!(value.to_string_box().value, "ana");
            }
        }

        let found = text
            .invoke_surface(
                StringMethodId::from_name_and_arity("find", 2).unwrap(),
                vec![
                    Box::new(StringBox::new("na")) as Box<dyn NyashBox>,
                    Box::new(IntegerBox::new(3)) as Box<dyn NyashBox>,
                ],
            )
            .unwrap();
        match found {
            StringSurfaceInvokeResult::Value(value) => {
                assert_eq!(value.to_string_box().value, "4");
            }
        }

        let last_from = text
            .invoke_surface(
                StringMethodId::LastIndexOfFrom,
                vec![
                    Box::new(StringBox::new("na")) as Box<dyn NyashBox>,
                    Box::new(IntegerBox::new(3)) as Box<dyn NyashBox>,
                ],
            )
            .unwrap();
        match last_from {
            StringSurfaceInvokeResult::Value(value) => {
                assert_eq!(value.to_string_box().value, "2");
            }
        }

        let contains = text
            .invoke_surface(
                StringMethodId::Contains,
                vec![Box::new(StringBox::new("nan")) as Box<dyn NyashBox>],
            )
            .unwrap();
        match contains {
            StringSurfaceInvokeResult::Value(value) => {
                assert_eq!(value.as_bool_fast(), Some(true));
            }
        }
    }
}
