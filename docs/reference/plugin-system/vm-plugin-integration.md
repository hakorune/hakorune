# VM Plugin Integration仕様書

## 🎯 概要

NyashのVMバックエンドとプラグインシステム（BID-FFI v1）の統合に関する技術仕様。Everything is Box哲学に基づき、**すべてのBox型（ビルトイン、ユーザー定義、プラグイン）**をVMで統一的に扱えるようにする。

> Status note: 初期案にあったスコープ終了専用の lifecycle tracker は現行実装から削除済みです。現在のプラグイン lifecycle owner は `PluginHandleInner` Drop / `finalize_now()` / `shutdown_plugins_v2()` です。

## ⚠️ **現在のVM実装の重大な問題**

1. **ユーザー定義Box未対応** - NewBoxで文字列を返すだけ
2. **birth/finiライフサイクル欠落** - コンストラクタ・デストラクタが呼ばれない
3. **メソッド呼び出しハードコード** - 新メソッド追加が困難

これらを解決し、インタープリターと同等の統一処理を実現する。

## 🏗️ アーキテクチャ

### 統一Box管理モデル

```
┌─────────────────────────────────────────────────┐
│                  Nyash VM                       │
├─────────────────────────────────────────────────┤
│  VMValue                                        │
│  ├─ Integer(i64)     ← 基本型は直接保持       │
│  ├─ String(String)                             │
│  ├─ Bool(bool)                                 │
│  └─ BoxRef(Arc<dyn NyashBox>) ← 複雑型全般    │
├─────────────────────────────────────────────────┤
│  統一Box管理層                                  │
│  ├─ BoxFactory       : 統一Box作成             │
│  ├─ PluginHandleInner: プラグインfini管理      │
│  └─ MethodDispatcher : 統一メソッド呼び出し    │
├─────────────────────────────────────────────────┤
│  変換レイヤー                                   │
│  ├─ to_nyash_box()   : VMValue → Box          │
│  └─ from_nyash_box() : Box → VMValue          │
├─────────────────────────────────────────────────┤
│  プラグインローダー (PluginLoaderV2)           │
│  └─ BID-FFI v1プロトコルで通信                │
└─────────────────────────────────────────────────┘
```

### VM構造体の完全形

```rust
pub struct VM {
    // 既存フィールド
    registers: HashMap<RegisterId, VMValue>,
    memory: HashMap<MemoryLocation, VMValue>,
    
    // 統一Box管理（新規）
    box_factory: Arc<BoxFactory>,           // 統一Box作成
    plugin_loader: Option<Arc<PluginLoaderV2>>, // プラグイン
    box_declarations: Arc<RwLock<HashMap<String, BoxDeclaration>>>, // ユーザー定義Box
}
```

## 📊 VMValue拡張仕様

### 型定義

```rust
pub enum VMValue {
    // 基本型（既存）
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Future(FutureBox),
    Void,
    
    // 拡張型（新規）
    BoxRef(Arc<dyn NyashBox>),
}
```

### 変換規則

#### NyashBox → VMValue

1. **基本型の最適化**
   - IntegerBox → VMValue::Integer（値を直接保持）
   - StringBox → VMValue::String（値を直接保持）
   - BoolBox → VMValue::Bool（値を直接保持）

2. **複雑型の参照保持**
   - PluginBoxV2 → VMValue::BoxRef
   - ユーザー定義Box → VMValue::BoxRef
   - その他のBox → VMValue::BoxRef

#### VMValue → NyashBox

1. **基本型の再Box化**
   - VMValue::Integer → IntegerBox::new()
   - VMValue::String → StringBox::new()
   - VMValue::Bool → BoolBox::new()

2. **参照型のクローン**
   - VMValue::BoxRef → Arc::clone_box()

## 🔄 MIR命令の処理

### NewBox命令の統一実装

```rust
MirInstruction::NewBox { dst, box_type, args } => {
    // 🌟 統一Box作成プロセス
    
    // Step 1: 引数を評価してNyashBoxに変換
    let nyash_args: Vec<Box<dyn NyashBox>> = args.iter()
        .map(|id| self.get_value(*id)?.to_nyash_box())
        .collect::<Result<Vec<_>, _>>()?;
    
    // Step 2: BoxFactory経由で統一作成
    let new_box = self.box_factory.create_box(box_type, &nyash_args)?;
    
    // Step 3: birth実行（ユーザー定義Boxの場合）
    if let Some(instance) = new_box.as_any().downcast_ref::<InstanceBox>() {
        // birthコンストラクタを検索
        let birth_key = format!("birth/{}", args.len());
        
        if let Some(box_decl) = self.box_declarations.read().unwrap().get(&instance.class_name) {
            if let Some(constructor) = box_decl.constructors.get(&birth_key) {
                // birthメソッドを実行
                self.push_scope(); // 新しいスコープ
                self.set_variable("me", new_box.clone()); // me をバインド
                
                // コンストラクタ本体を実行
                let result = self.execute_constructor(constructor, nyash_args)?;
                
                self.pop_scope(); // スコープ終了
            }
        }
    }
    
    // Step 4: プラグインBoxのbirth実行
    #[cfg(all(feature = "plugins", not(target_arch = "wasm32")))]
    if new_box.as_any().downcast_ref::<PluginBoxV2>().is_some() {
        // プラグインのbirthは既にcreate_box内で実行済み
    }
    
    // Step 5: fini owner
    // PluginBoxV2 の fini は PluginHandleInner Drop / finalize_now() /
    // shutdown_plugins_v2() が担当する。
    
    // Step 6: VMValueに変換して格納
    let vm_value = VMValue::from_nyash_box(new_box);
    self.set_value(*dst, vm_value);
}
```

### BoxCall命令の統一処理

```rust
MirInstruction::BoxCall { dst, box_val, method, args, effects } => {
    let box_vm_value = self.get_value(*box_val)?;
    
    // 統一的なメソッド呼び出し
    let result = match &box_vm_value {
        // 基本型の最適化パス
        VMValue::String(s) => {
            self.call_string_method_optimized(s, method, args)?
        },
        VMValue::Integer(i) => {
            self.call_integer_method_optimized(i, method, args)?
        },
        
        // BoxRef経由の汎用パス
        VMValue::BoxRef(arc_box) => {
            let nyash_args = convert_args_to_nyash(args);
            self.call_box_method_generic(arc_box.as_ref(), method, nyash_args)?
        },
        
        _ => return Err(VMError::TypeError("Not a box type"))
    };
    
    if let Some(dst_id) = dst {
        self.set_value(*dst_id, result);
    }
}
```

### ExternCall命令の実装

```rust
MirInstruction::ExternCall { dst, iface_name, method_name, args, effects } => {
    match (iface_name.as_str(), method_name.as_str()) {
        // プラグインBox作成
        ("plugin", "new") => {
            let box_type = self.get_value(args[0])?.to_string();
            let ctor_args = self.convert_args_to_nyash(&args[1..])?;
            
            if let Some(loader) = &self.plugin_loader {
                let plugin_box = loader.create_box(&box_type, ctor_args)?;
                let vm_value = VMValue::from_nyash_box(plugin_box);
                
                if let Some(dst_id) = dst {
                    self.set_value(*dst_id, vm_value);
                }
            }
        },
        
        // 既存のconsole.log等
        ("env.console", "log") => {
            // 既存の処理
        },
        
        _ => {
            println!("ExternCall stub: {}.{}", iface_name, method_name);
        }
    }
}
```

## 🔧 メモリ管理

### 参照カウント管理

1. **BoxRefの作成時**
   - Arc::fromでBox<dyn NyashBox>をArc<dyn NyashBox>に変換
   - 参照カウント = 1

2. **BoxRefのクローン時**
   - Arc::cloneで参照カウント増加
   - 軽量なポインタコピー

3. **BoxRefの破棄時**
   - 参照カウント減少
   - 0になったら自動解放

### スコープとライフタイム

```rust
// VMのスコープ管理
impl VM {
    fn exit_scope(&mut self) {
        // BoxRefを含むレジスタがクリアされると
        // 参照カウントが自動的に減少
        self.registers.clear();
    }
}
```

## 📈 パフォーマンス最適化

### 基本型の直接処理

```rust
// 最適化されたStringメソッド呼び出し
fn call_string_method_optimized(&self, s: &str, method: &str, args: &[ValueId]) 
    -> Result<VMValue, VMError> {
    match method {
        "length" => Ok(VMValue::Integer(s.len() as i64)),
        "substring" => {
            // 引数を直接整数として取得（Box化を回避）
            let start = self.get_value(args[0])?.to_i64()?;
            let end = self.get_value(args[1])?.to_i64()?;
            Ok(VMValue::String(s[start..end].to_string()))
        },
        _ => {
            // 未知のメソッドは汎用パスへ
            let string_box = Box::new(StringBox::new(s));
            self.call_box_method_generic(&*string_box, method, args)
        }
    }
}
```

### プラグイン呼び出しの最適化

1. **メソッドIDキャッシュ**
   - 頻繁に呼ばれるメソッドのIDをキャッシュ
   - 文字列比較を回避

2. **TLV変換の遅延評価**
   - 必要になるまでTLV変換を遅延
   - 基本型は直接渡す

## 🧪 テスト戦略

### 単体テスト

```rust
#[test]
fn test_vm_plugin_box_creation() {
    let plugin_loader = create_test_plugin_loader();
    let mut vm = VM::new_with_plugins(plugin_loader);
    
    // FileBoxの作成
    let result = vm.execute_extern_call(
        "plugin", "new", 
        vec!["FileBox", "test.txt"]
    );
    
    assert!(matches!(result, Ok(VMValue::BoxRef(_))));
}
```

### 統合テスト

```nyash
// VMで実行されるNyashコード
local file = new FileBox("output.txt")
file.write("VM Plugin Test")
local content = file.read()
assert(content == "VM Plugin Test")
```

### パフォーマンステスト

```rust
#[bench]
fn bench_plugin_method_call(b: &mut Bencher) {
    let vm = setup_vm_with_plugins();
    let file_box = create_file_box(&vm);
    
    b.iter(|| {
        vm.call_box_method(&file_box, "write", &["test"])
    });
}
```

## 🚨 エラーハンドリング

### プラグイン関連エラー

```rust
pub enum VMError {
    // 既存のエラー
    TypeError(String),
    RuntimeError(String),
    
    // プラグイン関連（新規）
    PluginNotFound(String),
    PluginMethodError { 
        plugin: String, 
        method: String, 
        error: String 
    },
    PluginInitError(String),
}
```

### エラー伝播

```rust
// プラグインエラーをVMエラーに変換
impl From<PluginError> for VMError {
    fn from(err: PluginError) -> Self {
        match err {
            PluginError::MethodNotFound(m) => {
                VMError::PluginMethodError { 
                    plugin: "unknown".to_string(),
                    method: m,
                    error: "Method not found".to_string()
                }
            },
            // ... 他のエラー変換
        }
    }
}
```

## 📊 メトリクスとモニタリング

### パフォーマンスメトリクス

- プラグイン呼び出し回数
- 平均呼び出し時間
- TLV変換オーバーヘッド
- メモリ使用量

### デバッグ情報

```rust
// デバッグモードでの詳細ログ
if cfg!(debug_assertions) {
    eprintln!("VM: Calling plugin method {}.{}", box_type, method);
    eprintln!("VM: Args: {:?}", args);
    eprintln!("VM: Result: {:?}", result);
}
```

## 🔄 ライフサイクル管理

### fini owner（現行実装）

```rust
impl Drop for PluginHandleInner {
    fn drop(&mut self) {
        // If fini_method_id exists and this handle has not already been
        // finalized, invoke plugin lifecycle fini through the resolved route.
    }
}

impl PluginHandleInner {
    pub fn finalize_now(&self) {
        // Used by singleton shutdown / explicit loader shutdown paths.
    }
}

pub fn shutdown_plugins_v2() {
    // Drains singleton handles and calls finalize_now().
}
```

### ライフサイクルの完全性

```nyash
// Plugin/native resources should be closed by their owner boundary.

local file = new FileBox("test.txt")
file.write("data")
file.fini()  // explicit resource boundary
```

## 🎯 統一の利点

### 1. **シンプルな実装**
- すべてのBox型が同じコードパスを通る
- 特殊ケースの削減
- バグの温床排除

### 2. **拡張性**
- 新しいBox型追加が容易
- プラグインも同じ扱い
- 将来の機能追加も簡単

### 3. **パフォーマンス**
- 基本型は最適化パス維持
- 必要時のみBoxRef使用
- メソッドディスパッチの効率化

---

**最終更新**: 2025-08-21  
**関連文書**: 
- [BID-FFI v1 実装仕様書](./bid-ffi-v1-actual-specification.md)
- [Phase 9.78a VM Plugin Integration](../../予定/native-plan/issues/phase_9_78a_vm_plugin_integration.md)
- [Phase 9.78a 深層分析](../../予定/native-plan/issues/phase_9_78a_vm_plugin_integration_deep_analysis.md)
 - [nyash.toml v2.1: BoxRef仕様](../plugin-system/nyash-toml-v2_1-spec.md)

### 付録: 引数エンコード（v2.1 追加）
- TLVタグ: 1=Bool, 2=I32, 3=I64, 4=F32, 5=F64, 6=String, 7=Bytes, 8=Handle(BoxRef)
- BoxRef payload(tag=8): `type_id:u32` + `instance_id:u32`（LE, 8バイト）
- `nyash.toml` の `args` で `{ kind="box", category="plugin" }` を指定したとき、Loaderは `tag=8` を使用

### 返り値（v2.2）
- プラグインが `tag=8` を返した場合、Loaderは `type_id` からBox型名を逆引きし `PluginBoxV2` を構築
- 同一ライブラリでなくてもOK（構成ファイル全体から探索）
