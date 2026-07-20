use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileValidationErrorV1 {
    Json {
        detail: String,
    },
    WrongSchema {
        actual: String,
    },
    EmptyProfiles,
    EmptyField {
        profile_id: String,
        field: &'static str,
    },
    DuplicateProfileId {
        profile_id: String,
    },
    DuplicateFeature {
        profile_id: String,
        feature: String,
    },
    TestCompileModeMismatch {
        profile_id: String,
    },
    UnsealedAmbientRustflags {
        profile_id: String,
    },
}

impl fmt::Display for ProfileValidationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json { detail } => write!(
                formatter,
                "[rust-source-topology/profile/json-invalid] {detail}"
            ),
            Self::WrongSchema { actual } => write!(
                formatter,
                "[rust-source-topology/profile/schema-mismatch] actual={actual}"
            ),
            Self::EmptyProfiles => write!(
                formatter,
                "[rust-source-topology/profile/empty-profile-set]"
            ),
            Self::EmptyField { profile_id, field } => write!(
                formatter,
                "[rust-source-topology/profile/empty-field] profile={profile_id} field={field}"
            ),
            Self::DuplicateProfileId { profile_id } => write!(
                formatter,
                "[rust-source-topology/profile/duplicate-id] profile={profile_id}"
            ),
            Self::DuplicateFeature {
                profile_id,
                feature,
            } => write!(
                formatter,
                "[rust-source-topology/profile/duplicate-feature] profile={profile_id} feature={feature}"
            ),
            Self::TestCompileModeMismatch { profile_id } => write!(
                formatter,
                "[rust-source-topology/profile/test-mode-mismatch] profile={profile_id}"
            ),
            Self::UnsealedAmbientRustflags { profile_id } => write!(
                formatter,
                "[rust-source-topology/profile/unsealed-rustflags] profile={profile_id}"
            ),
        }
    }
}

impl std::error::Error for ProfileValidationErrorV1 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgDecisionErrorV1 {
    EmptyProfileId,
    UnsupportedTargetTriple { target_triple: String },
    MalformedAttribute { syntax: String, detail: String },
    MalformedCfgExpression { syntax: String, detail: String },
    MalformedCfgAttr { syntax: String, detail: String },
}

impl fmt::Display for CfgDecisionErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProfileId => {
                write!(formatter, "[rust-source-topology/cfg/empty-profile-id]")
            }
            Self::UnsupportedTargetTriple { target_triple } => write!(
                formatter,
                "[rust-source-topology/cfg/unsupported-target] target={target_triple}"
            ),
            Self::MalformedAttribute { syntax, detail } => write!(
                formatter,
                "[rust-source-topology/cfg/malformed-attribute] syntax={syntax:?} detail={detail}"
            ),
            Self::MalformedCfgExpression { syntax, detail } => write!(
                formatter,
                "[rust-source-topology/cfg/malformed-expression] syntax={syntax:?} detail={detail}"
            ),
            Self::MalformedCfgAttr { syntax, detail } => write!(
                formatter,
                "[rust-source-topology/cfg/malformed-cfg-attr] syntax={syntax:?} detail={detail}"
            ),
        }
    }
}

impl std::error::Error for CfgDecisionErrorV1 {}
