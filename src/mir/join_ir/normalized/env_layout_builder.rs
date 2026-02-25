//! Environment Layout Builder
//!
//! Builds environment layouts for normalized functions.
//!
//! ## Responsibilities
//! - Build env layouts from function parameters
//! - Create default env layouts
//! - Manage env field construction

use crate::mir::join_ir::JoinFunction;

use super::{EnvField, EnvLayout};

/// Environment Layout Builder
///
/// Constructs environment layouts for normalized functions.
///
/// The env layout represents the closure environment for a function,
/// mapping parameter names to ValueIds.
pub struct EnvLayoutBuilder;

impl EnvLayoutBuilder {
    /// Build env layout from function parameters
    ///
    /// # Arguments
    /// * `func` - JoinFunction to extract parameters from
    /// * `layout_id` - Unique ID for the layout
    ///
    /// # Returns
    /// EnvLayout with fields named "field0", "field1", etc.
    ///
    /// # Example
    /// ```ignore
    /// let layout = EnvLayoutBuilder::build_from_params(&func, 0);
    /// assert_eq!(layout.fields.len(), func.params.len());
    /// ```
    pub fn build_from_params(func: &JoinFunction, layout_id: u32) -> EnvLayout {
        EnvLayout {
            id: layout_id,
            fields: func
                .params
                .iter()
                .enumerate()
                .map(|(idx, vid)| EnvField {
                    name: format!("field{}", idx),
                    ty: None,
                    value_id: Some(*vid),
                })
                .collect(),
        }
    }

    /// Build a default empty env layout
    ///
    /// # Arguments
    /// * `layout_id` - Unique ID for the layout
    ///
    /// # Returns
    /// Empty EnvLayout
    pub fn build_default(layout_id: u32) -> EnvLayout {
        EnvLayout {
            id: layout_id,
            fields: Vec::new(),
        }
    }

    /// Build env layout from explicit fields
    ///
    /// # Arguments
    /// * `layout_id` - Unique ID for the layout
    /// * `fields` - Vector of env fields
    ///
    /// # Returns
    /// EnvLayout with the specified fields
    pub fn build_from_fields(layout_id: u32, fields: Vec<EnvField>) -> EnvLayout {
        EnvLayout { id: layout_id, fields }
    }
}

// Re-export for backward compatibility
#[allow(deprecated)]
pub use self::EnvLayoutBuilder as EnvLayoutBuilderBox;

/// Legacy helper: Build env layout with minimal field naming
#[deprecated(note = "Use EnvLayoutBuilder::build_from_params instead")]
pub fn build_env_layout_from_params(func: &JoinFunction, layout_id: u32) -> EnvLayout {
    EnvLayoutBuilder::build_from_params(func, layout_id)
}

/// Legacy helper: Create default empty env layout
#[deprecated(note = "Use EnvLayoutBuilder::build_default instead")]
pub fn build_default_env_layout(layout_id: u32) -> EnvLayout {
    EnvLayoutBuilder::build_default(layout_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ValueId;

    #[test]
    fn test_build_from_params() {
        let func = JoinFunction::new(
            crate::mir::join_ir::JoinFuncId::new(0),
            "test".to_string(),
            vec![ValueId(1), ValueId(2), ValueId(3)],
        );

        let layout = EnvLayoutBuilder::build_from_params(&func, 0);

        assert_eq!(layout.id, 0);
        assert_eq!(layout.fields.len(), 3);
        assert_eq!(layout.fields[0].name, "field0");
        assert_eq!(layout.fields[0].value_id, Some(ValueId(1)));
        assert_eq!(layout.fields[1].name, "field1");
        assert_eq!(layout.fields[1].value_id, Some(ValueId(2)));
        assert_eq!(layout.fields[2].name, "field2");
        assert_eq!(layout.fields[2].value_id, Some(ValueId(3)));
    }

    #[test]
    fn test_build_default() {
        let layout = EnvLayoutBuilder::build_default(42);

        assert_eq!(layout.id, 42);
        assert_eq!(layout.fields.len(), 0);
    }

    #[test]
    fn test_build_from_fields() {
        let fields = vec![
            EnvField {
                name: "x".to_string(),
                ty: None,
                value_id: Some(ValueId(10)),
            },
            EnvField {
                name: "y".to_string(),
                ty: None,
                value_id: Some(ValueId(20)),
            },
        ];

        let layout = EnvLayoutBuilder::build_from_fields(1, fields);

        assert_eq!(layout.id, 1);
        assert_eq!(layout.fields.len(), 2);
    }
}
