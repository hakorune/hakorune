use super::*;
use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
enum CorpusClassV0 {
    ReadyExactParity,
    ExplicitUnsupported,
}

#[derive(Deserialize)]
struct CorpusCaseV0 {
    id: String,
    source: String,
    class: CorpusClassV0,
}

#[derive(Deserialize)]
struct CorpusManifestV0 {
    schema_version: u32,
    cases: Vec<CorpusCaseV0>,
}

fn manifest() -> CorpusManifestV0 {
    serde_json::from_str(include_str!(
        "../../../../../../tools/checks/fixtures/bounded_body_snapshot_program_v0_corpus_v0.json"
    ))
    .expect("bounded body snapshot corpus manifest")
}

fn current_program_v0_corpus_impl() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    let mut ready = 0;
    let mut unsupported = 0;
    let manifest = manifest();
    assert_eq!(manifest.schema_version, 0);
    assert_eq!(manifest.cases.len(), 7);
    for case in manifest.cases {
        let input =
            crate::stage1::program_json_v0::emit_program_json_v0_for_strict_authority_source(
                &case.source,
            )
            .unwrap_or_else(|error| panic!("{} serializer failed: {error}", case.id));
        match case.class {
            CorpusClassV0::ReadyExactParity => {
                let expected = snapshot_signature(&rust_snapshot(&input));
                assert_eq!(
                    run(
                        &mut interpreter,
                        &module,
                        &input,
                        "SnapshotDirectReaderFixtureV0Box.snapshot_signature/2",
                    ),
                    VMValue::String(expected),
                    "case={}",
                    case.id,
                );
                ready += 1;
            }
            CorpusClassV0::ExplicitUnsupported => {
                let expected = rust_outcome(&input);
                assert!(
                    expected.starts_with("Unsupported|"),
                    "case={}: {expected}",
                    case.id
                );
                assert_eq!(
                    run(
                        &mut interpreter,
                        &module,
                        &input,
                        "SnapshotDirectReaderFixtureV0Box.outcome/2",
                    ),
                    VMValue::String(expected),
                    "case={}",
                    case.id,
                );
                unsupported += 1;
            }
        }
        assert!(!interpreter.strict_json_session_active());
    }
    assert_eq!((ready, unsupported), (3, 4));
}

#[test]
fn current_authoritative_program_v0_corpus_has_no_skip_or_nomatch() {
    std::thread::Builder::new()
        .name("current-program-v0-corpus".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(current_program_v0_corpus_impl)
        .expect("spawn current ProgramV0 corpus thread")
        .join()
        .expect("current ProgramV0 corpus thread");
}
