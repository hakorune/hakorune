use super::*;

#[test]
fn extern_call_route_specs_are_complete_and_self_classifying() {
    let specs = extern_call_route_specs();
    assert_eq!(specs.len(), 28);

    for spec in specs {
        let kind = spec.kind();
        assert_eq!(
            classify_extern_call_route(spec.symbol(), spec.arity()),
            Some(kind)
        );
        for alias in spec.aliases() {
            assert_eq!(classify_extern_call_route(alias, spec.arity()), Some(kind));
        }
        assert_eq!(kind.route_id(), spec.route_id());
        assert_eq!(kind.core_op(), spec.core_op());
        assert_eq!(kind.symbol(), spec.symbol());
        assert_eq!(kind.proof(), spec.proof());
        assert_eq!(kind.return_shape(), spec.return_shape());
        assert_eq!(kind.value_demand(), spec.value_demand());
        assert_eq!(kind.effect_tags(), spec.effect_tags());
        assert_eq!(kind.arity(), spec.arity());
        assert_eq!(kind.value_arg_index(), spec.value_arg_index());
        assert_eq!(kind.accepts_void_result(), spec.accepts_void_result());
    }
}
