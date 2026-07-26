use super::inventory::{
    NormalMainBoxSurfaceV1, NormalMethodSurfaceV1, NormalSourceSurfaceInventoryV1,
};
use super::product::{
    NormalAdditionalCallableSiteV1, NormalMainMethodSiteV1, PreparedNormalSourcePlanInputV1,
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

        let (main_box_site, main_method, helper_methods) = match Self::validate_main(main_box) {
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

    fn validate_main(
        main_box: NormalMainBoxSurfaceV1,
    ) -> Result<
        (
            super::product::NormalTopLevelSiteV1,
            NormalMainMethodSiteV1,
            Vec<NormalMainMethodSiteV1>,
        ),
        NormalSourcePlanErrorV1,
    > {
        if !main_box.is_static {
            return Err(NormalSourcePlanErrorV1::MainMustBeStatic);
        }

        let mut main_method = None;
        let mut helper_methods = Vec::new();
        for method in main_box.methods {
            if method.method_key.as_ref() == "main" {
                main_method = Some(Self::validate_main_method(
                    main_box.site.statement_index(),
                    method,
                )?);
            } else {
                helper_methods.push(Self::validate_helper_method(
                    main_box.site.statement_index(),
                    method,
                )?);
            }
        }
        let Some(main_method) = main_method else {
            return Err(NormalSourcePlanErrorV1::MainMethodMissing);
        };
        Ok((main_box.site, main_method, helper_methods))
    }

    fn validate_main_method(
        main_statement_index: usize,
        method: NormalMethodSurfaceV1,
    ) -> Result<NormalMainMethodSiteV1, NormalSourcePlanErrorV1> {
        let Some(declaration_name) = method.declaration_name else {
            return Err(NormalSourcePlanErrorV1::MainMethodMustBeFunction);
        };
        if declaration_name.as_ref() != "main" {
            return Err(NormalSourcePlanErrorV1::MainMethodNameMismatch {
                method_key: method.method_key,
                declaration_name,
            });
        }
        let (Some(arity), Some(is_static)) = (method.arity, method.is_static) else {
            return Err(NormalSourcePlanErrorV1::MainMethodMustBeFunction);
        };
        if !is_static {
            return Err(NormalSourcePlanErrorV1::MainMethodMustBeStatic);
        }
        if arity != 0 {
            return Err(NormalSourcePlanErrorV1::MainArityMismatch { actual: arity });
        }
        Ok(NormalMainMethodSiteV1::new(
            main_statement_index,
            method.method_key,
            arity,
            is_static,
        ))
    }

    fn validate_helper_method(
        main_statement_index: usize,
        method: NormalMethodSurfaceV1,
    ) -> Result<NormalMainMethodSiteV1, NormalSourcePlanErrorV1> {
        let Some(declaration_name) = method.declaration_name else {
            return Err(NormalSourcePlanErrorV1::MainHelperMustBeFunction {
                method_key: method.method_key,
            });
        };
        if declaration_name.as_ref() != method.method_key.as_ref() {
            return Err(NormalSourcePlanErrorV1::MainHelperNameMismatch {
                method_key: method.method_key,
                declaration_name,
            });
        }
        let (Some(arity), Some(is_static)) = (method.arity, method.is_static) else {
            return Err(NormalSourcePlanErrorV1::MainHelperMustBeFunction {
                method_key: method.method_key,
            });
        };
        Ok(NormalMainMethodSiteV1::new(
            main_statement_index,
            method.method_key,
            arity,
            is_static,
        ))
    }
}
