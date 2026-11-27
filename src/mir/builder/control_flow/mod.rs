//! Control-flow entrypoints and shared MIR CFG, exit, and V2 loop support.

use super::ValueId;
use crate::ast::ASTNode;

pub(in crate::mir::builder) mod cleanup;
pub(in crate::mir::builder) mod debug;
pub(in crate::mir::builder) mod edgecfg;
pub(in crate::mir::builder) mod joinir;
pub(in crate::mir::builder) mod lower;
pub(in crate::mir::builder) mod plan;

impl super::MirBuilder {

    /// Lower an If while retaining the caller's raw child-descent port.
    pub(in crate::mir::builder) fn cf_if_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        condition: ASTNode,
        then_branch: ASTNode,
        else_branch: Option<ASTNode>,
    ) -> Result<ValueId, String>
    where
        Port: crate::mir::builder::recursive_child_lowering::RawAstChildLoweringPortV1,
    {
        use crate::mir::builder::if_form::IfBranchKindV1;
        use crate::mir::builder::recursive_child_lowering::drive_legacy_expression_v1;

        let checked = port.prepare_checked_bool_condition_v1()?;
        let (condition_value, mut checked) = match checked {
            Some(frame) => {
                let lowered = port.lower_checked_bool_condition_v1(self, frame, condition)?;
                (lowered.value(), Some(lowered))
            },
            None => (drive_legacy_expression_v1(self, port, condition)?, None),
        };
        let has_explicit_else = else_branch.is_some();
        let mut then_branch = Some(then_branch);
        let mut else_branch = else_branch;
        self.lower_if_form_with_physical_observer_v1(
            condition_value,
            None,
            has_explicit_else,
            move |builder, branch| match branch {
                IfBranchKindV1::Then => drive_legacy_expression_v1(
                    builder,
                    port,
                    then_branch
                        .take()
                        .ok_or_else(|| "[if-form/raw-then-demanded-twice]".to_string())?,
                ),
                IfBranchKindV1::Else => drive_legacy_expression_v1(
                    builder,
                    port,
                    else_branch
                        .take()
                        .ok_or_else(|| "[if-form/raw-else-demanded-without-input]".to_string())?,
                ),
            },
            checked.as_mut().map(|lowered| lowered.observer()),
        )
    }
}
