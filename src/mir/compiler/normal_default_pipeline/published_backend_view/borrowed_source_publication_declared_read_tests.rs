//! Explicit From read publication, use permissions and independent tamper checks.
use super::*;

#[test]
fn declared_read_result_keeps_local_support_through_field_publication() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let reborrow = "box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } } static box Main { main() { local p = new Leaf() local first = p.identity(p) local q = p.identity(first) return q.value } }";
        let nullable = "box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } maybe(n: i64, p: Leaf): Leaf from p { if n == 0 { return null } return p } } static box Main { main() { local p = new Leaf() local first = p.maybe(N, p) local q = p.identity(first) if q == null { return 0 } return q.value } }";
        let child = "box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } } box Parent { child: Leaf marker: i64 birth(child) { me.child = child me.marker = 0 } get(): Leaf from me { return me.child } } static box Main { main() { local owner = new Leaf() local child = new Leaf() local p = new Parent(child) local first = p.get() local q = owner.identity(first) return q.value } }";
        let receiver = "box Leaf { value: i64 birth() { me.value = 7 } identity(): Leaf from me { return me } } static box Main { main() { local p = new Leaf() local first = p.identity() local q = first.identity() return q.value } }";
        let child_receiver = "box Leaf { value: i64 birth() { me.value = 7 } identity(): Leaf from me { return me } } box Parent { child: Leaf birth(child) { me.child = child } get(): Leaf from me { return me.child } } static box Main { main() { local child = new Leaf() local p = new Parent(child) local first = p.get() local q = first.identity() return q.value } }";
        for (suffix, text) in [("receiver-reborrow-alias", receiver.replace("local q = first.identity()", "local alias = first local q = alias.identity()")), ("receiver-reborrow", receiver.to_owned()), ("receiver-reborrow-child", child_receiver.to_owned()), ("reborrow-read-after", reborrow.replace("return q.value", "return first.value")), ("reborrow-child", child.to_owned()), ("reborrow", reborrow.to_owned()), ("reborrow-null-0", nullable.replace("N,", "0,")), ("reborrow-null-1", nullable.replace("N,", "1,"))] {
        for optimize in [false, true] {
        MirCompiler::with_options(optimize).compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
            require_artifact_verification(verification);
            let input = view.issue_lifecycle_physical_abi_input()?;
            let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
            std::fs::write(std::env::temp_dir().join(format!("hako-issued-declared-read-{suffix}.json")), wire).unwrap();
            if !optimize && !suffix.starts_with("receiver-reborrow") {
                let mut changed = view.module().clone();
                let main = changed.functions.get_mut("main").unwrap();
                let origins = crate::mir::value_origin::AcyclicUnanimousValueOrigins::new(main);
                let mut replacements = 0;
                for row in main.blocks.values_mut().filter_map(|block| block.terminator.as_mut()) {
                    if let MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. } = row {
                        if let Some(receiver) = call.callee.receiver() {
                            if call.args.len() == 1 && origins.origin(call.args[0]) != origins.origin(receiver) {
                                call.args[0] = receiver; replacements += 1;
                            }
                        }
                    }
                }
                assert_eq!(replacements, 1);
                super::reject_changed_module(&changed, view.retained_handoff);
            }
            if suffix == "reborrow-read-after" {
                let mut changed = view.module().clone();
                let callee = changed.functions.get_mut("Leaf.identity/1").unwrap();
                let base = callee.params[1];
                callee.blocks.values_mut().next().unwrap().instructions.push(MirInstruction::FieldSet {
                    base, field: "value".into(), value: base, declared_type: None,
                });
                super::reject_changed_module(&changed, view.retained_handoff);
            }
            Ok(())
        }).unwrap_or_else(|issue| panic!("declared read {suffix} optimize={optimize}: {issue}"));
        }
        }
        for invalid in [receiver.replace("local q =", "p.value = 8 local q ="), receiver.replace("{ return me }", "{ me.value = 8 return me }"), reborrow.replace("return q.value", "p.value = 8 return first.value"), reborrow.replace("local q =", "p.value = 8 local q ="), reborrow.replace("{ return p }", "{ p.value = 8 return p }")] {
            let issue = MirCompiler::with_options(false).compile_normal_with_published(request(&invalid), |_, _| Ok(())).err().expect("invalid read reborrow");
            assert!(issue.to_string().contains("[freeze:contract]"), "{issue}");
        }
        for n in [0, 1] {
            let source = format!("box Leaf {{ value: i64 birth() {{ me.value = 7 }} maybe(n: i64, p: Leaf): Leaf from p {{ if n == 0 {{ return null }} return p }} }} static box Main {{ main() {{ local p = new Leaf() local q = p.maybe({n}, p) return 7 }} }}");
            MirCompiler::with_options(false).compile_normal_with_published(request(&source), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-declared-read-nullable-{n}.json")), wire).unwrap();
                Ok(())
            }).unwrap_or_else(|issue| panic!("nullable capture {n}: {issue}"));
            let guarded = source.replace("return 7", "if q == null { return 7 } return 9");
            MirCompiler::with_options(false).compile_normal_with_published(request(&guarded), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-declared-read-guarded-{n}.json")), wire).unwrap();
                Ok(())
            }).unwrap_or_else(|issue| panic!("nullable comparison {n}: {issue}"));
            let guarded_field = source.replace("return 7", "if q == null { return 0 } return q.value");
            for (suffix, guarded_field) in [("guarded-field", guarded_field.clone()),
                ("guarded-alias", guarded_field.replace("return q.value", "local r = q return r.value"))] {
            for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(request(&guarded_field), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-declared-read-{suffix}-{n}.json")), wire).unwrap();
                let mut changed = view.module().clone();
                let mut comparisons = 0;
                for block in changed.functions.get_mut("main").unwrap().blocks.values_mut() {
                    for row in &mut block.instructions {
                        if let MirInstruction::Compare { op, .. } = row {
                            *op = crate::mir::CompareOp::Ne;
                            comparisons += 1;
                        }
                    }
                }
                assert_eq!(comparisons, 1);
                super::reject_changed_module(&changed, view.retained_handoff);
                Ok(())
            }).unwrap_or_else(|issue| panic!("nullable field {suffix} {n} optimize={optimize}: {issue}"));
            }
            }
            for (label, tail) in [
                ("unguarded", "return q.value"),
                ("support-mutated", "if q == null { return 0 } p.value = 8 return q.value"),
                ("null-arm-rejoins", "if q == null { local ignored = 0 } return q.value"),
            ] {
                let unsafe_read = source.replace("return 7", tail);
                assert!(MirCompiler::with_options(false).compile_normal_with_published(request(&unsafe_read), |_, _| Ok(())).is_err(), "{label} must not gain nullable field permission");
            }
        }
        for (text, symbol, receiver, ordinal, artifact) in [
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } } static box Main { main() { local p = new Leaf() local q = p.identity(p) return q.value } }", "Leaf.identity/1", false, 0, "hako-issued-declared-read-local.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } } static box Main { main() { local p = new Leaf() local original = p.identity(p) local q = original return q.value } }", "Leaf.identity/1", false, 0, "hako-issued-declared-read-alias.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } forward(n: i64, p: Leaf): Leaf from p { local original = me.identity(p) local q = original return q } } static box Main { main() { local p = new Leaf() local q = p.forward(0, p) return q.value } }", "Leaf.forward/2", false, 1, "hako-issued-declared-read-forward.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(): Leaf from me { return me } } static box Main { main() { local p = new Leaf() local q = p.identity() return q.value } }", "Leaf.identity/0", true, 0, "hako-issued-declared-read-receiver.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(): Leaf from me { return me } forward(): Leaf from me { local original = me.identity() local q = original return q } } static box Main { main() { local p = new Leaf() local q = p.forward() return q.value } }", "Leaf.forward/0", true, 0, "hako-issued-declared-read-receiver-forward.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } } box Parent { child: Leaf marker: i64 birth(child) { me.child = child me.marker = 0 } identity(): Leaf from me { return me.child } } static box Main { main() { local child = new Leaf() local p = new Parent(child) local q = p.identity() return q.value } }", "Parent.identity/0", true, 0, "hako-issued-declared-read-field.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } } box Parent { child: Leaf marker: i64 birth(child) { me.child = child me.marker = 0 } get(): Leaf from me { return me.child } forward(): Leaf from me { local original = me.get() local q = original return q } } static box Main { main() { local child = new Leaf() local p = new Parent(child) local q = p.forward() return q.value } }", "Parent.forward/0", true, 0, "hako-issued-declared-read-field-forward.json"),
        ] {
        for optimize in [false, true].into_iter().filter(|optimize| !*optimize || artifact.ends_with("-forward.json")) {
        MirCompiler::with_options(optimize).compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
            require_artifact_verification(verification);
            if artifact.ends_with("-alias.json") {
                let mut changed = view.module().clone();
                let main = changed.functions.get_mut("main").unwrap();
                let base = main.blocks.values().find_map(|block| match &block.terminator {
                    Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::FieldGet { base, .. }, .. }) => Some(*base),
                    _ => None,
                }).expect("checked alias field read");
                let allocation = main.blocks.iter().find_map(|(id, block)| matches!(&block.terminator,
                    Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::NewBox { .. }, .. })).then_some(*id)).unwrap();
                let live = main.blocks.values().flat_map(|block| &block.instructions).find_map(|row| match row {
                    MirInstruction::InvokeNormalResult { dst, invoke_block } if *invoke_block == allocation => Some(*dst),
                    _ => None,
                }).unwrap();
                let mut replacements = 0;
                for row in main.blocks.values_mut().flat_map(|block| &mut block.instructions) {
                    if let MirInstruction::Copy { dst, src } = row {
                        if *dst == base { assert_ne!(*src, live); *src = live; replacements += 1; }
                    }
                }
                assert_eq!(replacements, 1);
                super::reject_changed_module(&changed, view.retained_handoff);
            }
            let input = view.issue_lifecycle_physical_abi_input()?;
            let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
            let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
            let functions = json["functions"].as_array().unwrap();
            let (target, callee) = functions.iter().enumerate().find(|(_, function)| function["name"] == symbol).unwrap();
            assert_eq!(callee["role"], "ordinary_borrowed_nullable_handle");
            assert_eq!(callee["result_read"], true);
            if receiver {
                assert_eq!(callee["result_support"], serde_json::json!({"kind":"receiver"}));
                assert!(callee.get("result_formal_ordinal").is_none());
            } else { assert_eq!(callee["result_formal_ordinal"], ordinal); }
            let calls: Vec<_> = functions.iter().flat_map(|function| function["blocks"].as_array().unwrap())
                .map(|block| &block["terminator"]["instruction"]["operation"])
                .filter(|op| op["kind"] == "ordinary_call" && op["call"]["target"] == target).collect();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0]["result_read"], true);
            assert_eq!(calls[0]["result"], "borrowed_nullable_handle");
            // Erasing both qualifiers remains internally consistent, but must
            // disagree with the ORIGINAL From declaration retained by input.
            let original_call = input.program().functions().iter().filter(|f| f.name() == "main").flat_map(|f| f.blocks())
                .map(|block| block.terminator().instruction()).find(|row| matches!(row,
                    crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call {
                        result: crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle {
                            access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly, .. }, .. }, .. })).unwrap();
            let mut erased_call = original_call.clone();
            if let crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call {
                result: crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle { access, .. }, .. }, .. } = &mut erased_call {
                *access = crate::mir::instruction::BorrowedResultAccessV1::LegacyUnqualified;
            }
            let mut changed = input.program().clone();
            for function in changed.functions.iter_mut() {
                if let PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryBorrowedNullableHandle { access, read_result, .. } = &mut function.role {
                    *access = crate::mir::instruction::BorrowedResultAccessV1::LegacyUnqualified;
                    *read_result = None;
                }
                for block in function.blocks.iter_mut() {
                    if std::ptr::eq(block.terminator.instruction(), original_call) { block.terminator.instruction = &erased_call; }
                }
            }
            let erased_result = match &erased_call {
                crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { result, .. }, .. } => *result,
                _ => unreachable!(),
            };
            let (ordinal, callee) = changed.functions.iter().enumerate().find(|(_, function)| function.name() == symbol).unwrap();
            assert_eq!(callee.role().ordinary_result(), Some(erased_result), "both downgraded descriptors deliberately agree");
            assert!(super::super::super::physical_program_json::corroborate_borrowed_function_snapshot(
                callee, ordinal as u32, &input).unwrap_err().contains("borrowed-carrier-function-drift"));
            let issue = super::super::super::physical_program_json::emit_lifecycle_physical_program_value(
                &changed, Some(&input)).unwrap_err();
            assert!(issue.contains("borrowed-carrier-projection-drift"), "{issue}");
            std::fs::write(std::env::temp_dir().join(artifact), wire).unwrap();
            Ok(())
        }).unwrap_or_else(|issue| panic!("{symbol} optimize={optimize}: {issue}"));
        }
        let forwarding = match artifact {
            "hako-issued-declared-read-forward.json" => Some(("local original = me.identity(p) local q = original return q", "local original = me.identity(p) local q = original p.value = 9 return q", "forward(n: i64, p: Leaf): Leaf from p", "forward(n: i64, p: Leaf)")),
            "hako-issued-declared-read-receiver-forward.json" => Some(("local original = me.identity() local q = original return q", "local original = me.identity() local q = original me.value = 9 return q", "forward(): Leaf from me", "forward()")),
            "hako-issued-declared-read-field-forward.json" => Some(("local original = me.get() local q = original return q", "local original = me.get() local q = original me.marker = 9 return q", "forward(): Leaf from me", "forward()")),
            _ => None,
        };
        if let Some((body, mutated, declaration, unqualified)) = forwarding {
            for source in [text.replace(body, mutated), text.replace(declaration, unqualified)] {
                assert!(MirCompiler::with_options(false).compile_normal_with_published(request(&source), |_, _| Ok(())).is_err(), "invalid forwarding source accepted: {source}");
            }
        }
        for (label, tail, refusal) in [
            ("write-result", "q.value = 9 return 0", "[freeze:contract]"),
            ("write-support-before-read", "p.value = 9 return q.value", "[freeze:contract]"),
            ("move-support-before-read", "local moved = move p return q.value", "[freeze:contract]"),
            ("move-read-result", "local moved = move q return 0", "MoveUnsupported"),
        ] {
            let tail = if symbol.starts_with("Parent.") && label == "write-support-before-read" { "p.marker = 9 return q.value" } else { tail };
            let source = text.replace("return q.value", tail);
            let issue = MirCompiler::with_options(false).compile_normal_with_published(request(&source),
                |_, _| Ok(())).err().expect(label);
            assert!(issue.to_string().contains(refusal), "{label}: {issue}");
        }
        }
    });
}
