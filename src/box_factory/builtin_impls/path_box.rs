/*!
 * Builtin PathBox Implementation
 *
 * Thin builtin wrapper around provider-lock PathService.
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;
use crate::boxes::path_box::PathBox;
use crate::providers::ring1::path::Ring1PathService;
use crate::runtime::provider_lock;
use std::sync::Arc;

/// Create builtin PathBox instance.
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    if provider_lock::get_pathbox_provider().is_none() {
        let _ = provider_lock::set_pathbox_provider(Arc::new(Ring1PathService::new()));
    }
    let path_box = PathBox::try_new().map_err(|e| RuntimeError::InvalidOperation { message: e })?;
    Ok(Box::new(path_box))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::ring1::path::Ring1PathService;
    use crate::runtime::provider_lock;
    use std::sync::Arc;

    #[test]
    fn test_builtin_path_box_creation() {
        let _ = provider_lock::set_pathbox_provider(Arc::new(Ring1PathService::new()));
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<PathBox>().is_some());
    }
}
