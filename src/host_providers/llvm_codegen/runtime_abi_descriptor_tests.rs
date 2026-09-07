use super::*;

fn descriptor_bytes() -> Vec<u8> {
    let mut bytes = vec![0; RECORD_SIZE];
    bytes[..8].copy_from_slice(MAGIC);
    for (offset, value) in [
        (8, RECORD_SIZE as u32),
        (12, 1),
        (16, 14),
        (20, 1),
        (24, 8),
        (28, 1),
        (32, 1),
        (36, 48),
        (40, 8),
        (44, 8),
        (48, 16),
        (52, 32),
        (56, 448),
        (60, 8),
        (64, 16),
        (68, 64),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes[72..86].copy_from_slice(b"x86_64-unknown");
    bytes
}

#[test]
fn rejects_nonzero_target_padding() {
    let mut bytes = descriptor_bytes();
    bytes[86] = 1;
    assert!(decode_descriptor(&bytes).unwrap_err().contains("padding"));
}

#[test]
fn rejects_layout_mismatch_before_session_use() {
    let mut bytes = descriptor_bytes();
    bytes[40..44].copy_from_slice(&3u32.to_le_bytes());
    assert!(decode_descriptor(&bytes)
        .unwrap_err()
        .contains("inconsistent"));
}

#[test]
#[ignore = "requires cargo build -p nyash_kernel --release first"]
fn reads_descriptor_from_actual_runtime_archive() {
    let descriptor = read_runtime_abi_descriptor(Path::new("target/release/libnyash_kernel.a"))
        .expect("target-compiled runtime archive descriptor");
    assert!(!descriptor.target_triple.is_empty());
}

#[test]
fn entry_abi_rejects_missing_duplicate_and_foreign_records() {
    let directory = std::env::temp_dir().join(format!(
        "nyash-entry-abi-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&directory).unwrap();
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(directory.clone());
    let valid = [b"NYENTRY1".as_slice(), &[1, 0, 0, 0, 1, 0, 0, 0]].concat();
    for (case, records, accepted) in [
        ("missing", vec![], false),
        ("valid", vec![valid.clone()], true),
        ("duplicate", vec![valid.clone(), valid.clone()], false),
        ("truncated", vec![valid[..15].to_vec()], false),
        ("foreign", vec![vec![0; 16]], false),
    ] {
        let archive = directory.join(format!("{case}.a"));
        let mut members = Vec::new();
        for (index, record) in records.iter().enumerate() {
            let source = directory.join(format!("{case}-{index}.c"));
            let object = source.with_extension("o");
            let bytes = record
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(",");
            std::fs::write(&source, format!(
                "__attribute__((used,section(\".nyash.entry_abi.v1\"))) const unsigned char entry_{index}[] = {{{bytes}}};"
            )).unwrap();
            assert!(Command::new("cc")
                .args(["-c", "-o"])
                .arg(&object)
                .arg(&source)
                .status()
                .unwrap()
                .success());
            members.push(object);
        }
        assert!(Command::new("ar")
            .arg("crs")
            .arg(&archive)
            .args(&members)
            .status()
            .unwrap()
            .success());
        let result = require_lifecycle_entry_abi(&archive);
        assert_eq!(result.is_ok(), accepted, "{case}: {result:?}");
    }
}

#[test]
#[ignore = "requires isolated release lifecycle archive"]
fn selects_actual_lifecycle_archive_and_rejects_renamed_legacy() {
    let session = LifecycleRuntimeSessionV1::select(PathBuf::from(
        "target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a",
    ))
    .expect("target-compiled lifecycle entry and runtime ABI");
    assert_eq!(
        session.descriptor().target_triple,
        "x86_64-unknown-linux-gnu"
    );
    // Admission examines content, so copying under a lifecycle filename must fail.
    let renamed = std::env::temp_dir().join(format!("renamed-lifecycle-{}.a", std::process::id()));
    std::fs::copy("target/release/libnyash_kernel.a", &renamed).unwrap();
    let result = LifecycleRuntimeSessionV1::select(renamed.clone());
    std::fs::remove_file(renamed).unwrap();
    assert!(result.unwrap_err().contains("entry ABI record"));
}
