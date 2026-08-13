//! One-shot post-link observation and publication owner for selected Dynamic.
//!
//! Preparation links only to a same-directory temporary executable, observes
//! real object/archive/executable facts, and issues one move-only receipt.
//! The consuming rename exists for W6-E but has no production caller in W6-D.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::static_artifact_descriptor::{
    expected_descriptor_from_json, observe_descriptor, require_archive_call_symbols,
    require_object_call_symbols, sha256_file, StaticAotArtifactDescriptorV1,
    StaticArtifactRejectV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct StaticLinkedAotArtifactReceiptV1 {
    object_path: PathBuf,
    runtime_archive_path: PathBuf,
    candidate_path: PathBuf,
    object_digest: [u8; 32],
    runtime_archive_digest: [u8; 32],
    executable_digest: [u8; 32],
    descriptor_digest: [u8; 32],
    descriptor: StaticAotArtifactDescriptorV1,
    symbol_census: StaticArtifactSymbolCensusV1,
    final_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
struct StaticArtifactSymbolCensusV1 {
    required: u8,
    object_undefined: u8,
    archive_defined: u8,
    executable_defined: u8,
}

impl StaticLinkedAotArtifactReceiptV1 {
    pub(super) fn final_path(&self) -> &Path {
        &self.final_path
    }

    #[cfg(test)]
    pub(super) fn artifact_paths(&self) -> (&Path, &Path, &Path) {
        (
            &self.object_path,
            &self.runtime_archive_path,
            &self.candidate_path,
        )
    }

    #[cfg(test)]
    pub(super) const fn descriptor_digest(&self) -> &[u8; 32] {
        &self.descriptor_digest
    }

    #[cfg(test)]
    pub(super) const fn artifact_digests(&self) -> (&[u8; 32], &[u8; 32], &[u8; 32]) {
        (
            &self.object_digest,
            &self.runtime_archive_digest,
            &self.executable_digest,
        )
    }

    #[cfg(test)]
    pub(super) const fn descriptor(&self) -> &StaticAotArtifactDescriptorV1 {
        &self.descriptor
    }

    #[cfg(test)]
    pub(super) const fn symbol_census(&self) -> (u8, u8, u8, u8) {
        (
            self.symbol_census.required,
            self.symbol_census.object_undefined,
            self.symbol_census.archive_defined,
            self.symbol_census.executable_defined,
        )
    }
}

#[derive(Debug)]
pub(super) struct PreparedStaticAotArtifactPublicationV1 {
    receipt: Option<StaticLinkedAotArtifactReceiptV1>,
}

impl PreparedStaticAotArtifactPublicationV1 {
    pub(super) fn receipt(&self) -> &StaticLinkedAotArtifactReceiptV1 {
        self.receipt
            .as_ref()
            .expect("prepared static artifact retains its receipt")
    }

    /// W6-E consuming commit.  W6-D deliberately has no production caller.
    pub(super) fn commit(
        mut self,
    ) -> Result<StaticLinkedAotArtifactReceiptV1, StaticArtifactRejectV1> {
        let final_path = self.receipt().final_path().to_path_buf();
        let candidate_path = self.receipt().candidate_path.clone();
        fs::rename(&candidate_path, &final_path)
            .map_err(|_| StaticArtifactRejectV1::PublishFailed)?;
        Ok(self
            .receipt
            .take()
            .expect("prepared static artifact receipt consumed once"))
    }
}

impl Drop for PreparedStaticAotArtifactPublicationV1 {
    fn drop(&mut self) {
        if let Some(receipt) = &self.receipt {
            let _ = fs::remove_file(&receipt.candidate_path);
        }
    }
}

pub(super) struct StaticAotArtifactPublicationTxnV1;

impl StaticAotArtifactPublicationTxnV1 {
    pub(super) fn prepare(
        input_json: &Path,
        object_path: &Path,
        final_path: &Path,
        runtime_archive: &Path,
        extra_libs: Option<&str>,
    ) -> Result<PreparedStaticAotArtifactPublicationV1, StaticArtifactRejectV1> {
        Self::prepare_with_linker(
            input_json,
            object_path,
            final_path,
            runtime_archive,
            |object, candidate, archive| {
                super::super::boundary_driver::link_object_to_exe_with_archive(
                    object, candidate, archive, extra_libs,
                )
                .map_err(|_| StaticArtifactRejectV1::LinkFailed)
            },
        )
    }

    fn prepare_with_linker<F>(
        input_json: &Path,
        object_path: &Path,
        final_path: &Path,
        runtime_archive: &Path,
        link: F,
    ) -> Result<PreparedStaticAotArtifactPublicationV1, StaticArtifactRejectV1>
    where
        F: FnOnce(&Path, &Path, &Path) -> Result<(), StaticArtifactRejectV1>,
    {
        let expected = expected_descriptor_from_json(input_json)?
            .ok_or(StaticArtifactRejectV1::MissingDescriptor)?;
        let (object_descriptor, object_descriptor_digest) = observe_descriptor(object_path)?;
        if object_descriptor != expected {
            return Err(StaticArtifactRejectV1::DescriptorMismatch);
        }
        require_object_call_symbols(object_path, &expected, true)?;
        require_archive_call_symbols(runtime_archive, &expected)?;
        let object_digest = sha256_file(object_path)?;
        let runtime_archive_digest = sha256_file(runtime_archive)?;
        let candidate_path = candidate_path_for(final_path)?;
        let preparation = (|| {
            link(object_path, &candidate_path, runtime_archive)?;
            let (executable_descriptor, executable_descriptor_digest) =
                observe_descriptor(&candidate_path)?;
            if executable_descriptor != expected
                || executable_descriptor_digest != object_descriptor_digest
            {
                return Err(StaticArtifactRejectV1::DescriptorMismatch);
            }
            require_object_call_symbols(&candidate_path, &expected, false)?;
            let executable_digest = sha256_file(&candidate_path)?;
            Ok(StaticLinkedAotArtifactReceiptV1 {
                object_path: object_path.to_path_buf(),
                runtime_archive_path: runtime_archive.to_path_buf(),
                candidate_path: candidate_path.clone(),
                object_digest,
                runtime_archive_digest,
                executable_digest,
                descriptor_digest: executable_descriptor_digest,
                descriptor: executable_descriptor,
                symbol_census: StaticArtifactSymbolCensusV1 {
                    required: 3,
                    object_undefined: 3,
                    archive_defined: 3,
                    executable_defined: 3,
                },
                final_path: final_path.to_path_buf(),
            })
        })();
        match preparation {
            Ok(receipt) => Ok(PreparedStaticAotArtifactPublicationV1 {
                receipt: Some(receipt),
            }),
            Err(error) => {
                let _ = fs::remove_file(candidate_path);
                Err(error)
            }
        }
    }
}

fn candidate_path_for(final_path: &Path) -> Result<PathBuf, StaticArtifactRejectV1> {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let parent = final_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|_| StaticArtifactRejectV1::PublishFailed)?;
    let name = final_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(StaticArtifactRejectV1::PublishFailed)?;
    for _ in 0..32 {
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            ".{}.dynamic-v2.{}.{}.tmp",
            name,
            std::process::id(),
            ordinal
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(StaticArtifactRejectV1::PublishFailed)
}

#[cfg(test)]
mod tests;
