//! Actual private C query -> MIR binding, retaining the C invocation until reply.
//! Retire this subprocess bridge when the public V2 host tests cover the boundary.
use super::*;
use std::ffi::CStr;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

struct LiveQuery(Child);
impl Drop for LiveQuery {
    fn drop(&mut self) {
        drop(self.0.stdin.take());
        let _ = self.0.wait();
    }
}

#[test]
#[ignore = "requires HAKO_NAMED_QUERY_TEST_DRIVER built with ASan and function instrumentation"]
fn map_literal_actual_c_query_binds_before_same_invocation_compile_or_cancel() {
    let driver = std::env::var("HAKO_NAMED_QUERY_TEST_DRIVER").expect("private C driver path");
    for (map_only, planner_reject) in [(false, false), (true, false), (true, true)] {
        let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "anchor", 0);
        let name = key.mir_symbol_projection();
        let mut anchor = function(&name, 0);
        constant(&mut anchor, 7, ConstValue::Integer(30));
        anchor
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Return {
                value: Some(ValueId::new(7)),
            });
        let mut root = function("main", 0);
        constant(
            &mut root,
            7,
            if map_only {
                ConstValue::Float(-0.0)
            } else {
                ConstValue::Integer(30)
            },
        );
        add(
            &mut root,
            MirInstruction::call(
                None,
                Callee::Global(key.canonical_global_target_v1().unwrap()),
                vec![],
                EffectMask::PURE,
            ),
        );
        alias(&mut root, 8, 7, "StringBox");
        if map_only {
            write(&mut root, 8);
        }
        root.blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Return {
                value: if map_only {
                    None
                } else {
                    Some(ValueId::new(8))
                },
            });
        let mut module = module(root);
        module.add_cataloged_box_method(key, anchor).unwrap();
        crate::mir::global_call_route_plan::refresh_module_global_call_routes(&mut module);
        crate::mir::same_module_definition_plan::refresh_module_same_module_definition_plans(
            &mut module,
        );
        assert!(!module
            .get_function("main")
            .unwrap()
            .metadata
            .same_module_definition_plans
            .is_empty());
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let frame = crate::mir::function::PublishedStaticMethodCFrameV1::from_view(&view).unwrap();
        let row = &frame.as_slice()[0];
        let request = serde_json::json!({
            "queries": [{"function":"main", "block":0, "instruction":2}],
            "call": {"function":unsafe { CStr::from_ptr(row.function_name).to_str().unwrap() },
                "target":unsafe { CStr::from_ptr(row.target_symbol).to_str().unwrap() },
                "instruction":row.instruction_index, "kind":row.kind}
        });
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("input.json");
        let object = dir.path().join("output.o");
        let body = if map_only {
            // Production is deliberately closed until V2; this existing test-only
            // exporter borrows the exact candidate without clone/refresh.
            assert!(crate::runner::mir_json_emit::emit_published_view_body(&view).is_err());
            crate::runner::mir_json_emit::emit_mir_json_string_for_unpublished_candidate(
                view.module(),
            )
            .unwrap()
        } else {
            crate::runner::mir_json_emit::emit_published_view_body(&view).unwrap()
        };
        std::fs::write(&input, body).unwrap();
        let mut process = LiveQuery(
            Command::new(&driver)
                .arg(&input)
                .arg(&object)
                .arg(request.to_string())
                .env("ASAN_OPTIONS", "detect_leaks=0")
                .env("HAKO_BACKEND_COMPILE_RECIPE", "pure-first")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
        );
        let mut output = BufReader::new(process.0.stdout.take().unwrap());
        let mut line = String::new();
        output.read_line(&mut line).unwrap();
        assert!(!line.is_empty(), "C query exited before responding");
        let response: serde_json::Value = serde_json::from_str(&line).unwrap();
        assert_eq!(response[0]["status"], 0);
        // Test-protocol decoding, not a production query ABI or name classifier.
        let consumer = match response[0]["consumer"].as_u64().unwrap() {
            0 => NamedAllocationConsumer::Array,
            1 => NamedAllocationConsumer::DirectArray,
            2 => NamedAllocationConsumer::Map,
            3 => NamedAllocationConsumer::File,
            4 => NamedAllocationConsumer::AliasOperandZero,
            5 => NamedAllocationConsumer::TypedObject,
            6 => NamedAllocationConsumer::InvalidPlan,
            7 => NamedAllocationConsumer::Unsupported,
            value => panic!("unknown private C outcome {value}"),
        };
        if planner_reject {
            assert!(MapBodyIndex::from_view(&view)
                .unwrap()
                .with_named_allocations([(("main", 0, 0), consumer)])
                .is_err());
        }
        if !planner_reject {
            let index = MapBodyIndex::from_view(&view)
                .unwrap()
                .with_named_allocations([(("main", 0, 2), consumer)])
                .unwrap();
            assert_eq!(
                index
                    .named_projection_action(("main", ValueId::new(8)))
                    .unwrap(),
                ProjectionAction::NamedAliasOperandZero
            );
            if map_only {
                assert_eq!(
                    index.map_value_domains().unwrap()[&("main", ValueId::new(8))],
                    BTreeSet::from([ValueDomain::F64])
                );
                let actions = BTreeMap::from([
                    (
                        ("main", ValueId::new(7)),
                        ProjectionAction::ExactF64((-0.0f64).to_bits()),
                    ),
                    (
                        ("main", ValueId::new(8)),
                        ProjectionAction::NamedAliasOperandZero,
                    ),
                ]);
                assert!(!index
                    .original_value_demands(&actions)
                    .unwrap()
                    .contains(&("main", ValueId::new(7))));
            }
        }
        // Remove the transport file after querying: retained compile must not reparse it.
        std::fs::remove_file(&input).unwrap();
        process
            .0
            .stdin
            .as_mut()
            .unwrap()
            .write_all(if map_only { b"x\n" } else { b"c\n" })
            .unwrap();
        drop(process.0.stdin.take());
        let mut tail = String::new();
        output.read_to_string(&mut tail).unwrap();
        let status = process.0.wait().unwrap();
        let mut error = String::new();
        process
            .0
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut error)
            .unwrap();
        assert!(status.success(), "{status}: {error}");
        assert!(
            tail.contains("rc=0 parsed=1 freed=1 program_reads=1 selections=2"),
            "{tail}: {error}"
        );
        assert_eq!(object.exists(), !map_only);
        if !map_only {
            let source = dir.path().join("main.c");
            let exe = dir.path().join("run");
            std::fs::write(
                &source,
                "extern long long ny_main(void); int main(void){return (int)ny_main();}\n",
            )
            .unwrap();
            assert!(Command::new("cc")
                .arg("-no-pie")
                .arg(&source)
                .arg(&object)
                .arg("-o")
                .arg(&exe)
                .status()
                .unwrap()
                .success());
            assert_eq!(Command::new(&exe).status().unwrap().code(), Some(30));
        }
    }
}
