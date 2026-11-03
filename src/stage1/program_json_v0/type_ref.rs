use crate::ast::TypeReferenceV1;

#[derive(Debug, Clone)]
pub(super) struct TypeRef {
    pub name: String,
    pub args: Vec<TypeRef>,
}

pub(super) fn parse_type_ref_text(type_text: &str) -> Result<TypeRef, String> {
    fn legacy_projection(reference: TypeReferenceV1) -> Result<TypeRef, String> {
        match reference {
            TypeReferenceV1::Name(name) => Ok(TypeRef { name, args: Vec::new() }),
            TypeReferenceV1::Apply { name, args } => Ok(TypeRef {
                name, args: args.into_iter().map(legacy_projection).collect::<Result<_, _>>()?,
            }),
            // The existing program-JSON contract omits array suffix metadata.
            TypeReferenceV1::Array(inner) => legacy_projection(*inner),
            TypeReferenceV1::Nullable(_) => Err("[generic/type-ref] nullable type unsupported by program JSON".into()),
        }
    }
    legacy_projection(TypeReferenceV1::parse(type_text)?)
}
