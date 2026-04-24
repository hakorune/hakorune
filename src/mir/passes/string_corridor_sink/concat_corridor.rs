#[allow(unused_imports)]
use super::*;

#[path = "concat_corridor_apply.rs"]
mod apply;
#[path = "concat_corridor_collect.rs"]
mod collect;

pub(crate) use apply::apply_concat_corridor_plans;
pub(crate) use collect::collect_concat_corridor_plans;

pub const SUBSTRING_LEN_EXTERN: &str = "nyash.string.substring_len_hii";
pub const SUBSTRING_CONCAT3_EXTERN: &str = "nyash.string.substring_concat3_hhhii";
pub const INSERT_HSI_EXTERN: &str = "nyash.string.insert_hsi";
