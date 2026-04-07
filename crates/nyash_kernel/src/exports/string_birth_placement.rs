use super::string_trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum BoundaryKind {
    ObserverOnly,
    Store,
    LoopCarry,
    AbiVisible,
    CloneShare,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RetainedForm {
    ReturnHandle,
    KeepTransient,
    RetainView,
    MustFreeze(BoundaryKind),
}

const SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES: usize = 8;

#[inline(always)]
fn retained_form_label(form: RetainedForm) -> &'static str {
    match form {
        RetainedForm::ReturnHandle => "return_handle",
        RetainedForm::KeepTransient => "keep_transient",
        RetainedForm::RetainView => "retain_view",
        RetainedForm::MustFreeze(_) => "must_freeze",
    }
}

#[inline(always)]
fn trace_retained_form(
    stage: &str,
    form: RetainedForm,
    reason: &str,
    extra: impl std::fmt::Display,
) {
    if !string_trace::enabled() {
        return;
    }
    string_trace::emit(stage, retained_form_label(form), reason, extra);
}

#[cold]
#[inline(never)]
fn trace_substring_retained_form_cold(
    view_enabled: bool,
    slice_len: usize,
    placement: RetainedForm,
) {
    if !string_trace::enabled() {
        return;
    }
    let reason = if !view_enabled {
        "view_disabled"
    } else if slice_len < SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES {
        "slice_len_lt_threshold"
    } else {
        "retain_view"
    };
    string_trace::emit(
        "placement",
        retained_form_label(placement),
        reason,
        format_args!(
            "view_enabled={} slice_len={} threshold={}",
            view_enabled, slice_len, SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES
        ),
    );
}

#[inline(always)]
pub(crate) fn substring_retention_class(view_enabled: bool, slice_len: usize) -> RetainedForm {
    let placement = if !view_enabled || slice_len < SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES {
        RetainedForm::MustFreeze(BoundaryKind::Store)
    } else {
        RetainedForm::RetainView
    };
    if string_trace::enabled() {
        trace_substring_retained_form_cold(view_enabled, slice_len, placement);
    }
    placement
}

#[inline(always)]
pub(crate) fn concat_suffix_retention_class(suffix_is_empty: bool) -> RetainedForm {
    let placement = if suffix_is_empty {
        RetainedForm::ReturnHandle
    } else {
        RetainedForm::KeepTransient
    };
    if string_trace::enabled() {
        let reason = if suffix_is_empty {
            "suffix_empty"
        } else {
            "suffix_non_empty"
        };
        trace_retained_form(
            "placement",
            placement,
            reason,
            format_args!("suffix_is_empty={}", suffix_is_empty),
        );
    }
    placement
}

#[inline(always)]
pub(crate) fn insert_middle_retention_class(
    source_is_empty: bool,
    middle_is_empty: bool,
) -> RetainedForm {
    let placement = if middle_is_empty {
        RetainedForm::ReturnHandle
    } else if source_is_empty {
        RetainedForm::MustFreeze(BoundaryKind::Store)
    } else {
        RetainedForm::KeepTransient
    };
    if string_trace::enabled() {
        let reason = if middle_is_empty {
            "middle_empty"
        } else if source_is_empty {
            "source_empty"
        } else {
            "keep_transient"
        };
        trace_retained_form(
            "placement",
            placement,
            reason,
            format_args!(
                "source_is_empty={} middle_is_empty={}",
                source_is_empty, middle_is_empty
            ),
        );
    }
    placement
}

#[inline(always)]
pub(crate) fn concat3_retention_class(
    a_is_empty: bool,
    b_is_empty: bool,
    c_is_empty: bool,
    allow_handle_reuse: bool,
) -> RetainedForm {
    let placement = if allow_handle_reuse
        && ((a_is_empty && b_is_empty) || (a_is_empty && c_is_empty) || (b_is_empty && c_is_empty))
    {
        RetainedForm::ReturnHandle
    } else {
        RetainedForm::KeepTransient
    };
    if string_trace::enabled() {
        let reason = if allow_handle_reuse
            && ((a_is_empty && b_is_empty)
                || (a_is_empty && c_is_empty)
                || (b_is_empty && c_is_empty))
        {
            "reuse_handle"
        } else {
            "keep_transient"
        };
        trace_retained_form(
            "placement",
            placement,
            reason,
            format_args!(
                "allow_handle_reuse={} a_is_empty={} b_is_empty={} c_is_empty={}",
                allow_handle_reuse, a_is_empty, b_is_empty, c_is_empty
            ),
        );
    }
    placement
}

#[cfg(test)]
mod tests {
    use super::{
        concat3_retention_class, concat_suffix_retention_class, insert_middle_retention_class,
    };
    use super::{substring_retention_class, BoundaryKind, RetainedForm};

    #[test]
    fn substring_placement_distinguishes_view_and_freeze() {
        assert_eq!(substring_retention_class(true, 9), RetainedForm::RetainView);
        assert_eq!(substring_retention_class(true, 8), RetainedForm::RetainView);
        assert_eq!(
            substring_retention_class(false, 9),
            RetainedForm::MustFreeze(BoundaryKind::Store)
        );
        assert_eq!(
            substring_retention_class(false, 8),
            RetainedForm::MustFreeze(BoundaryKind::Store)
        );
    }

    #[test]
    fn concat_and_insert_placement_keep_transient_or_reuse_handle() {
        assert_eq!(
            concat_suffix_retention_class(true),
            RetainedForm::ReturnHandle
        );
        assert_eq!(
            concat_suffix_retention_class(false),
            RetainedForm::KeepTransient
        );
        assert_eq!(
            insert_middle_retention_class(true, false),
            RetainedForm::MustFreeze(BoundaryKind::Store)
        );
        assert_eq!(
            insert_middle_retention_class(false, true),
            RetainedForm::ReturnHandle
        );
        assert_eq!(
            insert_middle_retention_class(false, false),
            RetainedForm::KeepTransient
        );
        assert_eq!(
            concat3_retention_class(false, false, false, true),
            RetainedForm::KeepTransient
        );
        assert_eq!(
            concat3_retention_class(true, true, false, true),
            RetainedForm::ReturnHandle
        );
    }
}
