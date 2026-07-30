//! Source-only facts for optional `Main.main/N` materialization.
//!
//! These receipts state only whether a route requests the callable compatibility
//! function and, for an App source, its exact symbol and arity.  They do not
//! own AST, source identity, a Builder, collector state, a runner choice, or a
//! physical `main/0` target.

use super::main_expansion::VerifiedRawRootExpansionV1;
use super::OwnedRawRootProjectionV1;

/// Source request for the optional callable `Main.main/N` compatibility entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CallableMainMaterializationPolicyV1 {
    Omitted,
    Required,
}

impl CallableMainMaterializationPolicyV1 {
    pub(in crate::mir) fn snapshot_from_normal_ingress() -> Self {
        if crate::config::env::builder_build_static_main_entry() {
            Self::Required
        } else {
            Self::Omitted
        }
    }

    pub(in crate::mir) const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }
}

/// Exact source callable fact, deliberately separate from physical root entry
/// targets and every runner's entry-selection policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct CallableMainMaterializationTargetV1 {
    symbol: Box<str>,
    arity: usize,
}

impl CallableMainMaterializationTargetV1 {
    fn new(symbol: &str, arity: usize) -> Self {
        Self {
            symbol: symbol.into(),
            arity,
        }
    }

    pub(in crate::mir) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir) const fn arity(&self) -> usize {
        self.arity
    }
}

/// Normal/default source receipt.  Script intentionally normalizes a required
/// request to Omitted, preserving the selected-normal compatibility contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum NormalEntryMaterializationSourceReceiptV1 {
    Script {
        policy: CallableMainMaterializationPolicyV1,
    },
    App {
        policy: CallableMainMaterializationPolicyV1,
        target: CallableMainMaterializationTargetV1,
    },
}

impl NormalEntryMaterializationSourceReceiptV1 {
    pub(in crate::mir::builder) fn seal(
        expansion: &VerifiedRawRootExpansionV1<'_>,
        requested: CallableMainMaterializationPolicyV1,
    ) -> Self {
        match expansion {
            VerifiedRawRootExpansionV1::Script => Self::Script {
                policy: CallableMainMaterializationPolicyV1::Omitted,
            },
            VerifiedRawRootExpansionV1::App(main) => {
                let callable = main
                    .callable_main_compat()
                    .expect("verified Main expansion must contain callable Main.main");
                Self::App {
                    policy: requested,
                    target: CallableMainMaterializationTargetV1::new(
                        callable.symbol(),
                        callable.arity(),
                    ),
                }
            }
        }
    }

    pub(in crate::mir) const fn policy(&self) -> CallableMainMaterializationPolicyV1 {
        match self {
            Self::Script { policy } | Self::App { policy, .. } => *policy,
        }
    }

    pub(in crate::mir) fn target(&self) -> Option<&CallableMainMaterializationTargetV1> {
        match self {
            Self::Script { .. } => None,
            Self::App { target, .. } => Some(target),
        }
    }
}

/// Raw/reference source receipt.  Raw Script + Required has no receipt: the
/// existing Raw binding owner keeps its established typed rejection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawEntryMaterializationSourceReceiptV1 {
    Script {
        policy: CallableMainMaterializationPolicyV1,
    },
    App {
        policy: CallableMainMaterializationPolicyV1,
        target: CallableMainMaterializationTargetV1,
    },
}

impl RawEntryMaterializationSourceReceiptV1 {
    pub(in crate::mir) fn seal(
        projection: &OwnedRawRootProjectionV1,
        requested: CallableMainMaterializationPolicyV1,
    ) -> Option<Self> {
        match projection {
            OwnedRawRootProjectionV1::Script { .. } if requested.is_required() => None,
            OwnedRawRootProjectionV1::Script { .. } => Some(Self::Script {
                policy: CallableMainMaterializationPolicyV1::Omitted,
            }),
            OwnedRawRootProjectionV1::App { .. } => {
                let callable = projection
                    .callable_main_locator()
                    .expect("verified raw App projection must contain callable Main.main");
                Some(Self::App {
                    policy: requested,
                    target: CallableMainMaterializationTargetV1::new(
                        callable.symbol(),
                        callable.arity(),
                    ),
                })
            }
        }
    }

    pub(in crate::mir) const fn policy(&self) -> CallableMainMaterializationPolicyV1 {
        match self {
            Self::Script { policy } | Self::App { policy, .. } => *policy,
        }
    }

    pub(in crate::mir) fn target(&self) -> Option<&CallableMainMaterializationTargetV1> {
        match self {
            Self::Script { .. } => None,
            Self::App { target, .. } => Some(target),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CallableMainMaterializationPolicyV1, NormalEntryMaterializationSourceReceiptV1,
        RawEntryMaterializationSourceReceiptV1,
    };
    use crate::mir::builder::main_expansion::VerifiedRawRootExpansionV1;
    use crate::mir::builder::{OwnedRawSourceV1, RawSourceOriginV1};
    use crate::parser::NyashParser;

    fn parse(source: &str) -> crate::ast::ASTNode {
        NyashParser::parse_from_string(source).expect("source must parse")
    }

    #[test]
    fn normal_receipt_normalizes_required_script_and_keeps_exact_app_target() {
        let script = parse("42");
        let script_expansion =
            VerifiedRawRootExpansionV1::from_program(&script).expect("Script expansion");
        let script_receipt = NormalEntryMaterializationSourceReceiptV1::seal(
            &script_expansion,
            CallableMainMaterializationPolicyV1::Required,
        );
        assert_eq!(
            script_receipt.policy(),
            CallableMainMaterializationPolicyV1::Omitted
        );
        assert!(script_receipt.target().is_none());

        let app = parse("static box Main { main(p0, p1) { return 0 } }");
        let app_expansion = VerifiedRawRootExpansionV1::from_program(&app).expect("App expansion");
        let app_receipt = NormalEntryMaterializationSourceReceiptV1::seal(
            &app_expansion,
            CallableMainMaterializationPolicyV1::Required,
        );
        assert_eq!(
            app_receipt.policy(),
            CallableMainMaterializationPolicyV1::Required
        );
        let target = app_receipt.target().expect("App callable target");
        assert_eq!(target.symbol(), "Main.main/2");
        assert_eq!(target.arity(), 2);
    }

    #[test]
    fn raw_receipt_preserves_script_required_rejection_and_source_target() {
        let script = OwnedRawSourceV1::bind(parse("42"), RawSourceOriginV1::BareAst)
            .expect("raw Script source");
        assert!(RawEntryMaterializationSourceReceiptV1::seal(
            script.projection(),
            CallableMainMaterializationPolicyV1::Required,
        )
        .is_none());

        let app = OwnedRawSourceV1::bind(
            parse("static box Main { main(p0) { return 0 } }"),
            RawSourceOriginV1::BareAst,
        )
        .expect("raw App source");
        let receipt = RawEntryMaterializationSourceReceiptV1::seal(
            app.projection(),
            CallableMainMaterializationPolicyV1::Required,
        )
        .expect("raw App receipt");
        assert_eq!(
            receipt.policy(),
            CallableMainMaterializationPolicyV1::Required
        );
        let target = receipt.target().expect("raw App callable target");
        assert_eq!(target.symbol(), "Main.main/1");
        assert_eq!(target.arity(), 1);
    }
}
