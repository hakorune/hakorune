use nyash_rust::ast::{
    BoxMemberGateSiteV1, BoxMethodCompatibilityOriginV1, BoxMethodEntryV1,
    BoxMethodGateSelectionV1, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryRoundtripRowV2,
    BoxMethodProvenanceV1, BoxMethodSourceSelectionV1, Span,
};
use serde_json::{json, Value};

pub(crate) fn encode_method_entry_v2(
    entry: &BoxMethodEntryV1,
    mut encode_declaration: impl FnMut(&nyash_rust::ASTNode) -> Value,
) -> Value {
    json!({
        "key": entry.name(),
        "decl": encode_declaration(entry.declaration()),
        "selected_method_ordinal": entry.site().selected_method_ordinal(),
        "diagnostic_span": encode_span(entry.diagnostic_span()),
        "provenance": encode_provenance(entry.provenance()),
    })
}

fn encode_span(span: nyash_rust::ast::Span) -> Value {
    json!({
        "start": span.start,
        "end": span.end,
        "line": span.line,
        "column": span.column,
    })
}

fn encode_provenance(provenance: &BoxMethodProvenanceV1) -> Value {
    match provenance {
        BoxMethodProvenanceV1::ExplicitSource { selection } => json!({
            "kind": "explicit_source",
            "selection": encode_selection(selection),
        }),
        BoxMethodProvenanceV1::Generated(generated) => match generated {
            BoxMethodGeneratedProvenanceV1::Property {
                property_name,
                selection,
            } => json!({
                "kind": "generated_property",
                "property_name": property_name,
                "selection": encode_selection(selection),
            }),
            BoxMethodGeneratedProvenanceV1::Delegate {
                field_name,
                exposed_name,
                selection,
            } => json!({
                "kind": "generated_delegate",
                "field_name": field_name,
                "exposed_name": exposed_name,
                "selection": encode_selection(selection),
            }),
            BoxMethodGeneratedProvenanceV1::MacroOrImport { generator } => json!({
                "kind": "generated_macro_or_import",
                "generator": generator,
            }),
        },
        BoxMethodProvenanceV1::CompatibilityOnly { origin } => json!({
            "kind": "compatibility_only",
            "origin": match origin {
                BoxMethodCompatibilityOriginV1::LegacyAstConstruction => "legacy_ast_construction",
                BoxMethodCompatibilityOriginV1::LegacyJsonV1 => "legacy_json_v1",
            },
        }),
    }
}

fn encode_selection(selection: &BoxMethodSourceSelectionV1) -> Value {
    match selection {
        BoxMethodSourceSelectionV1::Direct => json!({"kind": "direct"}),
        BoxMethodSourceSelectionV1::SelectedBuildGate { path } => json!({
            "kind": "selected_build_gate",
            "path": path.iter().map(encode_gate).collect::<Vec<_>>(),
        }),
    }
}

fn encode_gate(gate: &nyash_rust::ast::BoxMethodGateSelectionV1) -> Value {
    let gate_site: BoxMemberGateSiteV1 = gate.gate_site();
    json!({
        "gate_site_member_ordinal": gate_site.box_member_ordinal(),
        "branch_member_ordinal": gate.branch_member_ordinal(),
    })
}

pub(crate) fn decode_method_entry_v2(
    value: &Value,
    mut decode_declaration: impl FnMut(&Value) -> Option<nyash_rust::ASTNode>,
) -> Option<BoxMethodInventoryRoundtripRowV2> {
    let name = value.get("key")?.as_str()?.to_owned();
    let ordinal = value
        .get("selected_method_ordinal")?
        .as_u64()?
        .try_into()
        .ok()?;
    let declaration = decode_declaration(value.get("decl")?)?;
    let span = decode_span(value.get("diagnostic_span")?)?;
    let provenance = decode_provenance(value.get("provenance")?)?;
    Some(BoxMethodInventoryRoundtripRowV2::new(
        name,
        ordinal,
        declaration,
        provenance,
        span,
    ))
}

fn decode_span(value: &Value) -> Option<Span> {
    Some(Span::new(
        value.get("start")?.as_u64()?.try_into().ok()?,
        value.get("end")?.as_u64()?.try_into().ok()?,
        value.get("line")?.as_u64()?.try_into().ok()?,
        value.get("column")?.as_u64()?.try_into().ok()?,
    ))
}

fn decode_provenance(value: &Value) -> Option<BoxMethodProvenanceV1> {
    match value.get("kind")?.as_str()? {
        "explicit_source" => Some(BoxMethodProvenanceV1::ExplicitSource {
            selection: decode_selection(value.get("selection")?)?,
        }),
        "generated_property" => Some(BoxMethodProvenanceV1::Generated(
            BoxMethodGeneratedProvenanceV1::Property {
                property_name: value.get("property_name")?.as_str()?.into(),
                selection: decode_selection(value.get("selection")?)?,
            },
        )),
        "generated_delegate" => Some(BoxMethodProvenanceV1::Generated(
            BoxMethodGeneratedProvenanceV1::Delegate {
                field_name: value.get("field_name")?.as_str()?.into(),
                exposed_name: value.get("exposed_name")?.as_str()?.into(),
                selection: decode_selection(value.get("selection")?)?,
            },
        )),
        "generated_macro_or_import" => Some(BoxMethodProvenanceV1::Generated(
            BoxMethodGeneratedProvenanceV1::MacroOrImport {
                generator: value.get("generator")?.as_str()?.into(),
            },
        )),
        "compatibility_only" => Some(BoxMethodProvenanceV1::CompatibilityOnly {
            origin: match value.get("origin")?.as_str()? {
                "legacy_ast_construction" => BoxMethodCompatibilityOriginV1::LegacyAstConstruction,
                "legacy_json_v1" => BoxMethodCompatibilityOriginV1::LegacyJsonV1,
                _ => return None,
            },
        }),
        _ => None,
    }
}

fn decode_selection(value: &Value) -> Option<BoxMethodSourceSelectionV1> {
    match value.get("kind")?.as_str()? {
        "direct" => Some(BoxMethodSourceSelectionV1::Direct),
        "selected_build_gate" => {
            let path = value
                .get("path")?
                .as_array()?
                .iter()
                .map(|gate| {
                    Some(BoxMethodGateSelectionV1::from_parts(
                        BoxMemberGateSiteV1::from_box_member_ordinal(
                            gate.get("gate_site_member_ordinal")?
                                .as_u64()?
                                .try_into()
                                .ok()?,
                        ),
                        gate.get("branch_member_ordinal")?
                            .as_u64()?
                            .try_into()
                            .ok()?,
                    ))
                })
                .collect::<Option<Vec<_>>>()?;
            BoxMethodSourceSelectionV1::selected_build_gate(path).ok()
        }
        _ => None,
    }
}
