//! PARAMFIELD-ACCEPTANCE-R0 emit: the selected frontier shapes publish
//! their source-issued physical input unchanged for the consolidated
//! C/OBJ/EXE acceptance run — the unused formal/I64 result, the local-new
//! TypedHome call and the exact null guard with a literal-null actual;
//! the sibling slice witnesses emit the guarded-field and nullable-actual
//! inputs.
use super::*;

/// Each selected source shape publishes through its existing owner and
/// writes exactly the JSON the V4 driver consumes — no receipt is
/// injected as source authority and the tagged carrier/transport kinds
/// are pinned per case.
#[test]
fn param_field_acceptance_shapes_publish_their_issued_inputs() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, text, callee, arg_kind) in [
            (
                "unused-i64",
                "box Store { limit: i64 birth() { me.limit = 10 } \
                release(handle) { return 0 } } \
                static box Main { main() { local s = new Store() return s.release(5) } }",
                "Store.release/1",
                1_u64,
            ),
            (
                "localnew",
                "box Item { id: i64 serial: i64 birth(id, serial) { me.id = id me.serial = serial } } \
                box Store { limit: i64 birth() { me.limit = 10 } \
                release(handle) { return 0 } } \
                static box Main { main() { local s = new Store() \
                local h = new Item(1, 2) local r = s.release(h) return r } }",
                "Store.release/1",
                3_u64,
            ),
            (
                "null-guard",
                "box Store { limit: i64 birth() { me.limit = 10 } \
                check(handle) { if handle == null { return 7 } return 3 } } \
                static box Main { main() { local s = new Store() return s.check(null) } }",
                "Store.check/1",
                0_u64,
            ),
        ] {
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire =
                            super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(
                                &input,
                            )?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let function = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|f| f["name"] == callee)
                            .unwrap_or_else(|| panic!("{suffix}: {callee} published"));
                        assert!(
                            function["params"].as_array().unwrap().iter().any(|p| {
                                p["representation"] == "borrowed_kind_payload_v1"
                            }),
                            "{suffix}: opaque formal keeps the borrowed carrier: {wire}"
                        );
                        let caller = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|f| f["name"] == "main")
                            .expect("caller row");
                        let instructions: Vec<&serde_json::Value> = caller["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|block| {
                                block["instructions"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .chain(std::iter::once(
                                        &block["terminator"]["instruction"],
                                    ))
                            })
                            .map(|row| row.get("instruction").unwrap_or(row))
                            .collect();
                        let calls: Vec<&serde_json::Value> = instructions
                            .iter()
                            .filter(|row| row["operation"]["kind"] == "ordinary_call")
                            .copied()
                            .collect();
                        assert!(
                            calls.iter().any(|call| {
                                call["operation"]["call"]["args"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .any(|arg| arg["kind"] == arg_kind)
                            }),
                            "{suffix}: borrowed actual spells payload tag {arg_kind}: {wire}"
                        );
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-param-acceptance-{suffix}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{suffix}: {error}"));
        }
    });
}
