/*!
 * Builtin NullBox fallback.
 *
 * `null` remains a surface/compat spelling. The runtime no-value meaning is
 * canonicalized on `Void`, but builtin construction still needs a thin NullBox
 * route for legacy/plugin-free paths.
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;

/// Create builtin NullBox instance for fallback/runtime-compat paths.
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    Ok(Box::new(crate::boxes::null_box::NullBox::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boxes::null_box::NullBox;

    #[test]
    fn test_builtin_null_box_creation() {
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<NullBox>().is_some());
    }
}
