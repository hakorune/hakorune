//! Final artifact ownership shared by root families.
//! The pipeline owns this product while the backend view borrows it. Retaining
//! a source product does not issue source meaning or backend admission.

use crate::mir::normal_callable_semantic_package::{
    BirthAbiHandoffV1, FinalizedRootResultAbiV1, FinalizedRootSourceHandoffV1,
};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

/// Final artifact handoff for one exact root and its already-issued Birth keys.
/// It is an opaque retention of source products, never a source or ABI issuer.
#[derive(Debug)]
pub(crate) enum FinalizedRootHandoffV1 {
    NoBirth {
        root_key: String,
        root_source: Option<FinalizedRootSourceHandoffV1>,
        root_result: Option<FinalizedRootResultAbiV1>,
    },
    Births {
        root_key: String,
        root_source: Option<FinalizedRootSourceHandoffV1>,
        root_result: Option<FinalizedRootResultAbiV1>,
        keys: Box<[CanonicalSameModuleCallableKeyV1]>,
        births: Box<[BirthAbiHandoffV1]>,
    },
}

impl FinalizedRootHandoffV1 {
    pub(crate) fn root_key(&self) -> &str {
        match self {
            Self::NoBirth { root_key, .. } | Self::Births { root_key, .. } => root_key,
        }
    }

    pub(crate) fn root_result(&self) -> Option<FinalizedRootResultAbiV1> {
        match self {
            Self::NoBirth { root_result, .. } | Self::Births { root_result, .. } => *root_result,
        }
    }

    pub(crate) fn root_source(&self) -> Option<&FinalizedRootSourceHandoffV1> {
        match self {
            Self::NoBirth { root_source, .. } | Self::Births { root_source, .. } => {
                root_source.as_ref()
            }
        }
    }

    pub(crate) fn births(&self) -> &[BirthAbiHandoffV1] {
        match self {
            Self::NoBirth { .. } => &[],
            Self::Births { births, .. } => births,
        }
    }

    pub(crate) fn birth_keys(&self) -> &[CanonicalSameModuleCallableKeyV1] {
        match self {
            Self::NoBirth { .. } => &[],
            Self::Births { keys, .. } => keys,
        }
    }
}
