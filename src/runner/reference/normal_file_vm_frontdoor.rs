//! Production NormalFile front-door forge.
//!
//! This owner prepares one UTF-8 file and hands its AST to the existing Raw
//! VM-reference invocation contract. It deliberately does not execute, select
//! a CLI route, or widen the normal/default runner.

use crate::ast::ASTNode;
use crate::mir::{
    CanonicalSourceBytesDigestV1, RawVmReferenceInvocationV1, RawVmReferenceSupportProfileV1,
};
use hakorune_frontend_parser::parser::{GrammarProfile, ParserBuildConfig};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

mod parser_source_handoff;
mod raw_source_handoff;
mod script_source_input;
mod source_plan_input;

pub(crate) use parser_source_handoff::CanonicalParserSourceHandoffV1;
#[allow(unused_imports)]
pub(crate) use source_plan_input::{
    CanonicalCoreSourcePlanHandoffErrorV1, ClassifiedNormalFileSourcePlanV1,
    NormalFileSourcePlanRouteErrorV1, PreparedNormalFileSourcePlanRequestV1,
    RejectedCanonicalCoreSourcePlanHandoffV1, RejectedNormalFileSourcePlanRouteV1,
    RejectedNormalFileSourcePlanningV1,
};

#[derive(Debug)]
pub(crate) struct NormalFileVmFrontDoorV1;

impl NormalFileVmFrontDoorV1 {
    pub(crate) fn file_no_import_request(source_file: PathBuf) -> NormalFileRequestV1 {
        NormalFileRequestV1 {
            source_file: source_file.into_boxed_path(),
            profile: SealedNormalEntryProfileV1::file_no_import_vm_reference(),
        }
    }

    pub(crate) fn file_canonical_core_request(source_file: PathBuf) -> NormalFileRequestV1 {
        NormalFileRequestV1 {
            source_file: source_file.into_boxed_path(),
            profile: SealedNormalEntryProfileV1::file_canonical_core_vm_reference(),
        }
    }
}

#[derive(Debug)]
enum NormalEntryProfileV1 {
    FileNoImportVmReferenceV1 {
        downstream: RawVmReferenceSupportProfileV1,
    },
    FileCanonicalCoreVmReferenceV1,
}

#[derive(Debug)]
pub(crate) struct SealedNormalEntryProfileV1 {
    profile: NormalEntryProfileV1,
    _seal: SealedNormalEntryProfileSealV1,
}

#[derive(Debug)]
struct SealedNormalEntryProfileSealV1;

impl SealedNormalEntryProfileV1 {
    fn file_no_import_vm_reference() -> Self {
        Self {
            profile: NormalEntryProfileV1::FileNoImportVmReferenceV1 {
                downstream: RawVmReferenceSupportProfileV1::canonical_v1(),
            },
            _seal: SealedNormalEntryProfileSealV1,
        }
    }

    fn file_canonical_core_vm_reference() -> Self {
        Self {
            profile: NormalEntryProfileV1::FileCanonicalCoreVmReferenceV1,
            _seal: SealedNormalEntryProfileSealV1,
        }
    }

    fn into_raw_downstream(
        self,
    ) -> Result<RawVmReferenceSupportProfileV1, SealedNormalEntryProfileV1> {
        let Self { profile, _seal: _ } = self;
        match profile {
            NormalEntryProfileV1::FileNoImportVmReferenceV1 { downstream } => Ok(downstream),
            profile @ NormalEntryProfileV1::FileCanonicalCoreVmReferenceV1 => {
                Err(SealedNormalEntryProfileV1 {
                    profile,
                    _seal: SealedNormalEntryProfileSealV1,
                })
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct NormalFileRequestV1 {
    source_file: Box<Path>,
    profile: SealedNormalEntryProfileV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalFileSourceStageV1 {
    Profile,
    Read,
    Parse,
    SourceProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalFileProfileErrorV1 {
    EmptySourcePath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalFileReadErrorV1 {
    NotFound,
    InvalidUtf8,
    Other(ErrorKind),
}

#[derive(Debug)]
pub(crate) struct NormalFileParseErrorV1 {
    detail: Box<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalFileSourceProfileErrorV1 {
    UsingStatement,
    ImportStatement,
}

#[derive(Debug)]
pub(crate) enum NormalFileSourceErrorV1<'a> {
    Profile(NormalFileProfileErrorV1),
    Read(NormalFileReadErrorV1),
    Parse(&'a NormalFileParseErrorV1),
    SourceProfile(NormalFileSourceProfileErrorV1),
}

#[derive(Debug)]
pub(crate) enum RejectedNormalFileSourceV1 {
    Profile {
        request: NormalFileRequestV1,
        error: NormalFileProfileErrorV1,
    },
    Read {
        request: PreparedNormalFileRequestV1,
        error: NormalFileReadErrorV1,
    },
    Parse {
        loaded: LoadedNormalFileSourceV1,
        error: NormalFileParseErrorV1,
    },
    SourceProfile {
        loaded: LoadedNormalFileSourceV1,
        error: NormalFileSourceProfileErrorV1,
    },
}

impl RejectedNormalFileSourceV1 {
    pub(crate) const fn stage(&self) -> NormalFileSourceStageV1 {
        match self {
            Self::Profile { .. } => NormalFileSourceStageV1::Profile,
            Self::Read { .. } => NormalFileSourceStageV1::Read,
            Self::Parse { .. } => NormalFileSourceStageV1::Parse,
            Self::SourceProfile { .. } => NormalFileSourceStageV1::SourceProfile,
        }
    }

    pub(crate) fn error(&self) -> NormalFileSourceErrorV1<'_> {
        match self {
            Self::Profile { error, .. } => NormalFileSourceErrorV1::Profile(*error),
            Self::Read { error, .. } => NormalFileSourceErrorV1::Read(*error),
            Self::Parse { error, .. } => NormalFileSourceErrorV1::Parse(error),
            Self::SourceProfile { error, .. } => NormalFileSourceErrorV1::SourceProfile(*error),
        }
    }

    pub(crate) fn discard(self) {}
}

#[derive(Debug)]
pub(crate) struct PreparedNormalFileRequestV1 {
    source_file: Box<Path>,
    profile: SealedNormalEntryProfileV1,
}

#[derive(Debug)]
pub(crate) struct NormalFileSourceReceiptV1 {
    source_identity: Box<str>,
    source_digest: CanonicalSourceBytesDigestV1,
    utf8_len: usize,
    read_count: u8,
    parse_count: u8,
    _seal: NormalFileSourceReceiptSealV1,
}

#[derive(Debug)]
struct NormalFileSourceReceiptSealV1;

#[derive(Debug)]
pub(crate) struct LoadedNormalFileSourceV1 {
    source_file: Box<Path>,
    source_text: Box<str>,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: LoadedNormalFileSourceSealV1,
}

#[derive(Debug)]
struct LoadedNormalFileSourceSealV1;

#[derive(Debug)]
pub(crate) struct PreparedNormalFileSourceV1 {
    route: PreparedNormalFileParsedRouteV1,
    _seal: PreparedNormalFileSourceSealV1,
}

#[derive(Debug)]
struct PreparedNormalFileSourceSealV1;

/// Profile-selected parser route. The split happens before Script-A co-seal,
/// so Raw can close the parser sibling without constructing canonical input.
#[derive(Debug)]
enum PreparedNormalFileParsedRouteV1 {
    Raw {
        source_file: Box<Path>,
        source: raw_source_handoff::PreparedCanonicalParserRawSourceV1,
    },
    Canonical {
        source_file: Box<Path>,
        source: CanonicalParserSourceHandoffV1,
    },
}

#[derive(Debug)]
pub(crate) struct PreparedNormalFileVmHandoffV1 {
    invocation: RawVmReferenceInvocationV1,
    source: NormalFileSourceReceiptV1,
    _seal: PreparedNormalFileVmHandoffSealV1,
}

#[derive(Debug)]
struct PreparedNormalFileVmHandoffSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalFileVmHandoffErrorV1 {
    ProfileExcludesRawVmReference,
    SourceAuthorityUnavailable,
    SourceIncomplete,
    SourceIntegrityInvalid,
    CompatibilityClosure,
    UsingStatement,
    ImportStatement,
}

#[derive(Debug)]
pub(crate) struct RejectedNormalFileVmHandoffV1 {
    owner: RejectedNormalFileVmHandoffOwnerV1,
    error: NormalFileVmHandoffErrorV1,
}

#[derive(Debug)]
enum RejectedNormalFileVmHandoffOwnerV1 {
    Prepared(PreparedNormalFileSourceV1),
    Parser {
        source_file: Box<Path>,
        rejected: raw_source_handoff::RejectedRawVmSourceExtractionV1,
    },
    Extracted {
        source_file: Box<Path>,
        extracted: raw_source_handoff::PreparedRawVmSourceExtractionV1,
    },
}

impl RejectedNormalFileVmHandoffV1 {
    pub(crate) const fn error(&self) -> NormalFileVmHandoffErrorV1 {
        self.error
    }

    pub(crate) fn discard(self) {
        match self.owner {
            RejectedNormalFileVmHandoffOwnerV1::Prepared(source) => {
                source.discard_at_named_terminal()
            }
            RejectedNormalFileVmHandoffOwnerV1::Parser {
                source_file,
                rejected,
            } => {
                drop(source_file);
                rejected.discard();
            }
            RejectedNormalFileVmHandoffOwnerV1::Extracted {
                source_file,
                extracted,
            } => {
                drop(source_file);
                extracted.discard();
            }
        }
    }
}

impl NormalFileRequestV1 {
    pub(crate) fn prepare(self) -> Result<PreparedNormalFileRequestV1, RejectedNormalFileSourceV1> {
        if self.source_file.as_os_str().is_empty() {
            return Err(RejectedNormalFileSourceV1::Profile {
                request: self,
                error: NormalFileProfileErrorV1::EmptySourcePath,
            });
        }
        Ok(PreparedNormalFileRequestV1 {
            source_file: self.source_file,
            profile: self.profile,
        })
    }
}

impl PreparedNormalFileRequestV1 {
    pub(crate) fn read_once(self) -> Result<LoadedNormalFileSourceV1, RejectedNormalFileSourceV1> {
        let Self {
            source_file,
            profile,
        } = self;
        let source_text = match std::fs::read_to_string(&source_file) {
            Ok(source_text) => source_text.into_boxed_str(),
            Err(error) => {
                let error = classify_read_error(&error);
                return Err(RejectedNormalFileSourceV1::Read {
                    request: PreparedNormalFileRequestV1 {
                        source_file,
                        profile,
                    },
                    error,
                });
            }
        };
        let receipt = NormalFileSourceReceiptV1 {
            source_identity: source_file.to_string_lossy().into_owned().into_boxed_str(),
            source_digest: CanonicalSourceBytesDigestV1::from_utf8_bytes(source_text.as_bytes()),
            utf8_len: source_text.len(),
            read_count: 1,
            parse_count: 0,
            _seal: NormalFileSourceReceiptSealV1,
        };
        Ok(LoadedNormalFileSourceV1 {
            source_file,
            source_text,
            profile,
            receipt,
            _seal: LoadedNormalFileSourceSealV1,
        })
    }
}

impl LoadedNormalFileSourceV1 {
    pub(crate) fn parse_once(
        self,
    ) -> Result<PreparedNormalFileSourceV1, RejectedNormalFileSourceV1> {
        let Self {
            source_file,
            source_text,
            profile,
            mut receipt,
            _seal: _,
        } = self;
        receipt.parse_count = 1;
        let product =
            match crate::parser::string_postpass_entry::parse_with_callable_parameter_source(
                source_text.as_ref().to_owned(),
                Some(100_000),
                ParserBuildConfig {
                    grammar_profile: GrammarProfile::Canonical,
                    ..Default::default()
                },
            ) {
                Ok(product) => product,
                Err(error) => {
                    return Err(RejectedNormalFileSourceV1::Parse {
                        loaded: LoadedNormalFileSourceV1 {
                            source_file,
                            source_text,
                            profile,
                            receipt,
                            _seal: LoadedNormalFileSourceSealV1,
                        },
                        error: NormalFileParseErrorV1 {
                            detail: format!("{error:?}").into_boxed_str(),
                        },
                    });
                }
            };
        let product = product.into_normal_file_source();
        let route =
            product.consume_once(|source, script_rows| match profile.into_raw_downstream() {
                Ok(downstream) => PreparedNormalFileParsedRouteV1::Raw {
                    source_file,
                    source: raw_source_handoff::PreparedCanonicalParserRawSourceV1::new(
                        source,
                        script_rows,
                        downstream,
                        receipt,
                    ),
                },
                Err(profile) => PreparedNormalFileParsedRouteV1::Canonical {
                    source_file,
                    source: CanonicalParserSourceHandoffV1::new(
                        source,
                        script_rows,
                        profile,
                        receipt,
                    ),
                },
            });
        Ok(PreparedNormalFileSourceV1 {
            route,
            _seal: PreparedNormalFileSourceSealV1,
        })
    }
}

impl PreparedNormalFileSourceV1 {
    pub(crate) fn prepare_raw_vm_handoff(
        self,
    ) -> Result<PreparedNormalFileVmHandoffV1, RejectedNormalFileVmHandoffV1> {
        let Self { route, _seal } = self;
        drop(_seal);
        let (source_file, raw_source) = match route {
            PreparedNormalFileParsedRouteV1::Raw {
                source_file,
                source,
            } => (source_file, source),
            PreparedNormalFileParsedRouteV1::Canonical {
                source_file,
                source,
            } => {
                return Err(RejectedNormalFileVmHandoffV1 {
                    owner: RejectedNormalFileVmHandoffOwnerV1::Prepared(
                        PreparedNormalFileSourceV1 {
                            route: PreparedNormalFileParsedRouteV1::Canonical {
                                source_file,
                                source,
                            },
                            _seal: PreparedNormalFileSourceSealV1,
                        },
                    ),
                    error: NormalFileVmHandoffErrorV1::ProfileExcludesRawVmReference,
                });
            }
        };
        let extracted = match raw_source.extract_once() {
            Ok(extracted) => extracted,
            Err(rejected) => {
                let error = match rejected.error() {
                    crate::parser::ParserNormalRawVmSourceExtractionErrorV1::SourceAuthorityUnavailable => {
                        NormalFileVmHandoffErrorV1::SourceAuthorityUnavailable
                    }
                    crate::parser::ParserNormalRawVmSourceExtractionErrorV1::Incomplete => {
                        NormalFileVmHandoffErrorV1::SourceIncomplete
                    }
                    crate::parser::ParserNormalRawVmSourceExtractionErrorV1::IntegrityInvalid => {
                        NormalFileVmHandoffErrorV1::SourceIntegrityInvalid
                    }
                    crate::parser::ParserNormalRawVmSourceExtractionErrorV1::CompatibilityClosure => {
                        NormalFileVmHandoffErrorV1::CompatibilityClosure
                    }
                };
                return Err(RejectedNormalFileVmHandoffV1 {
                    owner: RejectedNormalFileVmHandoffOwnerV1::Parser {
                        source_file,
                        rejected,
                    },
                    error,
                });
            }
        };
        if let Some(error) = find_no_import_violation(extracted.ast()) {
            let error = match error {
                NormalFileSourceProfileErrorV1::UsingStatement => {
                    NormalFileVmHandoffErrorV1::UsingStatement
                }
                NormalFileSourceProfileErrorV1::ImportStatement => {
                    NormalFileVmHandoffErrorV1::ImportStatement
                }
            };
            return Err(RejectedNormalFileVmHandoffV1 {
                owner: RejectedNormalFileVmHandoffOwnerV1::Extracted {
                    source_file,
                    extracted,
                },
                error,
            });
        }
        let source_identity = source_file.to_string_lossy().into_owned().into_boxed_str();
        let (invocation, receipt) = extracted.into_invocation(source_identity);
        Ok(PreparedNormalFileVmHandoffV1 {
            invocation,
            source: receipt,
            _seal: PreparedNormalFileVmHandoffSealV1,
        })
    }
}

impl PreparedNormalFileSourceV1 {
    fn discard_at_named_terminal(self) {
        let Self { route, _seal } = self;
        drop(_seal);
        match route {
            PreparedNormalFileParsedRouteV1::Raw {
                source_file,
                source,
            } => {
                drop(source_file);
                source.discard_at_wrong_route_terminal();
            }
            PreparedNormalFileParsedRouteV1::Canonical {
                source_file,
                source,
            } => {
                drop(source_file);
                source.discard_at_wrong_route_terminal();
            }
        }
    }

    #[cfg(test)]
    fn receipt(&self) -> &NormalFileSourceReceiptV1 {
        match &self.route {
            PreparedNormalFileParsedRouteV1::Raw { source, .. } => source.receipt(),
            PreparedNormalFileParsedRouteV1::Canonical { source, .. } => source.receipt(),
        }
    }

    #[cfg(test)]
    fn profile_is_canonical_core(&self) -> bool {
        matches!(
            &self.route,
            PreparedNormalFileParsedRouteV1::Canonical { .. }
        )
    }
}

impl PreparedNormalFileVmHandoffV1 {
    pub(crate) fn into_raw_vm_reference_invocation(self) -> RawVmReferenceInvocationV1 {
        self.invocation
    }
}

fn classify_read_error(error: &std::io::Error) -> NormalFileReadErrorV1 {
    match error.kind() {
        ErrorKind::NotFound => NormalFileReadErrorV1::NotFound,
        ErrorKind::InvalidData => NormalFileReadErrorV1::InvalidUtf8,
        kind => NormalFileReadErrorV1::Other(kind),
    }
}

fn find_no_import_violation(node: &ASTNode) -> Option<NormalFileSourceProfileErrorV1> {
    let direct = match node {
        ASTNode::UsingStatement { .. } => Some(NormalFileSourceProfileErrorV1::UsingStatement),
        ASTNode::ImportStatement { .. } => Some(NormalFileSourceProfileErrorV1::ImportStatement),
        _ => None,
    };
    if direct.is_some() {
        return direct;
    }
    let mut found = None;
    node.for_each_child(&mut |child| {
        if found.is_none() {
            found = find_no_import_violation(child);
        }
    });
    found
}

impl SealedNormalEntryProfileV1 {
    pub(super) fn is_canonical_core(&self) -> bool {
        matches!(
            self.profile,
            NormalEntryProfileV1::FileCanonicalCoreVmReferenceV1
        )
    }
}

#[cfg(test)]
#[path = "normal_file_vm_frontdoor/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "normal_file_vm_frontdoor/atomic_root_cutover_tests.rs"]
mod atomic_root_cutover_tests;

#[cfg(test)]
#[path = "normal_file_vm_frontdoor/result_carrier_p0.rs"]
mod result_carrier_p0;
