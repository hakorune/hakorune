//! HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0 fixtures for the live bundle.

use super::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::{ConstValue, MirBuilder};

#[test]
fn access_port_keeps_shell_metadata_and_headers_as_short_separate_loans() {
    let mut builder = MirBuilder::new();
    let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);

    invocation.with_access_port(|builder, access| {
        access.with_shell(|shell| {
            assert_eq!(shell.module_name(), "disconnected-invocation");
            shell.set_global("answer".into(), ConstValue::Integer(42));
            assert!(shell.globals().contains_key("answer"));
        });

        access.with_headers(|headers| {
            assert_eq!(headers.symbol_count(), 0);
            assert!(!headers.contains_symbol("main"));
        });

        access.with_finalizer_headers(|headers| {
            assert_eq!(headers.symbol_count(), 0);
            assert!(headers.signature("main").is_none());
        });

        assert_eq!(builder.next_value_id().0, 0);
    });
}
