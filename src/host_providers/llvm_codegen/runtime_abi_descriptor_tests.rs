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

#[test]
fn native_array_inventory_requires_exact_single_external_function_definitions() {
    let valid: Vec<_> = NATIVE_ARRAY_SYMBOLS
        .iter()
        .map(|name| format!("{name} T 0 1"))
        .collect();
    assert!(require_native_array_symbol_inventory(&valid.join("\n")).is_ok());
    for index in 0..NATIVE_ARRAY_SYMBOLS.len() {
        let mut rows = valid.clone();
        rows.remove(index);
        assert!(require_native_array_symbol_inventory(&rows.join("\n")).is_err());
        for row in [
            format!("{} U", NATIVE_ARRAY_SYMBOLS[index]),
            format!("{} D 0 8", NATIVE_ARRAY_SYMBOLS[index]),
            format!("{} t 0 1", NATIVE_ARRAY_SYMBOLS[index]),
            format!("{}_extra T 0 1", NATIVE_ARRAY_SYMBOLS[index]),
        ] {
            let mut rows = valid.clone();
            rows[index] = row;
            assert!(require_native_array_symbol_inventory(&rows.join("\n")).is_err());
        }
    }
    let mut duplicate = valid.clone();
    duplicate.push(valid[0].clone());
    assert!(require_native_array_symbol_inventory(&duplicate.join("\n")).is_err());
}

#[test]
fn native_array_availability_reads_defined_symbols_from_actual_archives() {
    let directory =
        std::env::temp_dir().join(format!("nyash-array-symbols-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    struct Cleanup(PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(directory.clone());
    let mut objects = Vec::new();
    for (index, name) in NATIVE_ARRAY_SYMBOLS.iter().enumerate() {
        let source = directory.join(format!("{index}.c"));
        let object = source.with_extension("o");
        std::fs::write(
            &source,
            format!(
                "void function_{index}(void) __asm__(\"{name}\"); void function_{index}(void) {{}}"
            ),
        )
        .unwrap();
        assert!(Command::new("cc")
            .arg("-c")
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .status()
            .unwrap()
            .success());
        objects.push(object);
    }
    for missing in 0..=NATIVE_ARRAY_SYMBOLS.len() {
        let archive = directory.join(format!("missing-{missing}.a"));
        assert!(Command::new("ar")
            .arg("crs")
            .arg(&archive)
            .args(
                objects
                    .iter()
                    .enumerate()
                    .filter_map(|(index, path)| (index != missing).then_some(path))
            )
            .status()
            .unwrap()
            .success());
        let result = require_native_array_symbols(&archive);
        assert_eq!(
            result.is_ok(),
            missing == NATIVE_ARRAY_SYMBOLS.len(),
            "{result:?}"
        );
        assert_script_input_binding(&archive, missing == NATIVE_ARRAY_SYMBOLS.len());
    }
    let source = directory.join("undefined.c");
    let object = source.with_extension("o");
    std::fs::write(
        &source,
        format!(
            "extern void absent(void) __asm__(\"{}\"); void reference(void) {{ absent(); }}",
            NATIVE_ARRAY_SYMBOLS[0]
        ),
    )
    .unwrap();
    assert!(Command::new("cc")
        .arg("-c")
        .arg(&source)
        .arg("-o")
        .arg(&object)
        .status()
        .unwrap()
        .success());
    let archive = directory.join("undefined.a");
    assert!(Command::new("ar")
        .arg("crs")
        .arg(&archive)
        .arg(&object)
        .args(&objects[1..])
        .status()
        .unwrap()
        .success());
    assert!(require_native_array_symbols(&archive).is_err());
    // Different archive members with the same required definition are ambiguous.
    let duplicate = directory.join("duplicate.o");
    std::fs::copy(&objects[0], &duplicate).unwrap();
    let archive = directory.join("duplicate.a");
    assert!(Command::new("ar")
        .arg("crs")
        .arg(&archive)
        .args(&objects)
        .arg(&duplicate)
        .status()
        .unwrap()
        .success());
    assert!(require_native_array_symbols(&archive).is_err());
    assert!(require_native_array_symbols(&directory.join("absent.a")).is_err());
}

#[test]
fn bound_pair_input_checks_runtime_before_serialization_and_ignores_array_symbols() {
    use super::super::lifecycle_invocation::LifecycleInvocationInputV1;
    use crate::mir::{MirCompiler, NormalCompileRequestV1};
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../apps/typed-object-birth-min/main.hako"),
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source must be retained")
        };
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, _| -> Result<(), String> {
                let physical = view.issue_lifecycle_physical_abi_input()?;
                let mut descriptor = decode_descriptor(&descriptor_bytes())?;
                descriptor.target_triple = "x86_64-unknown-linux-gnu".into();
                // A selected session fixture: binding typed objects must not inspect Array symbols.
                let session = LifecycleRuntimeSessionV1 {
                    runtime_archive: PathBuf::from("fixture-without-array-symbols.a"),
                    descriptor,
                };
                let expected = crate::mir::emit_lifecycle_physical_abi_json(&physical)?;
                let bound = LifecycleInvocationInputV1::bind(physical.clone(), &session)?;
                assert_eq!(bound.serialize()?, expected);
                assert_eq!(bound.runtime_archive(), session.runtime_archive());
                for index in 0..5 {
                    let mut bad = session.clone();
                    match index {
                        0 => bad.descriptor.target_triple = "other".into(),
                        1 => bad.descriptor.pointer_width = 4,
                        2 => bad.descriptor.fault_abi_version = 2,
                        3 => bad.descriptor.frame_size += 8,
                        _ => bad.descriptor.diagnostic_details_offset = 8,
                    }
                    assert!(LifecycleInvocationInputV1::bind(physical.clone(), &bad).is_err());
                }
                assert!(session
                    .require_input(PublishedLifecycleRuntimeRequirementsV1::NativeArray, 1)
                    .is_err());
                assert!(session
                    .require_input(
                        PublishedLifecycleRuntimeRequirementsV1::TypedObject { storage_profile: 0 },
                        1
                    )
                    .is_err());
                Err("bound-pair-input-verified".into())
            },
        );
        assert!(matches!(result, Err(error) if error.contains("bound-pair-input-verified")));
    });
}

// These archive fixtures prove availability/binding, not runtime call behavior.
fn assert_script_input_binding(archive: &Path, available: bool) {
    use super::super::lifecycle_invocation::LifecycleInvocationInputV1;
    use crate::mir::{MirCompiler, NormalCompileRequestV1};
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for terminal in ["return 30", "return"] {
            let parsed =
                crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                    &format!("local a: Array<i64> = [10, 20]\n{terminal}"),
                    crate::parser::ParserBuildConfig::default(),
                )
                .unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
            else {
                panic!("source must be retained")
            };
            MirCompiler::with_options(true)
                .compile_normal_with_published(
                    NormalCompileRequestV1::for_mir_mode_callable_source(
                        source,
                        None,
                        Default::default(),
                    ),
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok());
                        let physical = view.issue_lifecycle_physical_abi_input()?;
                        assert_eq!(
                            physical.runtime_requirements(),
                            PublishedLifecycleRuntimeRequirementsV1::NativeArray
                        );
                        let mut descriptor = decode_descriptor(&descriptor_bytes())?;
                        descriptor.target_triple = "x86_64-unknown-linux-gnu".into();
                        let session = LifecycleRuntimeSessionV1 {
                            runtime_archive: archive.to_owned(),
                            descriptor,
                        };
                        let bound = LifecycleInvocationInputV1::bind(physical.clone(), &session);
                        assert_eq!(bound.is_ok(), available);
                        if let Ok(bound) = bound {
                            assert_eq!(bound.runtime_archive(), archive);
                            let json: serde_json::Value =
                                serde_json::from_str(&bound.serialize()?).unwrap();
                            assert_eq!(json["runtime_requirements"]["kind"], "native_array");
                            assert_eq!(
                                json["functions"][0]["role"],
                                if terminal == "return" {
                                    "root_unit"
                                } else {
                                    "root_i64"
                                }
                            );
                            let mut wrong = session.clone();
                            wrong.descriptor.fault_abi_version += 1;
                            assert!(
                                LifecycleInvocationInputV1::bind(physical.clone(), &wrong).is_err()
                            );
                            wrong = session.clone();
                            wrong.descriptor.target_triple = "other".into();
                            assert!(LifecycleInvocationInputV1::bind(physical, &wrong).is_err());
                        }
                        Ok(())
                    },
                )
                .unwrap();
        }
    });
}
