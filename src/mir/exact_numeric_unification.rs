use crate::mir::numeric_substrate::ExactNumericMirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactNumericMergeSite {
    Phi,
    Select,
    BinaryOpArithmetic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExactNumericUnificationError {
    MixedExactAndDynamic {
        site: ExactNumericMergeSite,
        exact_source_name: String,
    },
    TypeMismatch {
        site: ExactNumericMergeSite,
        left_source_name: String,
        right_source_name: String,
    },
}

pub(crate) fn unify_exact_numeric_inputs(
    site: ExactNumericMergeSite,
    incoming: &[Option<ExactNumericMirType>],
) -> Result<Option<ExactNumericMirType>, ExactNumericUnificationError> {
    let mut unified: Option<ExactNumericMirType> = None;
    let mut saw_dynamic = false;

    for ty in incoming {
        match (unified.as_ref(), ty) {
            (None, None) => {
                saw_dynamic = true;
            }
            (None, Some(next)) => {
                if saw_dynamic {
                    return Err(ExactNumericUnificationError::MixedExactAndDynamic {
                        site,
                        exact_source_name: next.source_name.clone(),
                    });
                }
                unified = Some(next.clone());
            }
            (Some(existing), None) => {
                return Err(ExactNumericUnificationError::MixedExactAndDynamic {
                    site,
                    exact_source_name: existing.source_name.clone(),
                });
            }
            (Some(existing), Some(next)) if existing == next => {}
            (Some(existing), Some(next)) => {
                return Err(ExactNumericUnificationError::TypeMismatch {
                    site,
                    left_source_name: existing.source_name.clone(),
                    right_source_name: next.source_name.clone(),
                });
            }
        }
    }

    Ok(unified)
}

pub(crate) fn unify_exact_numeric_control_merge(
    site: ExactNumericMergeSite,
    incoming: &[Option<ExactNumericMirType>],
) -> Result<Option<ExactNumericMirType>, ExactNumericUnificationError> {
    unify_exact_numeric_inputs(site, incoming)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::numeric_substrate::{exact_numeric_mir_type_from_declared_name, NumericTarget};

    fn target() -> NumericTarget {
        NumericTarget::host()
    }

    #[test]
    fn unifies_all_dynamic_control_merge_as_no_exact_type() {
        assert_eq!(
            unify_exact_numeric_control_merge(ExactNumericMergeSite::Phi, &[None, None]),
            Ok(None)
        );
    }

    #[test]
    fn unifies_same_exact_numeric_type_for_phi() {
        let usize_ty = exact_numeric_mir_type_from_declared_name(Some("usize"), target()).unwrap();

        assert_eq!(
            unify_exact_numeric_control_merge(
                ExactNumericMergeSite::Phi,
                &[Some(usize_ty.clone()), Some(usize_ty.clone())],
            ),
            Ok(Some(usize_ty))
        );
    }

    #[test]
    fn rejects_exact_and_dynamic_mix_for_select() {
        let usize_ty = exact_numeric_mir_type_from_declared_name(Some("usize"), target()).unwrap();

        assert_eq!(
            unify_exact_numeric_control_merge(
                ExactNumericMergeSite::Select,
                &[Some(usize_ty), None],
            ),
            Err(ExactNumericUnificationError::MixedExactAndDynamic {
                site: ExactNumericMergeSite::Select,
                exact_source_name: "usize".to_string(),
            })
        );
    }

    #[test]
    fn rejects_dynamic_and_exact_mix_for_select() {
        let usize_ty = exact_numeric_mir_type_from_declared_name(Some("usize"), target()).unwrap();

        assert_eq!(
            unify_exact_numeric_control_merge(
                ExactNumericMergeSite::Select,
                &[None, Some(usize_ty)],
            ),
            Err(ExactNumericUnificationError::MixedExactAndDynamic {
                site: ExactNumericMergeSite::Select,
                exact_source_name: "usize".to_string(),
            })
        );
    }

    #[test]
    fn rejects_mismatched_exact_numeric_source_names() {
        let usize_ty = exact_numeric_mir_type_from_declared_name(Some("usize"), target()).unwrap();
        let u64_ty = exact_numeric_mir_type_from_declared_name(Some("u64"), target()).unwrap();

        assert_eq!(
            unify_exact_numeric_control_merge(
                ExactNumericMergeSite::Phi,
                &[Some(usize_ty), Some(u64_ty)],
            ),
            Err(ExactNumericUnificationError::TypeMismatch {
                site: ExactNumericMergeSite::Phi,
                left_source_name: "usize".to_string(),
                right_source_name: "u64".to_string(),
            })
        );
    }
}
