//! Final-value transport; source identity travels with values through freshening.
//! This product carries no publication permission. The active loop scope owns it.
use crate::mir::ValueId;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum CoreLoopFinalValuesV1 {
    Raw(Vec<(String, ValueId)>),
}

impl From<Vec<(String, ValueId)>> for CoreLoopFinalValuesV1 {
    fn from(rows: Vec<(String, ValueId)>) -> Self {
        Self::Raw(rows)
    }
}
impl CoreLoopFinalValuesV1 {
    pub(in crate::mir::builder) fn raw_rows(&self) -> Result<&[(String, ValueId)], String> {
        match self {
            Self::Raw(rows) => Ok(rows),
        }
    }
    pub(in crate::mir::builder) fn raw_rows_mut(
        &mut self,
    ) -> Result<&mut Vec<(String, ValueId)>, String> {
        match self {
            Self::Raw(rows) => Ok(rows),
        }
    }
    pub(in crate::mir::builder) fn len(&self) -> usize {
        match self {
            Self::Raw(rows) => rows.len(),
        }
    }
    pub(in crate::mir::builder) fn diagnostic_values(
        &self,
    ) -> Box<dyn Iterator<Item = (&str, ValueId)> + '_> {
        match self {
            Self::Raw(rows) => Box::new(rows.iter().map(|(label, value)| (label.as_str(), *value))),
        }
    }
    pub(in crate::mir::builder) fn remap_values(
        &mut self,
        mut remap: impl FnMut(ValueId) -> ValueId,
    ) {
        match self {
            Self::Raw(rows) => {
                for (_, value) in rows {
                    *value = remap(*value);
                }
            }
        }
    }
}
