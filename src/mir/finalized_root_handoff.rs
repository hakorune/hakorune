//! Final artifact ownership shared by root families.
//! The pipeline owns this product while the backend view borrows it. Retaining
//! a source product does not issue source meaning or backend admission.

use crate::mir::normal_callable_semantic_package::{
    BirthAbiHandoffV1, FinalizedRootResultAbiV1, FinalizedRootSourceHandoffV1,
};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

/// Final artifact handoff for one exact callable or Script Array root.
/// It is an opaque retention of source products, never a source or ABI issuer.
#[derive(Debug)]
pub(crate) enum FinalizedRootHandoffV1 {
    ScriptArray {
        root_key: String,
        array: crate::mir::builder::FinalizedScriptArrayV1,
    },
    NoBirth {
        root_key: String,
        root_source: Option<FinalizedRootSourceHandoffV1>,
    },
    Births {
        root_key: String,
        root_source: Option<FinalizedRootSourceHandoffV1>,
        keys: Box<[CanonicalSameModuleCallableKeyV1]>,
        births: Box<[BirthAbiHandoffV1]>,
    },
}

impl FinalizedRootHandoffV1 {
    pub(crate) fn script_array(&self) -> Option<&crate::mir::builder::FinalizedScriptArrayV1> {
        match self {
            Self::ScriptArray { array, .. } => Some(array),
            Self::NoBirth { .. } | Self::Births { .. } => None,
        }
    }

    pub(crate) fn root_key(&self) -> &str {
        match self {
            Self::NoBirth { root_key, .. }
            | Self::Births { root_key, .. }
            | Self::ScriptArray { root_key, .. } => root_key,
        }
    }

    pub(crate) fn root_result(&self) -> Option<FinalizedRootResultAbiV1> {
        self.root_source()
            .map(FinalizedRootSourceHandoffV1::result_abi)
    }

    pub(crate) fn root_source(&self) -> Option<&FinalizedRootSourceHandoffV1> {
        match self {
            Self::ScriptArray { .. } => None,
            Self::NoBirth { root_source, .. } | Self::Births { root_source, .. } => {
                root_source.as_ref()
            }
        }
    }

    pub(crate) fn births(&self) -> Option<&[BirthAbiHandoffV1]> {
        match self {
            Self::ScriptArray { .. } => None,
            Self::NoBirth { .. } => Some(&[]),
            Self::Births { births, .. } => Some(births),
        }
    }

    pub(crate) fn birth_keys(&self) -> Option<&[CanonicalSameModuleCallableKeyV1]> {
        match self {
            Self::ScriptArray { .. } => None,
            Self::NoBirth { .. } => Some(&[]),
            Self::Births { keys, .. } => Some(keys),
        }
    }
}
