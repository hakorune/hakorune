use crate::hako_forward_bridge::{
    self, HakoFutureSpawnInstanceFn, HakoPluginInvokeByNameFn, HakoStringDispatchFn,
};

#[export_name = "nyrt.hako.register_plugin_invoke_by_name"]
pub extern "C" fn nyrt_hako_register_plugin_invoke_by_name(f: HakoPluginInvokeByNameFn) -> i64 {
    hako_forward_bridge::register_plugin_invoke_by_name(Some(f))
}

#[export_name = "nyrt.hako.register_future_spawn_instance"]
pub extern "C" fn nyrt_hako_register_future_spawn_instance(f: HakoFutureSpawnInstanceFn) -> i64 {
    hako_forward_bridge::register_future_spawn_instance(Some(f))
}

#[export_name = "nyrt.hako.register_string_dispatch"]
pub extern "C" fn nyrt_hako_register_string_dispatch(f: HakoStringDispatchFn) -> i64 {
    hako_forward_bridge::register_string_dispatch(Some(f))
}
