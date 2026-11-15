//! Observed operation counts shared by current loop physical profiles.
use super::operation_dispatcher::{
    CompletedLoopOperationDispatchV1, LoopOperationDispatchReceiptV1,
};

pub(super) fn profile_counts_from_dispatch(
    completed: &CompletedLoopOperationDispatchV1,
) -> (usize, usize, usize, usize) {
    let mut counts = (completed.operation_count(), 0, 0, 0);
    for receipt in completed.receipts() {
        match receipt {
            LoopOperationDispatchReceiptV1::Pure(_) => counts.1 += 1,
            LoopOperationDispatchReceiptV1::Read(_)
            | LoopOperationDispatchReceiptV1::CarrierSeed(_) => counts.2 += 1,
            LoopOperationDispatchReceiptV1::Write(_) => counts.3 += 1,
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::{profile_counts_from_dispatch, CompletedLoopOperationDispatchV1};

    #[test]
    fn profile_counts_use_observed_dispatch_operation_count() {
        let completed = CompletedLoopOperationDispatchV1 {
            operation_count: 3,
            receipts: Vec::new().into_boxed_slice(),
        };
        assert_eq!(profile_counts_from_dispatch(&completed), (3, 0, 0, 0));
    }
}
