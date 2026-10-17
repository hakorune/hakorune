//! Original read arguments checked against the same finished Call and boundary.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::MirFunction;

impl EmittedLexicalCallProjectionV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn array_read_records_v1(
        &self,
    ) -> impl Iterator<Item = (u32, &std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedOwnedArrayReadV1>)> {
        self.prepared.arguments.iter().filter_map(|argument| match argument {
            LexicalCallArgumentProjectionV1::BorrowedReadArgument { ordinal, read, .. } =>
                read.array_projection_v1().map(|read| (*ordinal, read)),
            _ => None,
        })
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_finished_read_arguments_v1(
        &self,
        function: &MirFunction,
        context: CallValidationContextV1<'_>,
        projection: &FinishedBindings,
        call_coordinate: (BasicBlockId, usize),
    ) -> Result<(), String> {
        for argument in &self.prepared.arguments {
            let LexicalCallArgumentProjectionV1::BorrowedReadArgument {
                ordinal, site, read, ..
            } = argument else { continue; };
            // Demand the same original argument/formal/source identity before
            // lending its physical record. No finished shape selects a source.
            borrowed_projection::value(
                argument, self.call_site().owner(), self.row.loan(),
                *ordinal, site, context,
            )?;
            read.validate_finished_v1(
                function, projection, context, self.call_site(),
                &self.invoke, call_coordinate, *ordinal,
            )?;
        }
        Ok(())
    }
}
