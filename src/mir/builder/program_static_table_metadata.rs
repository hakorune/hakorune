//! Selected normal Program static-table metadata transaction.
//!
//! This owner prepares the source-derived spec/plan pair before either
//! candidate-module metadata field changes, then commits both together.

use crate::ast::ASTNode;
use crate::mir::function::{MirModule, StaticDataPlan, StaticTableContractSpec};
use crate::mir::static_data_plan::{
    collect_static_table_specs_from_ast, static_data_plans_from_specs,
};

#[derive(Debug)]
pub(super) struct PreparedNormalProgramStaticTableMetadataV1<'module> {
    target: &'module mut MirModule,
    specs: Box<[StaticTableContractSpec]>,
    plans: Box<[StaticDataPlan]>,
}

impl<'module> PreparedNormalProgramStaticTableMetadataV1<'module> {
    pub(super) fn prepare(
        snapshot: &ASTNode,
        target: &'module mut MirModule,
    ) -> Result<Self, String> {
        let specs = collect_static_table_specs_from_ast(&target.name, snapshot)?;
        let plans = static_data_plans_from_specs(&specs);
        Ok(Self {
            target,
            specs: specs.into_boxed_slice(),
            plans: plans.into_boxed_slice(),
        })
    }

    pub(super) fn commit(self) {
        self.target.metadata.static_table_contract_specs = self.specs.into_vec();
        self.target.metadata.static_data_plans = self.plans.into_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn program(statements: Vec<ASTNode>) -> ASTNode {
        ASTNode::Program {
            statements,
            span: Span::unknown(),
        }
    }

    fn static_table(name: &str, element: &str, values: &[u64]) -> ASTNode {
        ASTNode::StaticConstTable {
            name: name.to_owned(),
            element_type: element.to_owned(),
            values: values.to_vec(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn commits_source_ordered_spec_and_plan_pairs_together() {
        let snapshot = program(vec![
            static_table("FIRST", "u16", &[1, 2]),
            static_table("SECOND", "u16", &[3]),
        ]);
        let mut target = MirModule::new("normal_static_table_pair/0".to_owned());

        PreparedNormalProgramStaticTableMetadataV1::prepare(&snapshot, &mut target)
            .expect("prepare static tables")
            .commit();

        assert_eq!(
            target
                .metadata
                .static_table_contract_specs
                .iter()
                .map(|spec| spec.table_id.declaration_name.as_str())
                .collect::<Vec<_>>(),
            ["FIRST", "SECOND"]
        );
        assert_eq!(
            target
                .metadata
                .static_data_plans
                .iter()
                .map(|plan| plan.source_name.as_str())
                .collect::<Vec<_>>(),
            ["FIRST", "SECOND"]
        );
    }

    #[test]
    fn failed_prepare_leaves_candidate_metadata_unpublished() {
        let snapshot = program(vec![static_table("BAD", "u8", &[1])]);
        let mut target = MirModule::new("normal_static_table_failure/0".to_owned());

        let error = PreparedNormalProgramStaticTableMetadataV1::prepare(&snapshot, &mut target)
            .expect_err("unsupported static-table element must reject");

        assert!(
            error.contains("[static-const/unsupported-element]"),
            "{error}"
        );
        assert!(target.metadata.static_table_contract_specs.is_empty());
        assert!(target.metadata.static_data_plans.is_empty());
    }
}
