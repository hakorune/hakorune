//! Structural global-call target vocabulary.
//!
//! This module is deliberately independent from source catalogs and wire
//! codecs.  The carrier describes an already-selected global family; it does
//! not prove declaration membership and it never parses or formats a target.

/// Builtin global targets admitted by the canonical call contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalBuiltinGlobalV1 {
    /// The exact builtin print/1 route.
    Print,
}

/// Same-module global target families.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalSameModuleGlobalTargetV1 {
    /// A same-module free function selected by its exact source name/arity.
    FreeFunction { name: Box<str>, arity: u32 },
    /// A same-module static box method selected by its exact owner/method/arity.
    StaticBoxMethod {
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
}

/// Canonical structural global target.
///
/// Consumers should treat this as an opaque, already-typed carrier.  Source
/// declaration membership and the sole production construction authority live
/// outside this crate and are enforced by the owner-specific repository guard.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalGlobalTargetV1 {
    Builtin(CanonicalBuiltinGlobalV1),
    SameModule(CanonicalSameModuleGlobalTargetV1),
}

/// Component-level validation failure for the carrier-only constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalGlobalTargetComponentV1 {
    FreeFunctionName,
    StaticOwner,
    StaticMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalGlobalTargetConstructionErrorV1 {
    EmptyComponent(CanonicalGlobalTargetComponentV1),
}

impl CanonicalGlobalTargetV1 {
    /// A display-only projection for diagnostics and legacy wire adapters.
    /// It never participates in target selection or reconstruction.
    pub fn display_name(&self) -> String {
        match self {
            Self::Builtin(CanonicalBuiltinGlobalV1::Print) => "print".to_owned(),
            Self::SameModule(CanonicalSameModuleGlobalTargetV1::FreeFunction { name, arity }) => {
                format!("{name}/{arity}")
            }
            Self::SameModule(CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
                owner,
                method,
                arity,
            }) => format!("{owner}.{method}/{arity}"),
        }
    }

    /// Borrow the source-facing name for diagnostics/lookup after a target is
    /// already selected.  Callers must not use this projection to issue a new
    /// target or to retry resolution.
    pub fn source_name(&self) -> &str {
        match self {
            Self::Builtin(CanonicalBuiltinGlobalV1::Print) => "print",
            Self::SameModule(CanonicalSameModuleGlobalTargetV1::FreeFunction { name, .. }) => name,
            Self::SameModule(CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
                method, ..
            }) => method,
        }
    }

    pub const fn arity(&self) -> Option<u32> {
        match self {
            Self::Builtin(_) => Some(1),
            Self::SameModule(CanonicalSameModuleGlobalTargetV1::FreeFunction { arity, .. })
            | Self::SameModule(CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
                arity, ..
            }) => Some(*arity),
        }
    }

    /// Build the static-method carrier after the caller has validated the
    /// exact declaration relation.  This checks only structural components;
    /// catalog/session/brand authority remains with the caller.
    pub fn new_static_box_method(
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
    ) -> Result<Self, CanonicalGlobalTargetConstructionErrorV1> {
        if owner.is_empty() {
            return Err(CanonicalGlobalTargetConstructionErrorV1::EmptyComponent(
                CanonicalGlobalTargetComponentV1::StaticOwner,
            ));
        }
        if method.is_empty() {
            return Err(CanonicalGlobalTargetConstructionErrorV1::EmptyComponent(
                CanonicalGlobalTargetComponentV1::StaticMethod,
            ));
        }
        Ok(Self::SameModule(
            CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
                owner,
                method,
                arity,
            },
        ))
    }

    /// Build the free-function structural shape for owner-local tests and a
    /// later dedicated issuer row.  No production caller is opened in B1-S0.
    pub fn new_free_function(
        name: Box<str>,
        arity: u32,
    ) -> Result<Self, CanonicalGlobalTargetConstructionErrorV1> {
        if name.is_empty() {
            return Err(CanonicalGlobalTargetConstructionErrorV1::EmptyComponent(
                CanonicalGlobalTargetComponentV1::FreeFunctionName,
            ));
        }
        Ok(Self::SameModule(
            CanonicalSameModuleGlobalTargetV1::FreeFunction { name, arity },
        ))
    }

    pub const fn builtin_print() -> Self {
        Self::Builtin(CanonicalBuiltinGlobalV1::Print)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CanonicalBuiltinGlobalV1, CanonicalGlobalTargetComponentV1,
        CanonicalGlobalTargetConstructionErrorV1, CanonicalGlobalTargetV1,
        CanonicalSameModuleGlobalTargetV1,
    };

    #[test]
    fn nested_shape_retains_static_method_components() {
        let target =
            CanonicalGlobalTargetV1::new_static_box_method("MathBox".into(), "add".into(), 2)
                .expect("valid static method");
        assert_eq!(
            target,
            CanonicalGlobalTargetV1::SameModule(
                CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
                    owner: "MathBox".into(),
                    method: "add".into(),
                    arity: 2,
                },
            )
        );
    }

    #[test]
    fn nested_builtin_and_free_function_shapes_are_distinct() {
        assert_eq!(
            CanonicalGlobalTargetV1::builtin_print(),
            CanonicalGlobalTargetV1::Builtin(CanonicalBuiltinGlobalV1::Print)
        );
        assert_eq!(
            CanonicalGlobalTargetV1::new_free_function("id".into(), 1)
                .expect("valid free function"),
            CanonicalGlobalTargetV1::SameModule(CanonicalSameModuleGlobalTargetV1::FreeFunction {
                name: "id".into(),
                arity: 1,
            },)
        );
    }

    #[test]
    fn empty_components_reject_without_fabricating_target() {
        assert_eq!(
            CanonicalGlobalTargetV1::new_static_box_method("".into(), "run".into(), 0),
            Err(CanonicalGlobalTargetConstructionErrorV1::EmptyComponent(
                CanonicalGlobalTargetComponentV1::StaticOwner,
            ))
        );
        assert_eq!(
            CanonicalGlobalTargetV1::new_static_box_method("Box".into(), "".into(), 0),
            Err(CanonicalGlobalTargetConstructionErrorV1::EmptyComponent(
                CanonicalGlobalTargetComponentV1::StaticMethod,
            ))
        );
        assert_eq!(
            CanonicalGlobalTargetV1::new_free_function("".into(), 0),
            Err(CanonicalGlobalTargetConstructionErrorV1::EmptyComponent(
                CanonicalGlobalTargetComponentV1::FreeFunctionName,
            ))
        );
    }

    #[test]
    fn equal_inputs_are_deterministic() {
        let left =
            CanonicalGlobalTargetV1::new_static_box_method("Box".into(), "run".into(), u32::MAX)
                .expect("valid target");
        let right =
            CanonicalGlobalTargetV1::new_static_box_method("Box".into(), "run".into(), u32::MAX)
                .expect("valid target");
        assert_eq!(left, right);
    }
}
