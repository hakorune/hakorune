/*! 📟 ConsoleBox - コンソール出力Box
 *
 * ## 📝 概要
 * Webブラウザのコンソール機能を統合したBox。
 * WASM環境ではブラウザコンソール、ネイティブ環境では標準出力。
 *
 * ## 🛠️ 利用可能メソッド
 * - `log(message)` - 通常のメッセージ出力
 * - `println(message)` - `log` のエイリアス（ユーザー向け sugar）
 * - `warn(message)` - 警告メッセージ出力
 * - `error(message)` - エラーメッセージ出力
 * - `clear()` - コンソール画面クリア
 *
 * ## Phase 122: println / log の統一
 *
 * `println` は `log` の完全なエイリアスです。内部的には同じ slot 400 を使用します。
 * ユーザーコードでは `println` を使用することを推奨しますが、`log` も同様に動作します。
 *
 * ## 💡 使用例
 * ```nyash
 * local console
 * console = new ConsoleBox()
 *
 * console.log("Hello, Nyash!")           // 通常ログ
 * console.warn("This is a warning")      // 警告
 * console.error("Something went wrong")  // エラー
 * console.clear()                        // クリア
 *
 * // デバッグ用途
 * local value
 * value = 42
 * console.log("Debug: value = " + value.toString())
 * ```
 *
 * ## 🌐 環境別動作
 * - **WASM環境**: ブラウザの開発者ツールコンソールに出力
 * - **ネイティブ環境**: ターミナル標準出力にプレフィックス付きで出力
 *
 * ## 🔍 デバッグ活用
 * ```nyash
 * // エラーハンドリング
 * if (error_condition) {
 *     console.error("Critical error occurred!")
 *     return null
 * }
 *
 * // 実行トレース
 * console.log("Function start")
 * // 処理...
 * console.log("Function end")
 * ```
 */

use crate::box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox};
use std::any::Any;
use std::fmt::Display;

/// ConsoleBox メソッド実装マクロ
/// WASM/非WASM環境で異なるメソッド実装を統一化
macro_rules! define_console_impl {
    (
        log: $log_impl:expr,
        warn: $warn_impl:expr,
        error: $error_impl:expr,
        clear: $clear_impl:expr,
        fmt_desc: $fmt_desc:expr
    ) => {
        impl ConsoleBox {
            pub fn new() -> Self {
                Self {
                    base: BoxBase::new(),
                }
            }

            pub fn log(&self, message: &str) {
                $log_impl(message);
            }

            pub fn println(&self, message: &str) {
                self.log(message);
            }

            pub fn warn(&self, message: &str) {
                $warn_impl(message);
            }

            pub fn error(&self, message: &str) {
                $error_impl(message);
            }

            pub fn clear(&self) {
                $clear_impl();
            }
        }

        impl BoxCore for ConsoleBox {
            fn box_id(&self) -> u64 {
                self.base.id
            }

            fn parent_type_id(&self) -> Option<std::any::TypeId> {
                self.base.parent_type_id
            }

            fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $fmt_desc)
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        impl NyashBox for ConsoleBox {
            fn to_string_box(&self) -> StringBox {
                StringBox::new($fmt_desc)
            }

            fn equals(&self, other: &dyn NyashBox) -> BoolBox {
                BoolBox::new(other.as_any().is::<ConsoleBox>())
            }

            fn type_name(&self) -> &'static str {
                "ConsoleBox"
            }

            fn clone_box(&self) -> Box<dyn NyashBox> {
                Box::new(self.clone())
            }

            fn share_box(&self) -> Box<dyn NyashBox> {
                self.clone_box()
            }
        }

        impl Display for ConsoleBox {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.fmt_box(f)
            }
        }
    };
}

// 🌐 Browser console access Box
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
pub struct ConsoleBox {
    base: BoxBase,
}

#[cfg(target_arch = "wasm32")]
define_console_impl!(
    log: |msg: &str| { web_sys::console::log_1(&msg.into()); },
    warn: |msg: &str| { web_sys::console::warn_1(&msg.into()); },
    error: |msg: &str| { web_sys::console::error_1(&msg.into()); },
    clear: || { web_sys::console::clear(); },
    fmt_desc: "[ConsoleBox - Browser Console Interface]"
);

// Non-WASM版 - モックアップ実装
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct ConsoleBox {
    base: BoxBase,
}

#[cfg(not(target_arch = "wasm32"))]
define_console_impl!(
    log: |msg: &str| { println!("[Console LOG] {}", msg); },
    warn: |msg: &str| { println!("[Console WARN] {}", msg); },
    error: |msg: &str| { println!("[Console ERROR] {}", msg); },
    clear: || { println!("[Console CLEAR]"); },
    fmt_desc: "[ConsoleBox - Mock Implementation]"
);
