use super::*;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn root(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "nyllvmc_static_artifact_{}_{}_{}",
        label,
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn write_candidate_json(path: &Path) -> StaticAotArtifactDescriptorV1 {
    let value = serde_json::json!({
        "kind": "MIR",
        "functions": [{
            "name": "main",
            "params": [],
            "metadata": {},
            "blocks": [{"id": 0, "instructions": [
                {"op": "ret", "value": null}
            ]}]
        }, {
            "name": "ParserScanLoopBox.skip_while/4",
            "params": [0, 1, 2, 3],
            "metadata": {
                "a_prime_i64_physical_receipt": {"schema_version": 1},
                "dynamic_v2_aot_call_admission_v2": {
                    "schema_version": 2,
                    "contract_id": "hako.text.scan@1",
                    "profile": 1,
                    "abi_revision": 1,
                    "wire_revision": 2,
                    "registry_generation": 7,
                    "return_type": "i64",
                    "return_lane": "immediate_i64",
                    "formal_parameters": [
                        {"role": "src", "value_id": 0, "lane": "opaque_handle"},
                        {"role": "pos", "value_id": 1, "lane": "immediate_i64"},
                        {"role": "end", "value_id": 2, "lane": "immediate_i64"},
                        {"role": "pred_chars", "value_id": 3, "lane": "opaque_handle"}
                    ],
                    "plan_stamp": {
                        "compiler_domain": 3,
                        "invocation_ordinal": 9
                    },
                    "calls": [
                        {
                            "role": "substring",
                            "site_id": 0,
                            "entry_id": 1,
                            "symbol": "hako.text.scan.substring.v1",
                            "abi_revision": 1,
                            "wire_revision": 2,
                            "receiver_lane": "opaque_handle",
                            "argument_lanes": ["immediate_i64", "immediate_i64"],
                            "result_lane": "opaque_handle",
                            "lease": "end_authorized",
                            "normal_shape": "end_authorized_handle",
                            "outcome_slot": 0,
                            "normal_result_dst": 20,
                            "effects": 16
                        },
                        {
                            "role": "index_of",
                            "site_id": 1,
                            "entry_id": 2,
                            "symbol": "hako.text.scan.index_of.v1",
                            "abi_revision": 1,
                            "wire_revision": 2,
                            "receiver_lane": "opaque_handle",
                            "argument_lanes": ["opaque_handle"],
                            "result_lane": "immediate_i64",
                            "lease": "none",
                            "normal_shape": "immediate_i64",
                            "outcome_slot": 1,
                            "normal_result_dst": 21,
                            "effects": 16
                        }
                    ]
                }
            },
            "blocks": [
                {"id": 0, "instructions": [
                    {"op": "checked_callout", "site_id": 0, "receiver": 0,
                     "args": [1, 2], "normal": 2, "fault": 3, "effects": 16}
                ]},
                {"id": 1, "instructions": [{"op": "jump", "target": 2}]},
                {"id": 2, "instructions": [
                    {"op": "checked_callout_normal_result", "site_id": 0, "dst": 20},
                    {"op": "checked_callout", "site_id": 1, "receiver": 3,
                     "args": [20], "normal": 4, "fault": 5, "effects": 16}
                ]},
                {"id": 3, "instructions": [
                    {"op": "checked_callout_fault", "site_id": 0}
                ]},
                {"id": 4, "instructions": [
                    {"op": "checked_callout_normal_result", "site_id": 1, "dst": 21},
                    {"op": "checked_callout_end", "site_id": 0, "lease_slot": 0},
                    {"op": "checked_callout_end", "site_id": 0, "lease_slot": 0},
                    {"op": "checked_callout_end", "site_id": 0, "lease_slot": 0},
                    {"op": "ret", "value": 21}
                ]},
                {"id": 5, "instructions": [
                    {"op": "checked_callout_fault", "site_id": 1}
                ]}
            ]
        }]
    });
    fs::write(path, serde_json::to_vec(&value).expect("json")).expect("write json");
    expected_descriptor_from_json(path)
        .expect("valid descriptor")
        .expect("selected descriptor")
}

fn descriptor_bytes(descriptor: &StaticAotArtifactDescriptorV1) -> [u8; 192] {
    let mut bytes = [0_u8; 192];
    bytes[..8].copy_from_slice(b"HAKODV2\0");
    put_u32(&mut bytes, 8, 1);
    put_u32(&mut bytes, 12, 192);
    put_u32(&mut bytes, 16, descriptor.profile);
    put_u32(&mut bytes, 20, descriptor.abi_revision);
    put_u32(&mut bytes, 24, descriptor.wire_revision);
    put_u32(&mut bytes, 28, 2);
    put_u64(&mut bytes, 32, descriptor.compiler_domain);
    put_u64(&mut bytes, 40, descriptor.invocation_ordinal);
    put_u64(&mut bytes, 48, descriptor.registry_generation);
    put_text(&mut bytes, 56, 32, &descriptor.contract_id);
    for (index, entry) in descriptor.entries.iter().enumerate() {
        let offset = 88 + index * 52;
        put_u32(&mut bytes, offset, entry.site_id);
        put_u32(&mut bytes, offset + 4, entry.entry_id);
        put_u32(&mut bytes, offset + 8, entry.logical_arity);
        put_text(&mut bytes, offset + 12, 40, &entry.symbol);
    }
    bytes
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn put_text(bytes: &mut [u8], offset: usize, capacity: usize, value: &str) {
    assert!(value.len() < capacity);
    bytes[offset..offset + value.len()].copy_from_slice(value.as_bytes());
}

fn write_object_source(path: &Path, descriptor: &[u8], corrupt: bool) {
    let mut bytes = descriptor.to_vec();
    if corrupt {
        bytes[20] ^= 1;
    }
    let initializer = bytes
        .iter()
        .map(|byte| format!("0x{byte:02x}"))
        .collect::<Vec<_>>()
        .join(",");
    let source = format!(
        r#"
#include <stdint.h>
extern void text_slice(void) __asm__("hako.text.scan.substring.v1");
extern void text_find(void) __asm__("hako.text.scan.index_of.v1");
extern void lease_end(void) __asm__("nyrt_dynamic_v2_lease_consume_end_authorized_v1");
__attribute__((used)) static void* required[] = {{
  (void*)&text_slice, (void*)&text_find, (void*)&lease_end
}};
__attribute__((used, section(".hako_dynamic_v2_descriptor")))
const unsigned char hako_dynamic_v2_static_artifact_descriptor_v1[192] = {{ {initializer} }};
int64_t ny_main(void) {{ return 0; }}
"#
    );
    fs::write(path, source).expect("write object source");
}

fn write_runtime_source(path: &Path, include_lease: bool) {
    let lease = if include_lease {
        "uint32_t nyrt_dynamic_v2_lease_consume_end_authorized_v1(uint64_t x) { return x == 0; }"
    } else {
        ""
    };
    let source = format!(
        r#"
#include <stdint.h>
uint32_t text_slice(void) __asm__("hako.text.scan.substring.v1");
uint32_t text_find(void) __asm__("hako.text.scan.index_of.v1");
uint32_t text_slice(void) {{ return 0; }}
uint32_t text_find(void) {{ return 0; }}
{lease}
extern int64_t ny_main(void);
int main(void) {{ return (int)ny_main(); }}
"#
    );
    fs::write(path, source).expect("write runtime source");
}

fn build_fixture(root: &Path, corrupt: bool, include_lease: bool) -> (PathBuf, PathBuf, PathBuf) {
    fs::create_dir_all(root).expect("fixture root");
    let json = root.join("candidate.json");
    let descriptor = write_candidate_json(&json);
    let object_source = root.join("candidate.c");
    let object = root.join("candidate.o");
    write_object_source(&object_source, &descriptor_bytes(&descriptor), corrupt);
    run(Command::new("cc")
        .arg("-c")
        .arg(&object_source)
        .arg("-o")
        .arg(&object));
    let runtime_source = root.join("runtime.c");
    let runtime_object = root.join("runtime.o");
    let archive = root.join("libnyash_kernel.a");
    write_runtime_source(&runtime_source, include_lease);
    run(Command::new("cc")
        .arg("-c")
        .arg(&runtime_source)
        .arg("-o")
        .arg(&runtime_object));
    run(Command::new("ar")
        .arg("rcs")
        .arg(&archive)
        .arg(&runtime_object));
    (json, object, archive)
}

fn run(command: &mut Command) {
    let output = command.output().expect("spawn fixture tool");
    assert!(
        output.status.success(),
        "fixture tool failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn system_linker(
    object: &Path,
    candidate: &Path,
    archive: &Path,
) -> Result<(), StaticArtifactRejectV1> {
    let output = Command::new("cc")
        .arg(object)
        .arg(archive)
        .arg("-o")
        .arg(candidate)
        .output()
        .map_err(|_| StaticArtifactRejectV1::LinkFailed)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(StaticArtifactRejectV1::LinkFailed)
    }
}

fn emit_boundary_object(ffi: PathBuf, json: PathBuf, object: PathBuf) -> Result<(), String> {
    std::thread::Builder::new()
        .name("boundary-artifact-e2e".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || unsafe {
            type CompileFn =
                unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> c_int;
            let library = libloading::Library::new(&ffi).map_err(|error| error.to_string())?;
            let compile: libloading::Symbol<CompileFn> = library
                .get(b"hako_llvmc_compile_json_pure_first\0")
                .map_err(|error| error.to_string())?;
            let input = CString::new(json.to_string_lossy().as_bytes())
                .map_err(|error| error.to_string())?;
            let output = CString::new(object.to_string_lossy().as_bytes())
                .map_err(|error| error.to_string())?;
            let mut error: *mut c_char = std::ptr::null_mut();
            let status = compile(input.as_ptr(), output.as_ptr(), &mut error);
            if status != 0 || !object.is_file() {
                return Err(format!("Boundary compile failed with status {status}"));
            }
            Ok(())
        })
        .map_err(|error| error.to_string())?
        .join()
        .map_err(|_| "Boundary compile thread panicked".to_string())?
}

#[test]
fn actual_artifacts_issue_one_receipt_before_consuming_commit() {
    let root = root("positive");
    let (json, object, archive) = build_fixture(&root, false, true);
    let final_path = root.join("program");
    let prepared = StaticAotArtifactPublicationTxnV1::prepare_with_linker(
        &json,
        &object,
        &final_path,
        &archive,
        system_linker,
    )
    .expect("prepare static artifact");
    assert!(!final_path.exists());
    assert_eq!(prepared.receipt().descriptor().entries.len(), 2);
    let (object_path, archive_path, candidate_path) = prepared.receipt().artifact_paths();
    assert_eq!(object_path, object);
    assert_eq!(archive_path, archive);
    assert!(candidate_path.is_file());
    assert_eq!(prepared.receipt().symbol_census(), (3, 3, 3, 3));
    assert_ne!(prepared.receipt().descriptor_digest(), &[0; 32]);
    let (object_digest, archive_digest, executable_digest) = prepared.receipt().artifact_digests();
    assert_ne!(object_digest, &[0; 32]);
    assert_ne!(archive_digest, &[0; 32]);
    assert_ne!(executable_digest, &[0; 32]);
    let receipt = prepared.commit().expect("atomic commit");
    assert_eq!(receipt.published_path(), final_path);
    assert_eq!(receipt.observed().final_path(), final_path);
    assert!(final_path.is_file());
    let transport_path = root.join("artifact-receipt.json");
    receipt
        .write_receipt_json(&json, &transport_path)
        .expect("write dedicated receipt");
    let transport: serde_json::Value =
        serde_json::from_slice(&fs::read(&transport_path).expect("receipt bytes"))
            .expect("receipt JSON");
    assert_eq!(transport["schema_version"], 1);
    assert_eq!(transport["status"], "published");
    assert_eq!(
        transport["descriptor"]["entries"].as_array().unwrap().len(),
        2
    );
    assert!(transport.get("candidate_path").is_none());
    assert!(Command::new(&final_path)
        .status()
        .expect("launch published artifact")
        .success());
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn rename_failure_keeps_candidate_and_final_path_unpublished() {
    let root = root("rename_failure");
    let (json, object, archive) = build_fixture(&root, false, true);
    let final_path = root.join("program");
    let prepared = StaticAotArtifactPublicationTxnV1::prepare_with_linker(
        &json,
        &object,
        &final_path,
        &archive,
        system_linker,
    )
    .expect("prepare static artifact");
    let candidate_path = prepared.receipt().artifact_paths().2.to_path_buf();
    fs::create_dir(&final_path).expect("reserve an invalid final path");
    let error = prepared.commit().expect_err("rename must reject");
    assert_eq!(error, StaticArtifactRejectV1::PublishFailed);
    assert!(!candidate_path.exists());
    assert!(final_path.is_dir());
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn boundary_generated_object_survives_exact_link_and_receipt_observation() {
    let root = root("boundary_e2e");
    let (json, handmade_object, archive) = build_fixture(&root, false, true);
    fs::remove_file(handmade_object).expect("remove handmade object");
    let object = root.join("boundary.o");
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    run(Command::new("bash")
        .arg(repository.join("tools/build_hako_llvmc_ffi.sh"))
        .current_dir(&repository));
    let ffi = repository.join("target/release/libhako_llvmc_ffi.so");
    emit_boundary_object(ffi, json.clone(), object.clone()).expect("Boundary object");
    let final_path = root.join("program");
    let prepared = StaticAotArtifactPublicationTxnV1::prepare_with_linker(
        &json,
        &object,
        &final_path,
        &archive,
        system_linker,
    )
    .expect("prepare Boundary artifact");
    assert_eq!(prepared.receipt().symbol_census(), (3, 3, 3, 3));
    assert!(!final_path.exists());
    let published = prepared.commit().expect("publish Boundary artifact");
    let receipt_path = root.join("boundary-receipt.json");
    published
        .write_receipt_json(&json, &receipt_path)
        .expect("write Boundary receipt");
    assert!(Command::new(&final_path)
        .status()
        .expect("launch Boundary artifact")
        .success());
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn non_entry_selected_metadata_is_rejected_before_artifact_observation() {
    let root = root("foreign_metadata");
    fs::create_dir_all(&root).expect("fixture root");
    let json = root.join("candidate.json");
    let selected = serde_json::json!({
        "name": "helper",
        "metadata": {
            "a_prime_i64_physical_receipt": {"schema_version": 1},
            "dynamic_v2_aot_call_admission_v2": {
            "contract_id": "hako.text.scan@1",
            "profile": 1,
            "abi_revision": 1,
            "wire_revision": 2,
            "registry_generation": 7,
            "plan_stamp": {"compiler_domain": 3, "invocation_ordinal": 9},
            "calls": []
        }},
        "blocks": [{"id": 0, "instructions": []}]
    });
    let value = serde_json::json!({
        "kind": "MIR",
        "functions": [
            {"name": "main", "blocks": [{"id": 0, "instructions": []}]},
            selected
        ]
    });
    fs::write(&json, serde_json::to_vec(&value).expect("json")).expect("write json");
    assert!(matches!(
        expected_descriptor_from_json(&json),
        Err(StaticArtifactRejectV1::ForeignDescriptor)
    ));
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn descriptor_drift_rejects_before_link_and_preserves_final() {
    let root = root("descriptor_drift");
    let (json, object, archive) = build_fixture(&root, true, true);
    let final_path = root.join("program");
    fs::write(&final_path, b"prior").expect("prior final");
    let result = StaticAotArtifactPublicationTxnV1::prepare_with_linker(
        &json,
        &object,
        &final_path,
        &archive,
        |_, _, _| panic!("descriptor drift must reject before link"),
    );
    assert!(matches!(
        result,
        Err(StaticArtifactRejectV1::DescriptorMismatch)
    ));
    assert_eq!(fs::read(&final_path).expect("prior final"), b"prior");
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn missing_archive_symbol_rejects_without_candidate_or_receipt() {
    let root = root("missing_symbol");
    let (json, object, archive) = build_fixture(&root, false, false);
    let final_path = root.join("program");
    let result = StaticAotArtifactPublicationTxnV1::prepare_with_linker(
        &json,
        &object,
        &final_path,
        &archive,
        |_, _, _| panic!("missing symbol must reject before link"),
    );
    assert!(matches!(result, Err(StaticArtifactRejectV1::MissingSymbol)));
    assert!(!final_path.exists());
    assert!(fs::read_dir(&root)
        .expect("fixture entries")
        .all(|entry| !entry
            .expect("entry")
            .file_name()
            .to_string_lossy()
            .contains(".tmp")));
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn duplicate_archive_symbol_rejects_before_link() {
    let root = root("duplicate_symbol");
    let (json, object, archive) = build_fixture(&root, false, true);
    let duplicate_source = root.join("runtime_duplicate.c");
    let duplicate_object = root.join("runtime_duplicate.o");
    write_runtime_source(&duplicate_source, true);
    run(Command::new("cc")
        .arg("-c")
        .arg(&duplicate_source)
        .arg("-o")
        .arg(&duplicate_object));
    run(Command::new("ar")
        .arg("q")
        .arg(&archive)
        .arg(&duplicate_object));
    run(Command::new("ar").arg("s").arg(&archive));
    let final_path = root.join("program");
    let result = StaticAotArtifactPublicationTxnV1::prepare_with_linker(
        &json,
        &object,
        &final_path,
        &archive,
        |_, _, _| panic!("duplicate symbol must reject before link"),
    );
    assert!(matches!(
        result,
        Err(StaticArtifactRejectV1::DuplicateSymbol)
    ));
    assert!(!final_path.exists());
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn link_failure_removes_candidate_and_preserves_final() {
    let root = root("link_failure");
    let (json, object, archive) = build_fixture(&root, false, true);
    let final_path = root.join("program");
    fs::write(&final_path, b"prior").expect("prior final");
    let result = StaticAotArtifactPublicationTxnV1::prepare_with_linker(
        &json,
        &object,
        &final_path,
        &archive,
        |_, candidate, _| {
            fs::write(candidate, b"partial").expect("partial candidate");
            Err(StaticArtifactRejectV1::LinkFailed)
        },
    );
    assert!(matches!(result, Err(StaticArtifactRejectV1::LinkFailed)));
    assert_eq!(fs::read(&final_path).expect("prior final"), b"prior");
    assert!(fs::read_dir(&root)
        .expect("fixture entries")
        .all(|entry| !entry
            .expect("entry")
            .file_name()
            .to_string_lossy()
            .contains(".tmp")));
    fs::remove_dir_all(root).expect("remove fixture");
}
