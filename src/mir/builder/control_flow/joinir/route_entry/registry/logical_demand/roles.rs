//! Logical roles that are independent of physical lowering identities.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LogicalRoleV1 {
    LoopBinding,
    AccumulatorBinding,
    ResultBinding,
    LoopCarrier,
    ExitCarrier,
    BreakExit,
    ContinueExit,
    ReturnExit,
    LoopBackContinuation,
    ExitContinuation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LogicalRoleSetErrorV1 {
    Duplicate(LogicalRoleV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LogicalRoleSetV1 {
    roles: Box<[LogicalRoleV1]>,
}

impl LogicalRoleSetV1 {
    pub(crate) fn try_new(
        roles: impl Into<Box<[LogicalRoleV1]>>,
    ) -> Result<Self, LogicalRoleSetErrorV1> {
        let roles = roles.into();
        for (index, role) in roles.iter().enumerate() {
            if roles[..index].contains(role) {
                return Err(LogicalRoleSetErrorV1::Duplicate(*role));
            }
        }
        Ok(Self { roles })
    }

    pub(crate) fn ordered(&self) -> &[LogicalRoleV1] {
        &self.roles
    }
}

#[cfg(test)]
mod tests {
    use super::{LogicalRoleSetErrorV1, LogicalRoleSetV1, LogicalRoleV1};

    #[test]
    fn roles_keep_declared_order_and_reject_duplicates() {
        let roles = LogicalRoleSetV1::try_new(
            vec![
                LogicalRoleV1::LoopBinding,
                LogicalRoleV1::LoopBackContinuation,
            ]
            .into_boxed_slice(),
        )
        .expect("unique roles");
        assert_eq!(
            roles.ordered(),
            &[
                LogicalRoleV1::LoopBinding,
                LogicalRoleV1::LoopBackContinuation
            ]
        );
        assert_eq!(
            LogicalRoleSetV1::try_new(
                vec![LogicalRoleV1::LoopCarrier, LogicalRoleV1::LoopCarrier].into_boxed_slice(),
            ),
            Err(LogicalRoleSetErrorV1::Duplicate(LogicalRoleV1::LoopCarrier))
        );
    }
}
