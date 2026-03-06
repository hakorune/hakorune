use crate::backend::mir_interpreter::MirInterpreter;
use crate::mir::join_ir::{JoinFuncId, JoinModule};

use super::exec::execute_function;
use super::{JoinRuntimeError, JoinValue};

pub fn run_joinir_function(
    vm: &mut MirInterpreter,
    module: &JoinModule,
    entry: JoinFuncId,
    args: &[JoinValue],
) -> Result<JoinValue, JoinRuntimeError> {
    // Phase R1/R4: the removed dev-only normalized route no longer participates here.
    // Direct JoinIR runner stays Structured-only.
    execute_function(vm, module, entry, args.to_vec())
}
