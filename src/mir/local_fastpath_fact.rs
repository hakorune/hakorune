/*!
 * LocalFastPathFact aggregation owner.
 *
 * Positive fastpath facts are backend-consumable proof. Producer families may
 * expose route/object evidence, but this module owns the final assignment to
 * `MirFunction.metadata.local_fastpath_facts` so producers cannot clobber each
 * other as the surface grows.
 */

pub use hakorune_mir_plans::local_fastpath_fact::build_local_fastpath_facts_from_map_repr_plans;

use crate::mir::MirFunction;

pub fn refresh_function_local_fastpath_facts(function: &mut MirFunction) {
    function.metadata.local_fastpath_facts =
        build_local_fastpath_facts_from_map_repr_plans(&function.metadata.map_repr_plans);
}
