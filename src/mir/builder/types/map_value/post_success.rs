//! Builder-free replay schedule for successful MapBox write observations.
//!
//! This module preserves existing receiver-keyed observation order without
//! owning map facts, LocalSSA, routing, or physical Call emission.

#![allow(dead_code)] // MAP-WRITE-OBSERVE0-S0 is intentionally disconnected.

use crate::mir::{Callee, ValueId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MapWriteOperationV1 {
    Set,
    Delete,
    Clear,
}

/// One existing semantic MapBox write observation to replay after a receipt.
#[derive(Debug)]
pub(super) struct MapWriteObservationDescriptorV1 {
    callee: Callee,
    args: Box<[ValueId]>,
    receiver: ValueId,
    operation: MapWriteOperationV1,
}

impl MapWriteObservationDescriptorV1 {
    fn from_existing_call(callee: &Callee, args: &[ValueId]) -> Option<Self> {
        let Callee::Method {
            box_name,
            method,
            receiver: Some(receiver),
            ..
        } = callee
        else {
            return None;
        };
        if box_name != "MapBox" {
            return None;
        }

        let user_arg_count = args
            .strip_prefix(std::slice::from_ref(receiver))
            .unwrap_or(args)
            .len();
        let method_id = crate::boxes::MapMethodId::from_name_and_arity(method, user_arg_count)?;
        let operation = match method_id {
            crate::boxes::MapMethodId::Set => MapWriteOperationV1::Set,
            crate::boxes::MapMethodId::Delete => MapWriteOperationV1::Delete,
            crate::boxes::MapMethodId::Clear => MapWriteOperationV1::Clear,
            _ => return None,
        };

        Some(Self {
            callee: callee.clone(),
            args: args.into(),
            receiver: *receiver,
            operation,
        })
    }

    pub(super) fn callee(&self) -> &Callee {
        &self.callee
    }

    pub(super) fn args(&self) -> &[ValueId] {
        &self.args
    }

    pub(super) fn receiver(&self) -> ValueId {
        self.receiver
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum MapWriteReplayErrorV1 {
    UnsupportedDescriptor,
    OperationMismatch,
}

/// A non-Clone, single-receipt schedule of existing Map write observations.
///
/// S0 creates and verifies this structural product only. I0 will be its first
/// consumer and will invoke the existing map-fact owner after physical success.
#[derive(Debug)]
pub(super) struct PreparedMapWriteReplayV1 {
    operation: MapWriteOperationV1,
    observations: Vec<MapWriteObservationDescriptorV1>,
}

impl PreparedMapWriteReplayV1 {
    /// Starts a replay only for an existing MapBox Set/Delete/Clear descriptor.
    pub(super) fn prepare(callee: &Callee, args: &[ValueId]) -> Option<Self> {
        let first = MapWriteObservationDescriptorV1::from_existing_call(callee, args)?;
        Some(Self {
            operation: first.operation,
            observations: vec![first],
        })
    }

    /// Adds a finalized/delegated receiver replay only when it changes the
    /// receiver identity and retains the same existing map-write operation.
    pub(super) fn append_if_distinct_receiver(
        &mut self,
        callee: &Callee,
        args: &[ValueId],
    ) -> Result<(), MapWriteReplayErrorV1> {
        let next = MapWriteObservationDescriptorV1::from_existing_call(callee, args)
            .ok_or(MapWriteReplayErrorV1::UnsupportedDescriptor)?;
        if next.operation != self.operation {
            return Err(MapWriteReplayErrorV1::OperationMismatch);
        }
        if self
            .observations
            .last()
            .is_some_and(|current| current.receiver == next.receiver)
        {
            return Ok(());
        }
        self.observations.push(next);
        Ok(())
    }

    /// Transfers the exact prevalidated schedule to the future receipt owner.
    pub(super) fn into_observations(self) -> Box<[MapWriteObservationDescriptorV1]> {
        self.observations.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::{MapWriteReplayErrorV1, PreparedMapWriteReplayV1};
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{Callee, ValueId};

    fn map_callee(method: &str, receiver: u32) -> Callee {
        Callee::Method {
            box_name: "MapBox".to_string(),
            method: method.to_string(),
            receiver: Some(ValueId::new(receiver)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }
    }

    fn receivers(replay: PreparedMapWriteReplayV1) -> Vec<ValueId> {
        replay
            .into_observations()
            .iter()
            .map(|descriptor| descriptor.receiver())
            .collect()
    }

    #[test]
    fn accepts_only_existing_map_write_surface() {
        for (method, args) in [
            ("set", vec![ValueId::new(2), ValueId::new(3)]),
            ("delete", vec![ValueId::new(2)]),
            ("remove", vec![ValueId::new(2)]),
            ("clear", vec![]),
        ] {
            assert!(PreparedMapWriteReplayV1::prepare(&map_callee(method, 1), &args).is_some());
        }

        assert!(
            PreparedMapWriteReplayV1::prepare(&map_callee("get", 1), &[ValueId::new(2)]).is_none()
        );
        assert!(PreparedMapWriteReplayV1::prepare(
            &Callee::Global("MapBox.set".to_string()),
            &[ValueId::new(2), ValueId::new(3)],
        )
        .is_none());
    }

    #[test]
    fn direct_unified_schedule_replays_source_then_distinct_final_receiver() {
        let source = map_callee("set", 10);
        let mut replay =
            PreparedMapWriteReplayV1::prepare(&source, &[ValueId::new(20), ValueId::new(30)])
                .unwrap();
        replay
            .append_if_distinct_receiver(
                &map_callee("set", 11),
                &[ValueId::new(11), ValueId::new(21), ValueId::new(31)],
            )
            .unwrap();

        assert_eq!(receivers(replay), vec![ValueId::new(10), ValueId::new(11)]);
    }

    #[test]
    fn delegated_schedule_preserves_source_then_materialized_then_final_receiver() {
        let source = map_callee("delete", 10);
        let mut replay = PreparedMapWriteReplayV1::prepare(&source, &[ValueId::new(20)]).unwrap();
        replay
            .append_if_distinct_receiver(
                &map_callee("delete", 11),
                &[ValueId::new(11), ValueId::new(21)],
            )
            .unwrap();
        replay
            .append_if_distinct_receiver(
                &map_callee("delete", 12),
                &[ValueId::new(12), ValueId::new(22)],
            )
            .unwrap();

        assert_eq!(
            receivers(replay),
            vec![ValueId::new(10), ValueId::new(11), ValueId::new(12)]
        );
    }

    #[test]
    fn equal_receiver_deduplicates_but_a_different_operation_rejects() {
        let source = map_callee("clear", 10);
        let mut replay = PreparedMapWriteReplayV1::prepare(&source, &[]).unwrap();
        replay
            .append_if_distinct_receiver(&map_callee("clear", 10), &[ValueId::new(10)])
            .unwrap();
        assert_eq!(receivers(replay), vec![ValueId::new(10)]);

        let source = map_callee("set", 10);
        let mut replay =
            PreparedMapWriteReplayV1::prepare(&source, &[ValueId::new(20), ValueId::new(30)])
                .unwrap();
        assert_eq!(
            replay.append_if_distinct_receiver(
                &map_callee("delete", 11),
                &[ValueId::new(11), ValueId::new(21)],
            ),
            Err(MapWriteReplayErrorV1::OperationMismatch)
        );
    }
}
