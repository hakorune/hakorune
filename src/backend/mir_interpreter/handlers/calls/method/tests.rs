    use super::*;
    use crate::backend::vm_types::VMError;
    use crate::box_trait::StringBox;
    use std::sync::Arc;

    fn stringbox_receiver(text: &str) -> VMValue {
        VMValue::BoxRef(Arc::new(StringBox::new(text)))
    }

    #[test]
    fn method_dispatch_stringbox_is_space_after_slot_miss() {
        let mut interp = MirInterpreter::new();
        let arg = ValueId(1);
        interp.regs.insert(arg, VMValue::String(" ".to_string()));

        let got = interp
            .execute_method_call(&stringbox_receiver("seed"), "is_space", &[arg])
            .expect("is_space should stay handled");

        assert_eq!(got, VMValue::Bool(true));
    }

    #[test]
    fn method_dispatch_stringbox_is_alpha_after_slot_miss() {
        let mut interp = MirInterpreter::new();
        let arg = ValueId(1);
        interp.regs.insert(arg, VMValue::String("A".to_string()));

        let got = interp
            .execute_method_call(&stringbox_receiver("seed"), "is_alpha", &[arg])
            .expect("is_alpha should stay handled");

        assert_eq!(got, VMValue::Bool(true));
    }

    #[test]
    fn method_dispatch_stringbox_unknown_method_still_fails_fast() {
        let mut interp = MirInterpreter::new();

        let err = interp
            .execute_method_call(&stringbox_receiver("seed"), "missing_method", &[])
            .expect_err("unknown StringBox method must fail fast");

        match err {
            VMError::InvalidInstruction(msg) => {
                assert_eq!(msg, "Unknown method 'missing_method' on StringBox");
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
