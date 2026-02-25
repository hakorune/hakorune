//! 型変換ユーティリティ
//!
//! レジスタ値の読み込み＋型変換チェーンを統一します。

use super::super::*;
use crate::box_trait::NyashBox;
use crate::mir::ValueId;

impl MirInterpreter {
    /// レジスタ値をBox<dyn NyashBox>として読み込む
    ///
    /// # Arguments
    /// * `vid` - 読み込むValueId
    ///
    /// # Returns
    /// * `Result<Box<dyn NyashBox>, VMError>` - 変換済みのBox
    #[inline]
    pub(crate) fn load_as_box(&mut self, vid: ValueId) -> Result<Box<dyn NyashBox>, VMError> {
        Ok(self.reg_load(vid)?.to_nyash_box())
    }

    /// レジスタ値をStringとして読み込む
    ///
    /// # Arguments
    /// * `vid` - 読み込むValueId
    ///
    /// # Returns
    /// * `Result<String, VMError>` - 変換済みのString
    #[inline]
    pub(crate) fn load_as_string(&mut self, vid: ValueId) -> Result<String, VMError> {
        Ok(self.reg_load(vid)?.to_string())
    }

    /// レジスタ値をi64として読み込む
    ///
    /// # Arguments
    /// * `vid` - 読み込むValueId
    ///
    /// # Returns
    /// * `Result<i64, VMError>` - 変換済みのi64
    ///
    /// # Errors
    /// * 値が整数でない場合はエラー
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn load_as_int(&mut self, vid: ValueId) -> Result<i64, VMError> {
        match self.reg_load(vid)? {
            VMValue::Integer(i) => Ok(i),
            other => {
                let type_name = match other {
                    VMValue::Integer(_) => "Integer",
                    VMValue::Float(_) => "Float",
                    VMValue::Bool(_) => "Bool",
                    VMValue::String(_) => "String",
                    VMValue::Void => "Void",
                    VMValue::BoxRef(b) => {
                        return Err(self.err_type_mismatch(
                            "load_as_int",
                            "Integer",
                            &b.type_name(),
                        ))
                    }
                    VMValue::Future(_) => "Future",
                    VMValue::WeakBox(_) => "WeakRef", // Phase 285A0
                };
                Err(self.err_type_mismatch("load_as_int", "Integer", type_name))
            }
        }
    }

    /// レジスタ値をboolとして読み込む
    ///
    /// # Arguments
    /// * `vid` - 読み込むValueId
    ///
    /// # Returns
    /// * `Result<bool, VMError>` - 変換済みのbool
    ///
    /// # Errors
    /// * 値がboolでない場合はエラー
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn load_as_bool(&mut self, vid: ValueId) -> Result<bool, VMError> {
        match self.reg_load(vid)? {
            VMValue::Bool(b) => Ok(b),
            other => {
                let type_name = match other {
                    VMValue::Integer(_) => "Integer",
                    VMValue::Float(_) => "Float",
                    VMValue::Bool(_) => "Bool",
                    VMValue::String(_) => "String",
                    VMValue::Void => "Void",
                    VMValue::BoxRef(b) => {
                        return Err(self.err_type_mismatch("load_as_bool", "Bool", &b.type_name()))
                    }
                    VMValue::Future(_) => "Future",
                    VMValue::WeakBox(_) => "WeakRef", // Phase 285A0
                };
                Err(self.err_type_mismatch("load_as_bool", "Bool", type_name))
            }
        }
    }

    /// 複数のレジスタ値をVec<Box<dyn NyashBox>>として読み込む
    ///
    /// # Arguments
    /// * `vids` - 読み込むValueIdのスライス
    ///
    /// # Returns
    /// * `Result<Vec<Box<dyn NyashBox>>, VMError>` - 変換済みのVec
    #[inline]
    pub(crate) fn load_args_as_boxes(
        &mut self,
        vids: &[ValueId],
    ) -> Result<Vec<Box<dyn NyashBox>>, VMError> {
        vids.iter().map(|vid| self.load_as_box(*vid)).collect()
    }

    /// 複数のレジスタ値をVec<VMValue>として読み込む
    ///
    /// # Arguments
    /// * `vids` - 読み込むValueIdのスライス
    ///
    /// # Returns
    /// * `Result<Vec<VMValue>, VMError>` - 読み込んだVMValueのVec
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn load_args_as_values(
        &mut self,
        vids: &[ValueId],
    ) -> Result<Vec<VMValue>, VMError> {
        vids.iter().map(|vid| self.reg_load(*vid)).collect()
    }
}
