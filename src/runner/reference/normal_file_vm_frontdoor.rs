//! Production-caller-zero NormalFile front-door forge.
//!
//! This owner prepares one UTF-8 file and hands its AST to the existing Raw
//! VM-reference invocation contract. It deliberately does not execute, select
//! a CLI route, or widen the normal/default runner.

use crate::ast::ASTNode;
use crate::mir::{RawVmReferenceInvocationV1, RawVmReferenceSupportProfileV1};
use hakorune_frontend_parser::parser::{GrammarProfile, ParserBuildConfig};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct NormalFileVmFrontDoorV1;

impl NormalFileVmFrontDoorV1 {
    pub(crate) fn file_no_import_request(source_file: PathBuf) -> NormalFileRequestV1 {
        NormalFileRequestV1 {
            source_file: source_file.into_boxed_path(),
            profile: SealedNormalEntryProfileV1::file_no_import_vm_reference(),
        }
    }
}

#[derive(Debug)]
enum NormalEntryProfileV1 {
    FileNoImportVmReferenceV1,
}

#[derive(Debug)]
pub(crate) struct SealedNormalEntryProfileV1 {
    profile: NormalEntryProfileV1,
    downstream: RawVmReferenceSupportProfileV1,
    _seal: SealedNormalEntryProfileSealV1,
}

#[derive(Debug)]
struct SealedNormalEntryProfileSealV1;

impl SealedNormalEntryProfileV1 {
    fn file_no_import_vm_reference() -> Self {
        Self {
            profile: NormalEntryProfileV1::FileNoImportVmReferenceV1,
            downstream: RawVmReferenceSupportProfileV1::canonical_v1(),
            _seal: SealedNormalEntryProfileSealV1,
        }
    }

    fn into_downstream(self) -> RawVmReferenceSupportProfileV1 {
        let Self {
            profile,
            downstream,
            _seal: _,
        } = self;
        match profile {
            NormalEntryProfileV1::FileNoImportVmReferenceV1 => downstream,
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
    source_file: Box<Path>,
    ast: ASTNode,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: PreparedNormalFileSourceSealV1,
}

#[derive(Debug)]
struct PreparedNormalFileSourceSealV1;

#[derive(Debug)]
pub(crate) struct PreparedNormalFileVmHandoffV1 {
    invocation: RawVmReferenceInvocationV1,
    source: NormalFileSourceReceiptV1,
    _seal: PreparedNormalFileVmHandoffSealV1,
}

#[derive(Debug)]
struct PreparedNormalFileVmHandoffSealV1;

impl NormalFileRequestV1 {
    pub(crate) fn prepare(
        self,
    ) -> Result<PreparedNormalFileRequestV1, RejectedNormalFileSourceV1> {
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
    pub(crate) fn read_once(
        self,
    ) -> Result<LoadedNormalFileSourceV1, RejectedNormalFileSourceV1> {
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
        let ast = match crate::parser::NyashParser::parse_from_string_with_build_config(
            source_text.as_ref(),
            ParserBuildConfig {
                grammar_profile: GrammarProfile::Canonical,
                ..Default::default()
            },
        ) {
            Ok(ast) => ast,
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
        if let Some(error) = find_no_import_violation(&ast) {
            return Err(RejectedNormalFileSourceV1::SourceProfile {
                loaded: LoadedNormalFileSourceV1 {
                    source_file,
                    source_text,
                    profile,
                    receipt,
                    _seal: LoadedNormalFileSourceSealV1,
                },
                error,
            });
        }
        Ok(PreparedNormalFileSourceV1 {
            source_file,
            ast,
            profile,
            receipt,
            _seal: PreparedNormalFileSourceSealV1,
        })
    }
}

impl PreparedNormalFileSourceV1 {
    pub(crate) fn prepare_raw_vm_handoff(self) -> PreparedNormalFileVmHandoffV1 {
        let Self {
            source_file,
            ast,
            profile,
            receipt,
            _seal: _,
        } = self;
        let source_file = source_file.to_string_lossy().into_owned().into_boxed_str();
        PreparedNormalFileVmHandoffV1 {
            invocation: profile.into_downstream().into_invocation(ast, Some(source_file)),
            source: receipt,
            _seal: PreparedNormalFileVmHandoffSealV1,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn request(path: PathBuf) -> NormalFileRequestV1 {
        NormalFileVmFrontDoorV1::file_no_import_request(path)
    }

    fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::write(&path, source).expect("write fixture source");
        path
    }

    #[test]
    fn empty_path_rejects_before_file_read() {
        let rejected = request(PathBuf::new()).prepare().expect_err("empty path rejects");
        assert_eq!(rejected.stage(), NormalFileSourceStageV1::Profile);
        assert!(matches!(
            rejected.error(),
            NormalFileSourceErrorV1::Profile(NormalFileProfileErrorV1::EmptySourcePath)
        ));
    }

    #[test]
    fn reads_and_parses_one_canonical_file_once() {
        let dir = tempdir().expect("tempdir");
        let path = write_source(dir.path(), "scalar.hako", "42");
        let prepared = request(path)
            .prepare()
            .expect("profile")
            .read_once()
            .expect("read")
            .parse_once()
            .expect("parse");
        assert_eq!(prepared.receipt.read_count, 1);
        assert_eq!(prepared.receipt.parse_count, 1);
        assert_eq!(prepared.receipt.utf8_len, 2);
    }

    #[test]
    fn parse_rejection_retains_one_read_receipt() {
        let dir = tempdir().expect("tempdir");
        let path = write_source(dir.path(), "invalid.hako", "@");
        let rejected = request(path)
            .prepare()
            .expect("profile")
            .read_once()
            .expect("read")
            .parse_once()
            .expect_err("parse rejects");
        assert_eq!(rejected.stage(), NormalFileSourceStageV1::Parse);
        let RejectedNormalFileSourceV1::Parse { loaded, .. } = rejected else {
            panic!("expected parse rejection");
        };
        assert_eq!(loaded.receipt.read_count, 1);
        assert_eq!(loaded.receipt.parse_count, 1);
    }

    #[test]
    fn source_using_rejects_after_parse_before_handoff() {
        let dir = tempdir().expect("tempdir");
        let path = write_source(dir.path(), "using.hako", "using foo");
        let rejected = request(path)
            .prepare()
            .expect("profile")
            .read_once()
            .expect("read")
            .parse_once()
            .expect_err("using rejects");
        assert_eq!(rejected.stage(), NormalFileSourceStageV1::SourceProfile);
        assert!(matches!(
            rejected.error(),
            NormalFileSourceErrorV1::SourceProfile(NormalFileSourceProfileErrorV1::UsingStatement)
        ));
    }

    #[test]
    fn consuming_handoff_keeps_the_existing_raw_profile_paired() {
        let dir = tempdir().expect("tempdir");
        let path = write_source(dir.path(), "handoff.hako", "42");
        let handoff = request(path)
            .prepare()
            .expect("profile")
            .read_once()
            .expect("read")
            .parse_once()
            .expect("parse")
            .prepare_raw_vm_handoff();
        assert_eq!(handoff.source.read_count, 1);
        assert_eq!(handoff.source.parse_count, 1);
        let invocation = handoff.into_raw_vm_reference_invocation();
        assert_eq!(invocation.compile.module_name.as_ref(), "main");
        assert_eq!(
            invocation.compile.profile,
            crate::mir::RawPublishedCompileProfileV1::narrow_v1()
        );
    }

    #[cfg(feature = "vm-reference")]
    #[test]
    fn handoff_reuses_the_existing_raw_vm_reference_execution_terminal() {
        let dir = tempdir().expect("tempdir");
        let first = write_source(dir.path(), "first.hako", "42");
        let second = write_source(dir.path(), "second.hako", "255");
        let mut compiler = crate::mir::MirCompiler::new();

        for (path, expected_status) in [(first, 42), (second, 255)] {
            let invocation = request(path)
                .prepare()
                .expect("profile")
                .read_once()
                .expect("read")
                .parse_once()
                .expect("parse")
                .prepare_raw_vm_handoff()
                .into_raw_vm_reference_invocation();
            let report = compiler
                .run_raw_vm_reference_v1(invocation)
                .expect("existing Raw VM-reference terminal should execute handoff");
            assert_eq!(report.status_code(), expected_status);
            assert_eq!(report.diagnostic_tag(), None);
        }
    }
}
