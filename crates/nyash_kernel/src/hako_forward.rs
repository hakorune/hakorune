use crate::hako_forward_bridge::{
    self, HakoFutureSpawnInstanceFn, HakoStringDispatchFn,
};

#[export_name = "nyrt.hako.register_future_spawn_instance"]
pub(crate) extern "C" fn nyrt_hako_register_future_spawn_instance(
    f: HakoFutureSpawnInstanceFn,
) -> i64 {
    hako_forward_bridge::register_future_spawn_instance(Some(f))
}

#[export_name = "nyrt.hako.register_string_dispatch"]
pub(crate) extern "C" fn nyrt_hako_register_string_dispatch(f: HakoStringDispatchFn) -> i64 {
    hako_forward_bridge::register_string_dispatch(Some(f))
}
