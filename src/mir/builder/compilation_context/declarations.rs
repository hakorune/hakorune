use super::*;

#[allow(dead_code)]
impl CompilationContext {
    /// Register a user-defined box (backward compatibility - no fields)
    pub fn register_user_box(&mut self, name: String) {
        self.user_defined_boxes.insert(name.clone(), Vec::new()); // Phase 285LLVM-1.1: Empty fields
        self.user_box_field_decls.insert(name, Vec::new());
    }

    pub fn register_brand_decl(&mut self, name: String, underlying_type_name: String) {
        self.brand_decls.insert(name, underlying_type_name);
    }

    pub fn is_brand_declared(&self, name: &str) -> bool {
        self.brand_decls.contains_key(name)
    }

    /// Phase 285LLVM-1.1: Register a user-defined box with field information
    pub fn register_user_box_with_fields(&mut self, name: String, fields: Vec<String>) {
        self.user_defined_boxes.insert(name.clone(), fields);
        self.user_box_field_decls.insert(name, Vec::new());
    }

    /// Register declared fields for a user box from parser-emitted field surfaces.
    ///
    /// This is the normalization boundary for `fields`, `field_decls`,
    /// `init_fields`, and `weak_fields`: after this point builder code should
    /// read `user_defined_boxes`, `user_box_field_decls`, and
    /// `weak_fields_by_box` instead of recomputing parser-side field views.
    pub fn register_user_box_declared_fields(
        &mut self,
        name: String,
        fields: &[String],
        field_decls: &[FieldDecl],
        init_fields: &[String],
        weak_fields: &[String],
    ) {
        let mut decls = field_decls.to_vec();
        let mut names: Vec<String> = decls.iter().map(|decl| decl.name.clone()).collect();

        for field in fields.iter().chain(init_fields.iter()) {
            if names.contains(field) {
                continue;
            }
            names.push(field.clone());
            decls.push(FieldDecl {
                name: field.clone(),
                declared_type_name: None,
                is_weak: weak_fields.contains(field),
                default_value: None,
            });
        }

        for decl in &mut decls {
            if weak_fields.contains(&decl.name) {
                decl.is_weak = true;
            }
        }

        self.user_defined_boxes.insert(name.clone(), names);
        self.user_box_field_decls.insert(name, decls);
    }

    pub fn register_user_box_with_field_decls(
        &mut self,
        name: String,
        field_decls: Vec<FieldDecl>,
    ) {
        let fields = field_decls.iter().map(|decl| decl.name.clone()).collect();
        self.user_defined_boxes.insert(name.clone(), fields);
        self.user_box_field_decls.insert(name, field_decls);
    }

    pub fn register_record_decl(
        &mut self,
        name: String,
        type_parameters: Vec<String>,
        field_decls: &[FieldDecl],
    ) {
        let fields = field_decls
            .iter()
            .map(|decl| UserBoxFieldDecl {
                name: decl.name.clone(),
                declared_type_name: decl.declared_type_name.clone(),
                is_weak: decl.is_weak,
            })
            .collect();
        self.record_decls.insert(
            name.clone(),
            RecordDecl {
                name: name.clone(),
                type_parameters,
                fields,
            },
        );
        let defaults = field_decls
            .iter()
            .filter_map(|decl| {
                decl.default_value
                    .as_deref()
                    .map(|expr| (decl.name.clone(), expr.clone()))
            })
            .collect::<HashMap<_, _>>();
        if defaults.is_empty() {
            self.record_field_defaults.remove(&name);
        } else {
            self.record_field_defaults.insert(name, defaults);
        }
    }

    pub fn is_record_decl(&self, name: &str) -> bool {
        self.record_decls.contains_key(name)
    }

    pub fn register_enum_decl(
        &mut self,
        name: String,
        type_parameters: Vec<String>,
        variants: Vec<EnumVariantDecl>,
    ) {
        self.enum_decls.insert(
            name,
            EnumDeclLocal {
                type_parameters,
                variants,
            },
        );
    }

    pub fn resolve_enum_variant(
        &self,
        enum_name: &str,
        variant_name: &str,
    ) -> Option<ResolvedEnumVariant<'_>> {
        self.enum_decls.get(enum_name).and_then(|decl| {
            decl.variants
                .iter()
                .enumerate()
                .find(|(_, variant)| variant.name == variant_name)
                .map(|(tag, variant)| ResolvedEnumVariant {
                    tag: tag as u32,
                    decl: variant,
                })
        })
    }

    pub fn enum_decls_for_module_metadata(
        &self,
    ) -> std::collections::BTreeMap<String, MirEnumDecl> {
        self.enum_decls
            .iter()
            .map(|(name, decl)| {
                (
                    name.clone(),
                    MirEnumDecl {
                        type_parameters: decl.type_parameters.clone(),
                        variants: decl
                            .variants
                            .iter()
                            .map(|variant| MirEnumVariantDecl {
                                name: variant.name.clone(),
                                payload_type_name: variant.payload_type_name.clone(),
                            })
                            .collect(),
                    },
                )
            })
            .collect()
    }

    pub fn register_record_local_value(
        &mut self,
        value: ValueId,
        record_name: String,
        fields: Vec<RecordLocalFieldValue>,
    ) {
        self.record_local_values.insert(
            value,
            RecordLocalValue {
                record_name,
                fields,
            },
        );
    }

    pub fn record_local_value(&self, value: ValueId) -> Option<&RecordLocalValue> {
        self.record_local_values.get(&value)
    }

    pub fn propagate_record_local_value(&mut self, src: ValueId, dst: ValueId) {
        if let Some(record) = self.record_local_values.get(&src).cloned() {
            self.record_local_values.insert(dst, record);
        }
    }

    pub fn propagate_record_local_value_from_phi(
        &mut self,
        inputs: &[(crate::mir::BasicBlockId, ValueId)],
        dst: ValueId,
    ) {
        let mut records = inputs
            .iter()
            .filter_map(|(_, value)| self.record_local_values.get(value));
        let Some(first) = records.next().cloned() else {
            return;
        };
        if records.all(|record| {
            record.record_name == first.record_name
                && record.fields.len() == first.fields.len()
                && record.fields.iter().zip(first.fields.iter()).all(|(a, b)| {
                    a.name == b.name
                        && a.declared_type_name == b.declared_type_name
                        && a.value == b.value
                })
        }) {
            self.record_local_values.insert(dst, first);
        }
    }

    pub fn clear_record_local_values(&mut self) {
        self.record_local_values.clear();
    }
}
