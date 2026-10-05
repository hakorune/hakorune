//! Composed I64 results keep the original caller/callee and cleanup authority.
use super::*;

#[test]
fn borrowed_local_call_results_publish_grounded_chain_in_both_orders() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for reverse in [false, true] {
            for annotated in [false, true] {
                let result = if annotated { ": i64" } else { "" };
                let read = "read(p: Item): i64 { if p == null { return 7 } return p.value }";
                let bridge = format!("bridge(p: Item){result} {{ local recv = new Transport() local out = recv.read(p) return out }}");
                let wrap = format!("wrap(p: Item){result} {{ local recv = new Transport() local out = recv.bridge(p) return out }}");
                let methods = if reverse {
                    format!("{wrap} {bridge} {read}")
                } else {
                    format!("{read} {bridge} {wrap}")
                };
                for (domain, prefix, actual) in [
                    ("object", "local item = new Item()", "item"),
                    ("null", "", "null"),
                ] {
                    let text = format!("box Item {{ value: i64 birth() {{ me.value = 5 }} }}
                        box Transport {{ flag: i64 birth() {{ me.flag = 0 }} {methods} }}
                        static box Main {{ main() {{ local recv = new Transport() {prefix} return recv.wrap({actual}) }} }}");
                    for optimize in [false, true] {
                        MirCompiler::with_options(optimize).compile_normal_with_published(
                            request(&text), |view, verification| -> Result<(), String> {
                                classify_pretransform_report(verification);
                                let input = view.issue_lifecycle_physical_abi_input()?;
                                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                                for target in ["Transport.bridge/1", "Transport.wrap/1"] {
                                    let function = json["functions"].as_array().unwrap().iter()
                                        .find(|function| function["name"] == target).unwrap();
                                    assert_eq!(function["role"], "ordinary_i64");
                                }
                                std::fs::write(std::env::temp_dir().join(format!(
                                    "hako-issued-borrowed-call-result-{domain}-reverse{reverse}-annotated{annotated}-opt{optimize}.json"
                                )), wire).unwrap();
                                Ok(())
                            },
                        ).unwrap_or_else(|error| panic!("{domain}/reverse{reverse}/annotated{annotated}/opt{optimize}: {error}"));
                    }
                }
            }
        }
    });
}
