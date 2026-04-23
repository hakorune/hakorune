/*!
 * Type Registry (Tier-0 雛形)
 *
 * 目的:
 * - TypeId → TypeBox 参照の最小インターフェースを用意（現時点では未実装・常に未登録）。
 * - VM/JIT 実装が存在を前提に呼び出しても no-op/fallback できる状態にする。
 *
 * スロット番号の方針（注釈）
 * - ここで定義する `slot` は「VTable 用の仮想メソッドID」です。VM/JIT の内部ディスパッチ最適化
 *   と、Builtin Box の高速経路（fast path）に使われます。
 * - HostAPI（プラグインのネイティブ関数呼び出し）で用いるメソッド番号空間とは独立です。
 *   HostAPI 側は TLV で型付き引数を渡し、プラグイン実装側の関数テーブルにマップされます。
 *   そのため重複しても問題ありません（互いに衝突しない設計）。
 * - 慣例として以下の帯域を利用します（将来の整理用の目安）：
 *   - 0..=3: ユニバーサルスロット（toString/type/equal/copy 相当）
 *   - 100..: Array 系（get/set/len ほか拡張）
 *   - 200..: Map 系（size/len/has/get/set/delete ほか拡張）
 *   - 300..: String 系（len/substring/concat/indexOf/replace/trim/toUpper/toLower）
 *   - 400..: Console 系（log/warn/error/clear）
 *   - 500..: Buffer 系（typed binary I/O。現在は VM handler dispatch を SSOT とし、slot は未固定）
 *   - 600..: File 系（readBytes/writeBytes。現在は VM handler dispatch を SSOT とし、slot は未固定）
 *
 * Phase 124: Primitive type support
 * - Primitive types (String, Integer, Array) are now registered with same slot numbers as their Box variants
 * - This enables unified dispatch for both VMValue::String and VMValue::BoxRef(StringBox)
 */

use super::core_box_ids::{CoreBoxId, CoreMethodId};
use super::type_box_abi::{MethodEntry, TypeBox};
use std::collections::HashSet;
use std::sync::OnceLock;

fn core_method_entries_for_box(box_id: CoreBoxId) -> Vec<MethodEntry> {
    CoreMethodId::iter()
        .filter(|method_id| method_id.box_id() == box_id)
        .filter_map(|method_id| {
            method_id.vtable_slot().map(|slot| MethodEntry {
                name: method_id.name(),
                arity: method_id.arity() as u8,
                slot,
            })
        })
        .collect()
}

fn core_method_entries_for_box_signatures(
    box_id: CoreBoxId,
    allowed: &[(&'static str, usize)],
) -> Vec<MethodEntry> {
    let core = core_method_entries_for_box(box_id);
    core.into_iter()
        .filter(|entry| {
            allowed
                .iter()
                .any(|(name, arity)| entry.name == *name && entry.arity as usize == *arity)
        })
        .collect()
}

fn merge_method_entries(
    mut entries: Vec<MethodEntry>,
    extras: &[MethodEntry],
) -> &'static [MethodEntry] {
    entries.extend_from_slice(extras);
    let mut seen = HashSet::new();
    entries.retain(|method| seen.insert((method.name, method.arity)));
    Box::leak(entries.into_boxed_slice())
}

fn array_surface_method_entries() -> Vec<MethodEntry> {
    let mut entries = Vec::new();
    for spec in crate::boxes::array::ARRAY_SURFACE_METHODS {
        entries.push(MethodEntry {
            name: spec.canonical,
            arity: spec.arity,
            slot: spec.slot,
        });
        for alias in spec.aliases {
            entries.push(MethodEntry {
                name: *alias,
                arity: spec.arity,
                slot: spec.slot,
            });
        }
    }
    entries
}

fn string_surface_method_entries() -> Vec<MethodEntry> {
    let mut entries = Vec::new();
    for spec in crate::boxes::basic::STRING_SURFACE_METHODS {
        entries.push(MethodEntry {
            name: spec.canonical,
            arity: spec.arity,
            slot: spec.slot,
        });
        for alias in spec.aliases {
            entries.push(MethodEntry {
                name: *alias,
                arity: spec.arity,
                slot: spec.slot,
            });
        }
    }
    entries
}

fn map_surface_method_entries() -> Vec<MethodEntry> {
    let mut entries = Vec::new();
    for spec in crate::boxes::MAP_SURFACE_METHODS {
        entries.push(MethodEntry {
            name: spec.canonical,
            arity: spec.arity,
            slot: spec.slot,
        });
        for alias in spec.aliases {
            entries.push(MethodEntry {
                name: *alias,
                arity: spec.arity,
                slot: spec.slot,
            });
        }
    }
    entries
}

const ARRAY_METHOD_EXTRAS: &[MethodEntry] = &[
    MethodEntry {
        name: "clear",
        arity: 0,
        slot: 105,
    },
    // P1: contains/indexOf/join
    MethodEntry {
        name: "contains",
        arity: 1,
        slot: 106,
    },
    MethodEntry {
        name: "indexOf",
        arity: 1,
        slot: 107,
    },
    MethodEntry {
        name: "join",
        arity: 1,
        slot: 108,
    },
    // P2: sort/reverse/slice
    MethodEntry {
        name: "sort",
        arity: 0,
        slot: 109,
    },
    MethodEntry {
        name: "reverse",
        arity: 0,
        slot: 110,
    },
];

// --- ConsoleBox --- (WASM v2 unified dispatch 用の雛形)
// 400: log(..), 401: warn(..), 402: error(..), 403: clear()
const CONSOLE_METHOD_EXTRAS: &[MethodEntry] = &[
    MethodEntry {
        name: "warn",
        arity: 1,
        slot: 401,
    },
    MethodEntry {
        name: "clear",
        arity: 0,
        slot: 403,
    },
];

static ARRAYBOX_TB: OnceLock<TypeBox> = OnceLock::new();
static MAPBOX_TB: OnceLock<TypeBox> = OnceLock::new();
static STRINGBOX_TB: OnceLock<TypeBox> = OnceLock::new();
static CONSOLEBOX_TB: OnceLock<TypeBox> = OnceLock::new();

fn arraybox_typebox() -> &'static TypeBox {
    ARRAYBOX_TB.get_or_init(|| {
        let mut core = core_method_entries_for_box(CoreBoxId::Array);
        core.extend(array_surface_method_entries());
        let methods = merge_method_entries(core, ARRAY_METHOD_EXTRAS);
        TypeBox::new_with("ArrayBox", methods)
    })
}

fn mapbox_typebox() -> &'static TypeBox {
    MAPBOX_TB.get_or_init(|| {
        let mut core = core_method_entries_for_box(CoreBoxId::Map);
        core.extend(map_surface_method_entries());
        let methods = merge_method_entries(core, &[]);
        TypeBox::new_with("MapBox", methods)
    })
}

fn stringbox_typebox() -> &'static TypeBox {
    STRINGBOX_TB.get_or_init(|| {
        let mut core = core_method_entries_for_box(CoreBoxId::String);
        core.extend(string_surface_method_entries());
        let methods = merge_method_entries(core, &[]);
        TypeBox::new_with("StringBox", methods)
    })
}

fn consolebox_typebox() -> &'static TypeBox {
    CONSOLEBOX_TB.get_or_init(|| {
        let core = core_method_entries_for_box(CoreBoxId::Console);
        let methods = merge_method_entries(core, CONSOLE_METHOD_EXTRAS);
        TypeBox::new_with("ConsoleBox", methods)
    })
}

// --- InstanceBox ---
// Representative methods exposed via unified slots for field access and diagnostics.
// 1: getField(name)
// 2: setField(name, value)
// 3: has(name)
// 4: size()
const INSTANCE_METHODS: &[MethodEntry] = &[
    MethodEntry {
        name: "getField",
        arity: 1,
        slot: 1,
    },
    MethodEntry {
        name: "setField",
        arity: 2,
        slot: 2,
    },
    MethodEntry {
        name: "has",
        arity: 1,
        slot: 3,
    },
    MethodEntry {
        name: "size",
        arity: 0,
        slot: 4,
    },
];
static INSTANCEBOX_TB: TypeBox = TypeBox::new_with("InstanceBox", INSTANCE_METHODS);

// --- Phase 124: Primitive Type Support ---
// Primitive types (String, Integer, Array) share the same slot numbers as their Box variants
// This enables unified dispatch for both primitives and boxes

const PRIMITIVE_STRING_EXTRAS: &[MethodEntry] = &[];

const PRIMITIVE_ARRAY_ALLOWED_SIGNATURES: &[(&str, usize)] =
    &[("get", 1), ("length", 0), ("push", 1)];
const PRIMITIVE_ARRAY_EXTRAS: &[MethodEntry] = &[
    MethodEntry {
        name: "set",
        arity: 2,
        slot: 101,
    },
    MethodEntry {
        name: "len",
        arity: 0,
        slot: 102,
    },
];

static PRIMITIVE_STRING_TB: OnceLock<TypeBox> = OnceLock::new();
static PRIMITIVE_ARRAY_TB: OnceLock<TypeBox> = OnceLock::new();

fn primitive_string_typebox() -> &'static TypeBox {
    PRIMITIVE_STRING_TB.get_or_init(|| {
        let methods =
            merge_method_entries(string_surface_method_entries(), PRIMITIVE_STRING_EXTRAS);
        TypeBox::new_with("String", methods)
    })
}

fn primitive_array_typebox() -> &'static TypeBox {
    PRIMITIVE_ARRAY_TB.get_or_init(|| {
        let core = core_method_entries_for_box_signatures(
            CoreBoxId::Array,
            PRIMITIVE_ARRAY_ALLOWED_SIGNATURES,
        );
        let methods = merge_method_entries(core, PRIMITIVE_ARRAY_EXTRAS);
        TypeBox::new_with("Array", methods)
    })
}

/// 型名から TypeBox を解決（雛形）。現在は常に None。
pub fn resolve_typebox_by_name(type_name: &str) -> Option<&'static TypeBox> {
    match type_name {
        "MapBox" => Some(mapbox_typebox()),
        "ArrayBox" => Some(arraybox_typebox()),
        "StringBox" => Some(stringbox_typebox()),
        "ConsoleBox" => Some(consolebox_typebox()),
        "InstanceBox" => Some(&INSTANCEBOX_TB),
        // Phase 124: Primitive types
        "String" => Some(primitive_string_typebox()),
        "Array" => Some(primitive_array_typebox()),
        _ => None,
    }
}

/// 型名・メソッド名・アリティからスロットを解決（雛形）
pub fn resolve_slot_by_name(type_name: &str, method: &str, arity: usize) -> Option<u16> {
    let tb = resolve_typebox_by_name(type_name)?;
    let ar = arity as u8;
    for m in tb.methods {
        if m.name == method && m.arity == ar {
            return Some(m.slot);
        }
    }
    None
}

/// Return list of known methods for a type (names only) for diagnostics.
pub fn known_methods_for(type_name: &str) -> Option<Vec<&'static str>> {
    let tb = resolve_typebox_by_name(type_name)?;
    let mut v: Vec<&'static str> = tb.methods.iter().map(|m| m.name).collect();
    v.sort();
    v.dedup();
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_method_vtable_slots_match_registry() {
        for method_id in CoreMethodId::iter() {
            let Some(expected_slot) = method_id.vtable_slot() else {
                continue;
            };
            let type_name = method_id.box_id().name();
            let resolved = resolve_slot_by_name(type_name, method_id.name(), method_id.arity());
            assert_eq!(
                resolved,
                Some(expected_slot),
                "vtable slot mismatch: {}.{}({})",
                type_name,
                method_id.name(),
                method_id.arity()
            );
        }
    }

    #[test]
    fn test_string_contains_slot_resolves_for_primitive_and_box() {
        assert_eq!(resolve_slot_by_name("String", "contains", 1), Some(309));
        assert_eq!(resolve_slot_by_name("StringBox", "contains", 1), Some(309));
    }

    #[test]
    fn test_string_trim_slot_resolves_for_primitive_and_box() {
        assert_eq!(resolve_slot_by_name("String", "trim", 0), Some(305));
        assert_eq!(resolve_slot_by_name("StringBox", "trim", 0), Some(305));
    }

    #[test]
    fn test_map_slots_resolve_from_surface_catalog() {
        for spec in crate::boxes::MAP_SURFACE_METHODS {
            assert_eq!(
                resolve_slot_by_name("MapBox", spec.canonical, spec.arity as usize),
                Some(spec.slot),
                "MapBox.{}({}) should resolve to slot {}",
                spec.canonical,
                spec.arity,
                spec.slot
            );
            for alias in spec.aliases {
                assert_eq!(
                    resolve_slot_by_name("MapBox", alias, spec.arity as usize),
                    Some(spec.slot),
                    "MapBox.{}({}) alias should resolve to slot {}",
                    alias,
                    spec.arity,
                    spec.slot
                );
            }
        }
    }
}
