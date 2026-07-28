use super::inventory::NormalSourceSurfaceInventoryV1;
use super::product::{
    NormalAdditionalCallableSiteV1, PreparedNormalSourcePlanInputV1,
    SealedNormalCallableModuleSourceV1, SealedNormalMainSourceV1, SealedNormalScalarRootV1,
    SealedNormalScriptSourceV1, SealedNormalSourcePlanV1,
};
use super::rejection::{NormalSourcePlanErrorV1, RejectedNormalSourcePlanV1};

pub(crate) struct NormalSourcePlanClassifierV1;

impl NormalSourcePlanClassifierV1 {
    pub(crate) fn seal(
        input: PreparedNormalSourcePlanInputV1,
    ) -> Result<SealedNormalSourcePlanV1, RejectedNormalSourcePlanV1> {
        let inventory = NormalSourceSurfaceInventoryV1::collect(input)?;
        Self::seal_inventory(inventory)
    }

    fn seal_inventory(
        inventory: NormalSourceSurfaceInventoryV1,
    ) -> Result<SealedNormalSourcePlanV1, RejectedNormalSourcePlanV1> {
        let NormalSourceSurfaceInventoryV1 {
            input,
            script_sites,
            top_level_callables,
            main_boxes,
            non_main_box_sites: _,
            unsupported,
        } = inventory;

        if main_boxes.len() > 1 {
            return Err(RejectedNormalSourcePlanV1::new(
                input,
                NormalSourcePlanErrorV1::DuplicateMain,
            ));
        }
        if let Some(site) = unsupported.into_vec().into_iter().next() {
            return Err(RejectedNormalSourcePlanV1::new(
                input,
                NormalSourcePlanErrorV1::UnsupportedTopLevelSurface {
                    statement_index: site.statement_index,
                    kind: site.kind,
                },
            ));
        }
        if !script_sites.is_empty() && (!main_boxes.is_empty() || !top_level_callables.is_empty()) {
            return Err(RejectedNormalSourcePlanV1::new(
                input,
                NormalSourcePlanErrorV1::MixedSourceFamilies,
            ));
        }

        let Some(main_box) = main_boxes.into_vec().pop() else {
            if !top_level_callables.is_empty() {
                return Err(RejectedNormalSourcePlanV1::new(
                    input,
                    NormalSourcePlanErrorV1::MissingSourceEntry,
                ));
            }
            return Ok(SealedNormalSourcePlanV1::ScalarRoot(
                SealedNormalScalarRootV1::Script(SealedNormalScriptSourceV1::seal(
                    input,
                    script_sites,
                )),
            ));
        };

        let (main_box_site, main_method, helper_methods) =
            match super::main_source::validate_main_surface(&main_box) {
                Ok(validated) => validated,
                Err(error) => return Err(RejectedNormalSourcePlanV1::new(input, error)),
            };

        let mut additional_callables = top_level_callables
            .into_vec()
            .into_iter()
            .map(NormalAdditionalCallableSiteV1::TopLevel)
            .collect::<Vec<_>>();
        additional_callables.extend(
            helper_methods
                .into_iter()
                .map(NormalAdditionalCallableSiteV1::MainMethod),
        );

        if additional_callables.is_empty() {
            return Ok(SealedNormalSourcePlanV1::ScalarRoot(
                SealedNormalScalarRootV1::Main0(SealedNormalMainSourceV1::seal(
                    input,
                    main_box_site,
                    main_method,
                )),
            ));
        }

        Ok(SealedNormalSourcePlanV1::CallableModule(
            SealedNormalCallableModuleSourceV1::seal(
                input,
                main_box_site,
                main_method,
                additional_callables.into_boxed_slice(),
            ),
        ))
    }
}
