use crate::mir::MirType;

pub(super) fn format_mir_type_label(ty: &MirType) -> String {
    match ty {
        MirType::Integer => "i64".to_string(),
        MirType::Float => "f64".to_string(),
        MirType::Bool => "i1".to_string(),
        MirType::String => "str".to_string(),
        MirType::Box(name) => format!("box<{}>", name),
        MirType::Array(inner) => format!("[{}]", format_mir_type_label(inner)),
        MirType::Future(inner) => format!("future<{}>", format_mir_type_label(inner)),
        MirType::WeakRef => "weakref".to_string(),
        MirType::Void => "void".to_string(),
        MirType::Unknown => "?".to_string(),
    }
}
