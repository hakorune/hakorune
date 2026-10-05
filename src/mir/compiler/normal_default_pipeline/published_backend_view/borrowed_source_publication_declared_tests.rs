//! Declared/forwarded domains survive the real source-to-physical pipeline.
use super::*;

#[test]
fn declared_and_forwarding_only_domains_publish_exact_class_layouts() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for shape in [
            "declared",
            "declared-copy",
            "opaque-forward",
            "opaque-copy",
            "ignored",
        ] {
            for (domain, prefix, actual) in [
                ("null", "", "null"),
                ("object", "local item = new Item()", "item"),
            ] {
                let opaque = shape.starts_with("opaque-");
                let prefix = if opaque && domain == "null" {
                    "local item = new Item() local witness = recv.bridge(item)"
                } else {
                    prefix
                };
                let parameter = if opaque { "p" } else { "p: Item" };
                let body = match shape {
                    "ignored" => "return 11",
                    "declared-copy" | "opaque-copy" => "local alias = p local recv = new Transport() local out = recv.read(alias) return 11",
                    _ => "local recv = new Transport() local out = recv.read(p) return 11",
                };
                let reader = if shape == "ignored" {
                    ""
                } else {
                    "read(p): i64 { if p == null { return 7 } if p.value > 0 { return 5 } return 0 }"
                };
                let text = format!(
                    "box Item {{ value: i64 birth() {{ me.value = 5 }} }}
                     box Transport {{ flag: i64 birth() {{ me.flag = 0 }}
                         {reader}
                         bridge({parameter}): i64 {{ {body} }} }}
                     static box Main {{ main() {{ local recv = new Transport()
                         {prefix} return recv.bridge({actual}) }} }}"
                );
                for optimize in [false, true] {
                    MirCompiler::with_options(optimize).compile_normal_with_published(
                        request(&text), |view, verification| -> Result<(), String> {
                            classify_pretransform_report(verification);
                            // Resolve the expected fixture declaration from the original
                            // canonical table, never from the transported view itself.
                            let expected_object = view.module().canonical_object_definitions()
                                .expect("source canonical objects").iter()
                                .position(|definition| definition.diagnostic_name() == "Item")
                                .expect("original Item definition") as u64;
                            let input = view.issue_lifecycle_physical_abi_input()?;
                            let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                            let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                            let bridge = json["functions"].as_array().unwrap().iter()
                                .find(|f| f["name"] == "Transport.bridge/1").unwrap();
                            let param = &bridge["params"][0];
                            assert_eq!(param["representation"], "borrowed_kind_payload_v1");
                            let object = param["object_view"].as_u64().expect("class domain survives ignored/forwarded root");
                            assert_eq!(object, expected_object, "transport must preserve the original class, including null-only/ignored roots");
                            reject_erased_carriers(&input);
                            reject_changed_producers(&input);
                            assert!(json["layouts"].as_array().unwrap().iter()
                                .any(|layout| layout["object_id"] == object), "null-only class requires its canonical layout");
                            assert_eq!(param.as_object().unwrap().len(), 3);
                            if !opaque {
                                let function = input.program().functions().iter()
                                    .find(|f| f.name() == "Transport.bridge/1").unwrap();
                                assert_eq!(function.param_types()[1], crate::mir::MirType::Integer);
                            }
                            if shape != "ignored" {
                                let reader = json["functions"].as_array().unwrap().iter()
                                    .find(|f| f["name"] == "Transport.read/1").unwrap();
                                assert_eq!(reader["params"][0]["object_view"], object);
                            }
                            if shape == "ignored" && domain == "null" {
                                for block in json["functions"].as_array().unwrap().iter()
                                    .flat_map(|f| f["blocks"].as_array().unwrap()) {
                                    let operation = &block["terminator"]["instruction"]["operation"];
                                    assert!(operation["kind"] != "new_box" || operation["object_id"] != object);
                                    for row in block["instructions"].as_array().unwrap() {
                                        assert!(row["instruction"]["op"] != "object_field_get" || row["instruction"]["object_id"] != object);
                                    }
                                }
                            }
                            std::fs::write(std::env::temp_dir().join(format!(
                                "hako-issued-declared-borrow-{shape}-{domain}-opt{optimize}.json"
                            )), wire).unwrap();
                            Ok(())
                        },
                    ).unwrap_or_else(|error| panic!("{shape}/{domain}/opt{optimize}: {error}"));
                }
            }
        }
    });
}
