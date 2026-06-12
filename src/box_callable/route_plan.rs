//! Route plan vocabulary derived from BoxCallableRegistry entries.
//!
//! These plans are the execution-facing shape. Type ABI / BoxDescriptor
//! projections must not be queried from hot paths.

use super::{BoxCallableTarget, FunctionId, IntrinsicId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvokeRoutePlan {
    PluginV2 {
        type_id: u32,
        invoke_box_available: bool,
        allow_compat_shim: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodCallRoutePlan {
    InternalSlot {
        slot: u16,
    },
    PluginInvoke {
        type_id: u32,
        method_id: u32,
        returns_result: bool,
        invoke_route: InvokeRoutePlan,
    },
    UserFunction {
        function_id: FunctionId,
    },
    Intrinsic {
        intrinsic_id: IntrinsicId,
    },
    SlowDynamic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewBoxRoutePlan {
    Builtin {
        type_id: u32,
    },
    UserBoxConstructor {
        type_id: u32,
        function_id: FunctionId,
    },
    PluginBirth {
        type_id: u32,
        birth_id: u32,
        fini_id: Option<u32>,
        invoke_route: InvokeRoutePlan,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropBoxRoutePlan {
    None,
    UserFini {
        type_id: u32,
        function_id: FunctionId,
    },
    PluginFini {
        type_id: u32,
        fini_id: u32,
        invoke_route: InvokeRoutePlan,
    },
}

impl MethodCallRoutePlan {
    pub fn from_target(
        target: &BoxCallableTarget,
        invoke_route: Option<InvokeRoutePlan>,
    ) -> Option<Self> {
        match target {
            BoxCallableTarget::InternalSlot { slot } => {
                Some(MethodCallRoutePlan::InternalSlot { slot: *slot })
            }
            BoxCallableTarget::PluginMethod {
                type_id,
                method_id,
                returns_result,
            } => Some(MethodCallRoutePlan::PluginInvoke {
                type_id: *type_id,
                method_id: *method_id,
                returns_result: *returns_result,
                invoke_route: invoke_route?,
            }),
            BoxCallableTarget::UserFunction { function_id } => {
                Some(MethodCallRoutePlan::UserFunction {
                    function_id: *function_id,
                })
            }
            BoxCallableTarget::Intrinsic { intrinsic_id } => Some(MethodCallRoutePlan::Intrinsic {
                intrinsic_id: *intrinsic_id,
            }),
            BoxCallableTarget::PluginLifecycle { .. } => None,
        }
    }
}

impl NewBoxRoutePlan {
    pub fn plugin_birth_from_target(
        target: &BoxCallableTarget,
        invoke_route: InvokeRoutePlan,
    ) -> Option<Self> {
        let BoxCallableTarget::PluginLifecycle {
            type_id,
            birth_id: Some(birth_id),
            fini_id,
        } = target
        else {
            return None;
        };
        Some(NewBoxRoutePlan::PluginBirth {
            type_id: *type_id,
            birth_id: *birth_id,
            fini_id: *fini_id,
            invoke_route,
        })
    }
}

impl DropBoxRoutePlan {
    pub fn plugin_fini_from_target(
        target: &BoxCallableTarget,
        invoke_route: InvokeRoutePlan,
    ) -> Option<Self> {
        let BoxCallableTarget::PluginLifecycle {
            type_id,
            fini_id: Some(fini_id),
            ..
        } = target
        else {
            return None;
        };
        Some(DropBoxRoutePlan::PluginFini {
            type_id: *type_id,
            fini_id: *fini_id,
            invoke_route,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invoke_route() -> InvokeRoutePlan {
        InvokeRoutePlan::PluginV2 {
            type_id: 42,
            invoke_box_available: true,
            allow_compat_shim: false,
        }
    }

    #[test]
    fn method_call_plan_keeps_internal_slot_target() {
        let target = BoxCallableTarget::InternalSlot { slot: 309 };

        let plan = MethodCallRoutePlan::from_target(&target, None);

        assert_eq!(plan, Some(MethodCallRoutePlan::InternalSlot { slot: 309 }));
    }

    #[test]
    fn method_call_plan_keeps_plugin_method_id_space() {
        let target = BoxCallableTarget::PluginMethod {
            type_id: 42,
            method_id: 7,
            returns_result: true,
        };

        let plan = MethodCallRoutePlan::from_target(&target, Some(invoke_route()));

        assert_eq!(
            plan,
            Some(MethodCallRoutePlan::PluginInvoke {
                type_id: 42,
                method_id: 7,
                returns_result: true,
                invoke_route: invoke_route(),
            })
        );
    }

    #[test]
    fn method_call_plan_requires_invoke_route_for_plugin_method() {
        let target = BoxCallableTarget::PluginMethod {
            type_id: 42,
            method_id: 7,
            returns_result: true,
        };

        let plan = MethodCallRoutePlan::from_target(&target, None);

        assert_eq!(plan, None);
    }

    #[test]
    fn lifecycle_target_can_build_newbox_and_dropbox_plans() {
        let target = BoxCallableTarget::PluginLifecycle {
            type_id: 42,
            birth_id: Some(1),
            fini_id: Some(999),
        };

        let new_plan = NewBoxRoutePlan::plugin_birth_from_target(&target, invoke_route());
        let drop_plan = DropBoxRoutePlan::plugin_fini_from_target(&target, invoke_route());

        assert_eq!(
            new_plan,
            Some(NewBoxRoutePlan::PluginBirth {
                type_id: 42,
                birth_id: 1,
                fini_id: Some(999),
                invoke_route: invoke_route(),
            })
        );
        assert_eq!(
            drop_plan,
            Some(DropBoxRoutePlan::PluginFini {
                type_id: 42,
                fini_id: 999,
                invoke_route: invoke_route(),
            })
        );
    }
}
