use super::*;
use crate::box_trait::NyashBox;

pub(super) fn try_handle_map_box(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    let recv = this.reg_load(box_val)?;
    let recv_box_any: Box<dyn NyashBox> = match recv.clone() {
        VMValue::BoxRef(b) => b.share_box(),
        other => other.to_nyash_box(),
    };
    if let Some(mb) = recv_box_any
        .as_any()
        .downcast_ref::<crate::boxes::map_box::MapBox>()
    {
        match method {
            "birth" => {
                // No-op constructor init for MapBox
                this.write_void(dst);
                return Ok(true);
            }
            // Field bridge: treat getField/setField as get/set with string key
            "getField" => {
                this.validate_args_exact("MapBox.getField", args, 1)?;
                let k_vm = this.reg_load(args[0])?;
                // Field access expects a String key; otherwise return a stable tag.
                if !matches!(k_vm, VMValue::String(_)) {
                    this.write_result(
                        dst,
                        VMValue::String("[map/bad-key] field name must be string".to_string()),
                    );
                    return Ok(true);
                }
                let k = this.load_as_box(args[0])?;
                let ret = mb.get(k);
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "setField" => {
                this.validate_args_exact("MapBox.setField", args, 2)?;
                let k_vm = this.reg_load(args[0])?;
                if !matches!(k_vm, VMValue::String(_)) {
                    this.write_result(
                        dst,
                        VMValue::String("[map/bad-key] field name must be string".to_string()),
                    );
                    return Ok(true);
                }
                let k = this.load_as_box(args[0])?;
                let v = this.load_as_box(args[1])?;
                let ret = mb.set(k, v);
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "set" => {
                this.validate_args_exact("MapBox.set", args, 2)?;
                let k_vm = this.reg_load(args[0])?;
                if !matches!(k_vm, VMValue::String(_)) {
                    this.write_result(
                        dst,
                        VMValue::String("[map/bad-key] key must be string".to_string()),
                    );
                    return Ok(true);
                }
                let k = this.load_as_box(args[0])?;
                let v = this.load_as_box(args[1])?;
                let ret = mb.set(k, v);
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "get" => {
                this.validate_args_exact("MapBox.get", args, 1)?;
                let k_vm = this.reg_load(args[0])?;
                if !matches!(k_vm, VMValue::String(_)) {
                    this.write_result(
                        dst,
                        VMValue::String("[map/bad-key] key must be string".to_string()),
                    );
                    return Ok(true);
                }
                let k = this.load_as_box(args[0])?;
                let ret = mb.get(k);
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "has" => {
                this.validate_args_exact("MapBox.has", args, 1)?;
                let k = this.load_as_box(args[0])?;
                let ret = mb.has(k);
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "delete" => {
                this.validate_args_exact("MapBox.delete", args, 1)?;
                let k_vm = this.reg_load(args[0])?;
                if !matches!(k_vm, VMValue::String(_)) {
                    this.write_result(
                        dst,
                        VMValue::String("[map/bad-key] key must be string".to_string()),
                    );
                    return Ok(true);
                }
                let k = this.load_as_box(args[0])?;
                let ret = mb.delete(k);
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "len" | "length" | "size" => {
                let ret = mb.size();
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "keys" => {
                let ret = mb.keys();
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "values" => {
                let ret = mb.values();
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "clear" => {
                // Reset map to empty; return a neutral value
                let ret = mb.clear();
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}
