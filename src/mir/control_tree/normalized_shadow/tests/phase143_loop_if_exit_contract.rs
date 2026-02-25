use crate::mir::control_tree::normalized_shadow::common::loop_if_exit_contract::*;

#[test]
fn test_shape_p0_break_only() {
        let shape = LoopIfExitShape::p0_break_only();
        assert_eq!(shape.then, LoopIfExitThen::Break);
        assert!(!shape.has_else);
        assert!(shape.else_.is_none());
    }

    #[test]
    fn test_shape_validate_p0_break_ok() {
        let shape = LoopIfExitShape::p0_break_only();
        assert!(shape.validate_for_p0().is_ok());
    }

    #[test]
    fn test_shape_validate_p0_else_not_supported() {
        let shape = LoopIfExitShape {
            has_else: true,
            then: LoopIfExitThen::Break,
            else_: Some(LoopIfExitThen::Continue),
            cond_scope: crate::mir::control_tree::normalized_shadow::common::expr_lowering_contract::ExprLoweringScope::PureOnly,
        };
        assert!(matches!(
            shape.validate_for_p0(),
            Err(OutOfScopeReason::ElseNotSupported(_))
        ));
    }

    #[test]
    fn test_shape_validate_p0_continue_not_supported() {
        let shape = LoopIfExitShape {
            has_else: false,
            then: LoopIfExitThen::Continue,
            else_: None,
            cond_scope: crate::mir::control_tree::normalized_shadow::common::expr_lowering_contract::ExprLoweringScope::PureOnly,
        };
        assert!(matches!(
            shape.validate_for_p0(),
            Err(OutOfScopeReason::ThenNotExit(_))
        ));
    }

    #[test]
    fn test_shape_validate_p1_continue_ok() {
        let shape = LoopIfExitShape {
            has_else: false,
            then: LoopIfExitThen::Continue,
            else_: None,
            cond_scope: crate::mir::control_tree::normalized_shadow::common::expr_lowering_contract::ExprLoweringScope::PureOnly,
        };
        assert!(shape.validate_for_p1().is_ok());
    }

    #[test]
    fn test_shape_validate_p1_break_ok() {
        let shape = LoopIfExitShape::p0_break_only();
        assert!(shape.validate_for_p1().is_ok());
    }

    #[test]
    fn test_shape_validate_p2_break_else_continue_ok() {
        let shape = LoopIfExitShape {
            has_else: true,
            then: LoopIfExitThen::Break,
            else_: Some(LoopIfExitThen::Continue),
            cond_scope: crate::mir::control_tree::normalized_shadow::common::expr_lowering_contract::ExprLoweringScope::PureOnly,
        };
        assert!(shape.validate_for_p2().is_ok());
    }

    #[test]
    fn test_loop_if_exit_then_eq() {
        assert_eq!(LoopIfExitThen::Break, LoopIfExitThen::Break);
        assert_eq!(LoopIfExitThen::Continue, LoopIfExitThen::Continue);
        assert_ne!(LoopIfExitThen::Break, LoopIfExitThen::Continue);
    }

