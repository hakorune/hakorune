//! Post-link executable-plan boundary for the selected AOT lane.
//!
//! This module is deliberately disconnected from the compiler dispatcher.  A
//! future activation cell will provide the binding input and link-observed
//! entry facts; this owner only verifies their equality and returns one
//! move-only plan.  It does not resolve a registry, selector, provider, or VM
//! route.

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};

/// Test-only projection of the compile-session stamp; production activation
/// must supply the neutral owner’s branded stamp instead of minting this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AotPlanStampProjectionV1 {
    compiler_domain: NonZeroU64,
    invocation_ordinal: NonZeroU64,
}

impl AotPlanStampProjectionV1 {
    pub(crate) const fn new(compiler_domain: NonZeroU64, invocation_ordinal: NonZeroU64) -> Self {
        Self {
            compiler_domain,
            invocation_ordinal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ArtifactDigestV1([u8; 32]);

impl ArtifactDigestV1 {
    pub(crate) fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub(crate) fn from_hex(input: &str) -> Result<Self, PlanRejectV1> {
        if input.len() != 64 {
            return Err(PlanRejectV1::DigestMismatch);
        }
        let mut bytes = [0_u8; 32];
        for (index, slot) in bytes.iter_mut().enumerate() {
            let start = index * 2;
            *slot = u8::from_str_radix(&input[start..start + 2], 16)
                .map_err(|_| PlanRejectV1::DigestMismatch)?;
        }
        Ok(Self(bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AotEntryExpectationV1 {
    entry_id: u32,
    symbol: Box<str>,
}

impl AotEntryExpectationV1 {
    pub(crate) fn new(entry_id: u32, symbol: impl Into<Box<str>>) -> Result<Self, PlanRejectV1> {
        if entry_id == 0 {
            return Err(PlanRejectV1::InvalidEntry);
        }
        let symbol = symbol.into();
        if symbol.is_empty() {
            return Err(PlanRejectV1::InvalidEntry);
        }
        Ok(Self { entry_id, symbol })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LinkedArtifactBindingInputV1 {
    image_path: PathBuf,
    expected_digest: ArtifactDigestV1,
    abi_revision: u32,
    wire_revision: u32,
    stamp: AotPlanStampProjectionV1,
    entries: Box<[AotEntryExpectationV1]>,
}

impl LinkedArtifactBindingInputV1 {
    pub(crate) fn new(
        image_path: impl Into<PathBuf>,
        expected_digest: ArtifactDigestV1,
        abi_revision: u32,
        wire_revision: u32,
        stamp: AotPlanStampProjectionV1,
        entries: Vec<AotEntryExpectationV1>,
    ) -> Result<Self, PlanRejectV1> {
        if abi_revision == 0 || wire_revision == 0 || entries.is_empty() {
            return Err(PlanRejectV1::InvalidBinding);
        }
        if entries.iter().enumerate().any(|(index, entry)| {
            entries[..index]
                .iter()
                .any(|prior| prior.entry_id == entry.entry_id)
        }) {
            return Err(PlanRejectV1::DuplicateEntry);
        }
        Ok(Self {
            image_path: image_path.into(),
            expected_digest,
            abi_revision,
            wire_revision,
            stamp,
            entries: entries.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LinkedArtifactObservedFactsV1 {
    image_path: PathBuf,
    digest: ArtifactDigestV1,
    abi_revision: u32,
    wire_revision: u32,
    stamp: AotPlanStampProjectionV1,
    entries: Box<[ResolvedAotEntryV1]>,
}

impl LinkedArtifactObservedFactsV1 {
    pub(crate) fn new(
        image_path: impl Into<PathBuf>,
        digest: ArtifactDigestV1,
        abi_revision: u32,
        wire_revision: u32,
        stamp: AotPlanStampProjectionV1,
        entries: Vec<ResolvedAotEntryV1>,
    ) -> Self {
        Self {
            image_path: image_path.into(),
            digest,
            abi_revision,
            wire_revision,
            stamp,
            entries: entries.into_boxed_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedAotEntryV1 {
    entry_id: u32,
    symbol: Box<str>,
    address: u64,
}

impl ResolvedAotEntryV1 {
    pub(crate) fn new(
        entry_id: u32,
        symbol: impl Into<Box<str>>,
        address: u64,
    ) -> Result<Self, PlanRejectV1> {
        let symbol = symbol.into();
        if entry_id == 0 || symbol.is_empty() || address == 0 {
            return Err(PlanRejectV1::InvalidEntry);
        }
        Ok(Self {
            entry_id,
            symbol,
            address,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlanRejectV1 {
    ArtifactMissing,
    DigestMismatch,
    InvalidBinding,
    InvalidEntry,
    AbiMismatch,
    WireMismatch,
    StampMismatch,
    EntrySetMismatch,
    DuplicateEntry,
    PathMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RuntimeExecutablePlanV1 {
    image_path: PathBuf,
    image_digest: ArtifactDigestV1,
    abi_revision: u32,
    wire_revision: u32,
    stamp: AotPlanStampProjectionV1,
    entries: Box<[ResolvedAotEntryV1]>,
}

impl RuntimeExecutablePlanV1 {
    pub(crate) fn image_path(&self) -> &Path {
        &self.image_path
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

/// Verify the post-link facts and issue one move-only executable plan.
pub(crate) fn issue_runtime_executable_plan(
    expected: LinkedArtifactBindingInputV1,
    observed: LinkedArtifactObservedFactsV1,
) -> Result<RuntimeExecutablePlanV1, PlanRejectV1> {
    if expected.image_path != observed.image_path {
        return Err(PlanRejectV1::PathMismatch);
    }
    if expected.expected_digest != observed.digest {
        return Err(PlanRejectV1::DigestMismatch);
    }
    if expected.abi_revision != observed.abi_revision {
        return Err(PlanRejectV1::AbiMismatch);
    }
    if expected.wire_revision != observed.wire_revision {
        return Err(PlanRejectV1::WireMismatch);
    }
    if expected.stamp != observed.stamp {
        return Err(PlanRejectV1::StampMismatch);
    }
    if expected.entries.len() != observed.entries.len() {
        return Err(PlanRejectV1::EntrySetMismatch);
    }
    for (index, expected_entry) in expected.entries.iter().enumerate() {
        let observed_entry = &observed.entries[index];
        if expected_entry.entry_id != observed_entry.entry_id
            || expected_entry.symbol.as_ref() != observed_entry.symbol.as_ref()
        {
            return Err(PlanRejectV1::EntrySetMismatch);
        }
        if observed
            .entries
            .iter()
            .filter(|entry| entry.entry_id == observed_entry.entry_id)
            .count()
            != 1
        {
            return Err(PlanRejectV1::DuplicateEntry);
        }
    }
    Ok(RuntimeExecutablePlanV1 {
        image_path: observed.image_path,
        image_digest: observed.digest,
        abi_revision: observed.abi_revision,
        wire_revision: observed.wire_revision,
        stamp: observed.stamp,
        entries: observed.entries,
    })
}

pub(crate) fn sha256_file(path: &Path) -> Result<ArtifactDigestV1, PlanRejectV1> {
    if !path.is_file() {
        return Err(PlanRejectV1::ArtifactMissing);
    }
    let mut file = File::open(path).map_err(|_| PlanRejectV1::ArtifactMissing)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];
    read_all(&mut file, &mut buffer, &mut hasher)?;
    Ok(ArtifactDigestV1::from_bytes(hasher.finalize().into()))
}

fn read_all(file: &mut File, buffer: &mut [u8], hasher: &mut Sha256) -> Result<(), PlanRejectV1> {
    loop {
        let count = file
            .read(buffer)
            .map_err(|_| PlanRejectV1::ArtifactMissing)?;
        if count == 0 {
            return Ok(());
        }
        hasher.update(&buffer[..count]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn stamp() -> AotPlanStampProjectionV1 {
        AotPlanStampProjectionV1::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(7).unwrap())
    }

    fn entries() -> Vec<AotEntryExpectationV1> {
        vec![
            AotEntryExpectationV1::new(1, "hako.text.scan.substring.v1").unwrap(),
            AotEntryExpectationV1::new(2, "hako.text.scan.index_of.v1").unwrap(),
        ]
    }

    fn observed(path: &Path, digest: ArtifactDigestV1) -> LinkedArtifactObservedFactsV1 {
        LinkedArtifactObservedFactsV1::new(
            path,
            digest,
            1,
            2,
            stamp(),
            vec![
                ResolvedAotEntryV1::new(1, "hako.text.scan.substring.v1", 0x1000).unwrap(),
                ResolvedAotEntryV1::new(2, "hako.text.scan.index_of.v1", 0x2000).unwrap(),
            ],
        )
    }

    #[test]
    fn plan_issues_only_when_facts_match() {
        let path = PathBuf::from("/tmp/libnyash_kernel.a");
        let digest = ArtifactDigestV1::from_bytes([7; 32]);
        let expected =
            LinkedArtifactBindingInputV1::new(&path, digest, 1, 2, stamp(), entries()).unwrap();
        let plan = issue_runtime_executable_plan(expected, observed(&path, digest)).unwrap();
        assert_eq!(plan.entry_count(), 2);
        assert_eq!(plan.image_path(), path);
    }

    #[test]
    fn stale_stamp_and_wrong_digest_reject_before_plan() {
        let path = PathBuf::from("/tmp/libnyash_kernel.a");
        let digest = ArtifactDigestV1::from_bytes([7; 32]);
        let expected = LinkedArtifactBindingInputV1::new(
            &path,
            digest,
            1,
            2,
            AotPlanStampProjectionV1::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(8).unwrap()),
            entries(),
        )
        .unwrap();
        assert_eq!(
            issue_runtime_executable_plan(expected, observed(&path, digest)),
            Err(PlanRejectV1::StampMismatch)
        );

        let expected = LinkedArtifactBindingInputV1::new(
            &path,
            ArtifactDigestV1::from_bytes([8; 32]),
            1,
            2,
            stamp(),
            entries(),
        )
        .unwrap();
        assert_eq!(
            issue_runtime_executable_plan(expected, observed(&path, digest)),
            Err(PlanRejectV1::DigestMismatch)
        );
    }

    #[test]
    fn duplicate_expected_entry_rejects_before_plan() {
        let path = PathBuf::from("/tmp/libnyash_kernel.a");
        let duplicate = vec![
            AotEntryExpectationV1::new(1, "hako.text.scan.substring.v1").unwrap(),
            AotEntryExpectationV1::new(1, "hako.text.scan.substring.v1").unwrap(),
        ];
        assert_eq!(
            LinkedArtifactBindingInputV1::new(
                &path,
                ArtifactDigestV1::from_bytes([7; 32]),
                1,
                2,
                stamp(),
                duplicate,
            ),
            Err(PlanRejectV1::DuplicateEntry)
        );
    }

    #[test]
    fn sha256_file_rejects_missing_and_hashes_regular_file() {
        let root = std::env::temp_dir().join(format!(
            "nyllvmc_plan_digest_{}_{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        assert_eq!(sha256_file(&root), Err(PlanRejectV1::ArtifactMissing));
        fs::write(&root, b"artifact").unwrap();
        let digest = sha256_file(&root).unwrap();
        assert_eq!(
            digest,
            ArtifactDigestV1::from_hex(
                "c7c5c1d70c5dec4416ab6158afd0b223ef40c29b1dc1f97ed9428b94d4cadb1c"
            )
            .unwrap()
        );
        fs::remove_file(root).unwrap();
    }
}
