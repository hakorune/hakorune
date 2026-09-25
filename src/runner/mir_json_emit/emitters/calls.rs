use serde_json::json;

use crate::mir::definitions::Callee;
use crate::mir::{EffectMask, ValueId};

use super::super::helpers::emit_unified_mir_call;
use super::super::root::JsonEgressProfile;
use super::calls_compat_v0::emit_call_compat_v0;

/// Canonical-v1 call writer plus profile dispatch. The v0
/// `"boxcall"`/`"externcall"` compatibility projection is owned by
/// `calls_compat_v0` (R6-S0 boundary S0-B).
pub(crate) fn emit_call(
    dst: &Option<ValueId>,
    func: &ValueId,
    callee: Option<&Callee>,
    args: &[ValueId],
    effects: &EffectMask,
    profile: JsonEgressProfile,
) -> Option<serde_json::Value> {
    if profile.is_canonical_v1() {
        let callee = callee?;
        let effects_str: Vec<&str> = if effects.is_io() { vec!["IO"] } else { vec![] };
        let args_u32: Vec<u32> = args.iter().map(|v| v.as_u32()).collect();
        return Some(emit_unified_mir_call(
            dst.map(|v| v.as_u32()),
            callee,
            &args_u32,
            &effects_str,
        ));
    }

    emit_call_compat_v0(dst, func, callee, args, effects, profile)
}

pub(crate) fn emit_new_box(dst: &ValueId, box_type: &str, args: &[ValueId]) -> serde_json::Value {
    let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
    json!({"op":"newbox","type": box_type, "args": args_a, "dst": dst.as_u32()})
}

pub(crate) fn emit_new_closure(
    dst: &ValueId,
    params: &[String],
    body_id: &Option<crate::mir::function::ClosureBodyId>,
    body: &[crate::ast::ASTNode],
    captures: &[(String, ValueId)],
    me: &Option<ValueId>,
) -> Result<serde_json::Value, String> {
    if body_id.is_some() || !body.is_empty() {
        return Err("[freeze:contract][mir-json/closure-body-wire-unavailable]".to_string());
    }

    // NewClosure is already canonicalized callsite shape.
    // Emit the empty descriptor only; a body-backed closure must not be
    // silently projected without its module metadata relation.
    let callee = Callee::Closure {
        params: params.to_vec(),
        captures: captures.to_vec(),
        me_capture: *me,
    };
    Ok(emit_unified_mir_call(Some(dst.as_u32()), &callee, &[], &[]))
}

#[cfg(test)]
mod tests {
    use super::{emit_call, emit_new_closure};
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::definitions::Callee;
    use crate::mir::{EffectMask, ValueId};
    use crate::runner::mir_json_emit::root::JsonEgressProfile;

    fn compatibility_v0(methodize: bool) -> JsonEgressProfile {
        JsonEgressProfile::CompatibilityV0 { methodize }
    }

    #[test]
    fn v0_typed_global_call_ignores_stale_numeric_func_decoration() {
        let v = emit_call(
            &Some(ValueId::new(3)),
            &ValueId::new(99),
            Some(&Callee::Global(crate::mir::test_global_target(
                "my_func/0".to_string(),
            ))),
            &[ValueId::new(1)],
            &EffectMask::PURE,
            compatibility_v0(false),
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("call"));
        assert!(
            v.get("func").is_none(),
            "typed func decoration must be omitted"
        );
        let callee = v.get("callee").expect("callee must exist");
        assert_eq!(callee.get("type").and_then(|x| x.as_str()), Some("Global"));
        assert_eq!(
            callee.get("name").and_then(|x| x.as_str()),
            Some("my_func/0")
        );
    }

    #[test]
    fn v0_typed_call_variants_ignore_stale_numeric_func_decoration() {
        let typed = vec![
            Callee::Global(crate::mir::test_global_target("worker/0".to_string())),
            Callee::Constructor {
                box_type: "WorkerBox".to_string(),
            },
            Callee::Value(ValueId::new(7)),
            Callee::Closure {
                params: vec!["value".to_string()],
                captures: vec![("captured".to_string(), ValueId::new(8))],
                me_capture: Some(ValueId::new(9)),
            },
        ];

        for callee in typed {
            let v = emit_call(
                &Some(ValueId::new(3)),
                &ValueId::new(99),
                Some(&callee),
                &[ValueId::new(1)],
                &EffectMask::PURE,
                compatibility_v0(false),
            )
            .expect("must emit typed call");

            assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("call"));
            assert!(
                v.get("func").is_none(),
                "typed v0 call must not copy stale numeric func: {v}"
            );
            assert!(v.get("callee").is_some(), "typed callee must be emitted");
        }

        let extern_call = emit_call(
            &None,
            &ValueId::new(99),
            Some(&Callee::Extern("env.worker.run".to_string())),
            &[ValueId::new(1)],
            &EffectMask::IO,
            compatibility_v0(false),
        )
        .expect("must emit extern call");
        assert_eq!(
            extern_call.get("op").and_then(|x| x.as_str()),
            Some("externcall")
        );
        assert!(extern_call["func"].is_string());
    }

    #[test]
    fn v0_legacy_call_preserves_explicit_numeric_func_decoration() {
        let v = emit_call(
            &None,
            &ValueId::new(99),
            None,
            &[ValueId::new(1)],
            &EffectMask::PURE,
            compatibility_v0(false),
        )
        .expect("must emit legacy call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("call"));
        assert_eq!(v.get("func").and_then(|x| x.as_u64()), Some(99));
        assert!(v.get("callee").is_none());
    }

    #[test]
    fn v0_print_global_maps_to_externcall() {
        let v = emit_call(
            &None,
            &ValueId::INVALID,
            Some(&Callee::Global(crate::mir::test_global_target(
                "print".to_string(),
            ))),
            &[ValueId::new(7)],
            &EffectMask::IO,
            compatibility_v0(false),
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("externcall"));
        assert_eq!(
            v.get("func").and_then(|x| x.as_str()),
            Some("nyash.console.log")
        );
    }

    #[test]
    fn compatibility_profile_methodize_projects_method_as_mir_call() {
        let v = emit_call(
            &Some(ValueId::new(9)),
            &ValueId::INVALID,
            Some(&Callee::Method {
                box_name: "FileBox".to_string(),
                method: "open".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            &[ValueId::new(2), ValueId::new(3)],
            &EffectMask::IO,
            compatibility_v0(true),
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("mir_call"));
        assert_eq!(
            v["mir_call"]["callee"]["type"].as_str(),
            Some("Method"),
            "methodized Stage1 route must stay on mir_call"
        );
    }

    #[test]
    fn canonical_profile_defaults_to_mir_call_for_method() {
        let v = emit_call(
            &Some(ValueId::new(9)),
            &ValueId::INVALID,
            Some(&Callee::Method {
                box_name: "FileBox".to_string(),
                method: "open".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            &[ValueId::new(2), ValueId::new(3)],
            &EffectMask::IO,
            JsonEgressProfile::CanonicalV1,
        )
        .expect("must emit call");

        assert_eq!(
            v.get("op").and_then(|x| x.as_str()),
            Some("mir_call"),
            "Stage1 mainline defaults must stay on canonical mir_call"
        );
    }

    #[test]
    fn compatibility_profile_without_methodize_keeps_boxcall() {
        let v = emit_call(
            &Some(ValueId::new(9)),
            &ValueId::INVALID,
            Some(&Callee::Method {
                box_name: "FileBox".to_string(),
                method: "open".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            &[ValueId::new(2), ValueId::new(3)],
            &EffectMask::IO,
            compatibility_v0(false),
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("boxcall"));
    }

    #[test]
    fn method_none_keeps_legacy_receiver_func_until_r6() {
        let v = emit_call(
            &None,
            &ValueId::new(12),
            Some(&Callee::Method {
                box_name: "FileBox".to_string(),
                method: "open".to_string(),
                receiver: None,
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            &[ValueId::new(2)],
            &EffectMask::IO,
            compatibility_v0(false),
        )
        .expect("must emit compatibility method call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("boxcall"));
        assert_eq!(v.get("box").and_then(|x| x.as_u64()), Some(12));
    }

    #[test]
    fn empty_new_closure_keeps_descriptor_projection() {
        let value = emit_new_closure(&ValueId::new(3), &[], &None, &[], &[], &None)
            .expect("empty closure descriptor is lossless");

        assert_eq!(value["op"].as_str(), Some("mir_call"));
        assert_eq!(
            value["mir_call"]["callee"]["type"].as_str(),
            Some("Closure")
        );
    }

    #[test]
    fn body_backed_new_closure_rejects_lossy_wire_projection() {
        let error = emit_new_closure(&ValueId::new(3), &[], &Some(7), &[], &[], &None)
            .expect_err("body identity must not be dropped");

        assert!(error.contains("closure-body-wire-unavailable"));
    }
}
