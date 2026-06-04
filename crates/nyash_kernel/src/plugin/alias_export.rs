#[macro_export]
macro_rules! nyash_export_fn_alias {
    ($fn_name:ident, $export_name:literal, ($($arg:ident : $ty:ty),* $(,)?), -> $ret:ty, $body:block) => {
        #[export_name = $export_name]
        pub extern "C" fn $fn_name($($arg : $ty),*) -> $ret $body
    };
}

#[macro_export]
macro_rules! nyash_export_i64_alias {
    ($fn_name:ident, $export_name:literal, ($($arg:ident : $ty:ty),* $(,)?), $body:block) => {
        $crate::nyash_export_fn_alias!($fn_name, $export_name, ($($arg : $ty),*), -> i64, $body);
    };
}
