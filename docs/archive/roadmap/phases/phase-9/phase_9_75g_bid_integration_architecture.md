# Phase 9.75g: BID統合プラグインアーキテクチャ実装計画

## 🎯 概要

**目的**: ビルトインBox動的ライブラリ化とBID（Box Interface Definition）統合により、全バックエンド（インタープリター/VM/WASM/AOT）で統一的に使えるプラグインシステムを構築する。

**期間**: 2週間（段階的実装）

**優先度**: 🔥 最高（VM性能改善の基盤にもなる）

## 🌟 設計哲学（AI大会議の結論を反映）

### 二層化アーキテクチャ
```
┌─────────────────────────────────────────┐
│        Nyashコード（不変）              │
├─────────────────────────────────────────┤
│     BID層（インターフェース定義）       │
│    - 型定義、メソッドシグネチャ        │
│    - エフェクト、エラー仕様           │
├─────────────────────────────────────────┤
│    Connector層（実装・トランスポート）  │
│    - DynamicLibrary (.so/.dll)         │
│    - REST/gRPC（将来）                │
│    - Language Bridge（将来）           │
└─────────────────────────────────────────┘
```

### 設計原則
1. **段階的実装**: 完璧を求めず、動くものから始める
2. **最小型集合**: i64, f64, string, bool, handle から開始
3. **コード生成**: 手書きコードを最小化、型安全性確保
4. **粗粒度API**: tight loopを避ける設計指針

## 📋 実装フェーズ

### ✅ Phase 9.75g-0: プロトタイプ実装（Day 1-5 完了！）

#### 実装完了項目（2025-08-18）
1. **仕様策定完了**
   - birth/finiライフサイクル管理追加
   - メモリ所有権ルール明確化
   - プラグインが自らBox名を宣言する設計

2. **基盤実装（Step 1-3）**
   - ✅ FileBoxプラグイン（293KB .so、6メソッド実装）
   - ✅ nyash.toml設定ファイル
   - ✅ plugin-tester診断ツール（汎用設計）

3. **重要な設計原則達成**
   - Box名非決め打ち（プラグインが宣言）
   - 汎用的設計（任意のプラグインに対応）
   - birth/finiライフサイクル実装

#### 実装詳細

##### FileBoxプラグイン（plugins/nyash-filebox-plugin/）
```rust
// 4つのFFI関数エクスポート
#[no_mangle] pub extern "C" fn nyash_plugin_abi() -> i32 { 1 }
#[no_mangle] pub extern "C" fn nyash_plugin_init(host: *const NyashHostVtable, info: *mut NyashPluginInfo) -> i32
#[no_mangle] pub extern "C" fn nyash_plugin_invoke(method_id: u32, args: *const u8, result: *mut u8) -> i32
#[no_mangle] pub extern "C" fn nyash_plugin_shutdown()

// 自己宣言型設計
static TYPE_NAME: &[u8] = b"FileBox\0";
(*info).type_id = 6;  // FileBoxのID
(*info).type_name = TYPE_NAME.as_ptr() as *const c_char;
```

##### plugin-tester診断ツール（tools/plugin-tester/）
```rust
// 汎用的設計 - Box名を決め打ちしない
let box_name = if plugin_info.type_name.is_null() {
    "<unknown>".to_string()
} else {
    CStr::from_ptr(plugin_info.type_name).to_string_lossy().to_string()
};

// 診断出力
println!("Plugin Information:");
println!("  Box Type: {} (ID: {})", box_name, plugin_info.type_id);
println!("  Methods: {}", plugin_info.method_count);
```

##### 実行結果
```
$ cargo run -- ../../plugins/nyash-filebox-plugin/target/debug/libnyash_filebox_plugin.so
Plugin loaded successfully!
Plugin Information:
  Box Type: FileBox (ID: 6)
  Methods: 6
  - birth [ID: 0, Sig: 0xBEEFCAFE] (constructor)
  - open [ID: 1, Sig: 0x12345678]
  - read [ID: 2, Sig: 0x87654321]
  - write [ID: 3, Sig: 0x11223344]
  - close [ID: 4, Sig: 0xABCDEF00]
  - fini [ID: 4294967295, Sig: 0xDEADBEEF] (destructor)
```

### 🎯 Phase 9.75g-1: Nyash統合実装（Step 4 - 段階的アプローチ）

実際のplugin-tester成功実装を基に、以下の順序でNyashに統合：

#### Step 4.1: TLVエンコード/デコード実装（src/bid/tlv.rs）
```rust
// プラグインとの通信プロトコル基盤
// plugin-testerで検証済みの仕様を実装

pub struct BidTLV {
    pub version: u8,
    pub flags: u8,
    pub argc: u16,
    pub entries: Vec<TLVEntry>,
}

pub struct TLVEntry {
    pub type_id: u8,
    pub reserved: u8,
    pub length: u16,
    pub data: Vec<u8>,
}

// エンコード/デコード実装
impl BidTLV {
    pub fn encode_string(s: &str) -> TLVEntry {
        TLVEntry {
            type_id: 0x03,  // STRING
            reserved: 0,
            length: s.len() as u16,
            data: s.as_bytes().to_vec(),
        }
    }
    
    pub fn decode_string(entry: &TLVEntry) -> Result<String, BidError> {
        String::from_utf8(entry.data.clone())
            .map_err(|_| BidError::InvalidEncoding)
    }
}
```

#### Step 4.2: プラグインローダー実装（src/bid/loader.rs）
```rust
// plugin-testerの成功部分を移植
// nyash.tomlパーサー（簡易版）

pub struct PluginLoader {
    plugins: HashMap<String, Arc<Plugin>>,
}

struct Plugin {
    library: Library,
    info: NyashPluginInfo,
    invoke_fn: unsafe extern "C" fn(u32, *const u8, *mut u8) -> i32,
}

impl PluginLoader {
    pub fn load_from_config(config_path: &str) -> Result<Self, BidError> {
        // nyash.tomlを読み込み
        let config = parse_nyash_toml(config_path)?;
        
        // 各プラグインをロード
        for (box_name, plugin_name) in config.plugins {
            self.load_plugin(&box_name, &plugin_name)?;
        }
        
        Ok(self)
    }
}
```

#### Step 4.3: BoxFactoryRegistry実装（src/bid/registry.rs）
```rust
// ビルトイン vs プラグインの透過的切り替え
// new FileBox()時の動的ディスパッチ

pub struct BoxFactoryRegistry {
    builtin_factories: HashMap<String, BoxFactory>,
    plugin_factories: HashMap<String, PluginBoxFactory>,
}

impl BoxFactoryRegistry {
    pub fn create_box(&self, box_name: &str, args: Vec<BidValue>) 
        -> Result<Box<dyn NyashBox>, BidError> 
    {
        // プラグイン優先で検索
        if let Some(plugin_factory) = self.plugin_factories.get(box_name) {
            return plugin_factory.create(args);
        }
        
        // ビルトインにフォールバック
        if let Some(builtin_factory) = self.builtin_factories.get(box_name) {
            return builtin_factory.create(args);
        }
        
        Err(BidError::BoxTypeNotFound(box_name.to_string()))
    }
}
```

#### Step 4.4: PluginBoxプロキシ実装（src/bid/plugin_box.rs）
```rust
// NyashBoxトレイト実装
// Dropトレイトでfini()呼び出し保証

pub struct PluginBox {
    plugin: Arc<Plugin>,
    handle: BidHandle,
}

impl NyashBox for PluginBox {
    fn type_name(&self) -> &'static str {
        // プラグインから取得した名前を返す
        &self.plugin.info.type_name
    }
    
    fn invoke_method(&self, method: &str, args: Vec<BidValue>) 
        -> Result<BidValue, BidError> 
    {
        // TLVエンコード → FFI呼び出し → TLVデコード
        let tlv_args = encode_to_tlv(args)?;
        let mut result_buf = vec![0u8; 4096];
        
        let status = unsafe {
            (self.plugin.invoke_fn)(
                method_id,
                tlv_args.as_ptr(),
                result_buf.as_mut_ptr()
            )
        };
        
        if status == 0 {
            decode_from_tlv(&result_buf)
        } else {
            Err(BidError::PluginError(status))
        }
    }
}

impl Drop for PluginBox {
    fn drop(&mut self) {
        // fini()メソッドを呼び出してリソース解放
        let _ = self.invoke_method("fini", vec![]);
    }
}
```


### Phase 9.75g-2: C ABI動的ライブラリConnector（3日）

#### 2.1 DynamicLibraryConnector実装（Day 3）
```rust
// src/bid/connectors/dynamic_library.rs
pub struct DynamicLibraryConnector {
    library_cache: Mutex<HashMap<String, Arc<Library>>>,
}

impl UniversalConnector for DynamicLibraryConnector {
    fn connect(&self, bid: &BidDefinition) -> Result<Box<dyn Connection>, BidError> {
        let path = &bid.transport.location;
        
        // ライブラリをロード
        let library = unsafe { 
            Library::new(path)
                .map_err(|e| BidError::Transport(format!("Failed to load {}: {}", path, e)))?
        };
        
        // バージョンチェック
        let version_fn: Symbol<unsafe extern "C" fn() -> u32> = unsafe {
            library.get(b"nyash_bid_version\0")?
        };
        
        let version = unsafe { version_fn() };
        if version != bid.version {
            return Err(BidError::Transport(format!(
                "Version mismatch: expected {}, got {}", 
                bid.version, version
            )));
        }
        
        Ok(Box::new(DynamicLibraryConnection {
            library: Arc::new(library),
            bid: bid.clone(),
        }))
    }
}
```

#### 2.2 高速vtableパス（Day 4）
```rust
// src/bid/vtable.rs
#[repr(C)]
pub struct InterfaceVTable {
    pub version: u32,
    pub interface_id: [u8; 16],  // UUID
    pub method_count: u32,
    pub methods: *const MethodEntry,
}

#[repr(C)]
pub struct MethodEntry {
    pub name: *const c_char,
    pub function: *const c_void,
    pub param_count: u32,
    pub param_types: *const BidTypeId,
    pub return_type: BidTypeId,
}

// 使用例（FileBox）
impl DynamicLibraryConnection {
    fn get_vtable(&self, interface: &str) -> Option<InterfaceVTable> {
        // シンボル名: nyash_{interface}_vtable
        let symbol_name = format!("nyash_{}_vtable\0", interface);
        
        let vtable_ptr: Symbol<*const InterfaceVTable> = unsafe {
            self.library.get(symbol_name.as_bytes()).ok()?
        };
        
        Some(unsafe { (*vtable_ptr).clone() })
    }
}
```

#### 2.3 FileBoxプラグイン移植（Day 5）
```rust
// plugins/nyash-file/src/lib.rs
use nyash_bid::*;

// C ABI関数
#[no_mangle]
pub extern "C" fn nyash_bid_version() -> u32 {
    1
}

#[no_mangle]
pub static NYASH_FILE_VTABLE: InterfaceVTable = InterfaceVTable {
    version: 1,
    interface_id: *b"nyash.file.v1.0\0",
    method_count: 4,
    methods: &FILE_METHODS as *const _,
};

static FILE_METHODS: [MethodEntry; 4] = [
    MethodEntry {
        name: b"open\0" as *const _ as *const c_char,
        function: nyash_file_open as *const _,
        param_count: 2,
        param_types: &[BidTypeId::String, BidTypeId::String] as *const _,
        return_type: BidTypeId::Handle,
    },
    // read, write, close...
];

// 実装
#[no_mangle]
pub extern "C" fn nyash_file_open(
    path: *const c_char,
    mode: *const c_char,
) -> *mut FileHandle {
    // 既存のFileBox実装を再利用
}
```

### Phase 9.75g-3: インタープリター統合（2日）

#### 3.1 BIDローダー統合（Day 6）
```rust
// src/interpreter/bid_loader.rs
pub struct BidPluginLoader {
    connectors: HashMap<TransportType, Box<dyn UniversalConnector>>,
    connections: HashMap<String, Box<dyn Connection>>,
}

impl BidPluginLoader {
    pub fn new() -> Self {
        let mut connectors = HashMap::new();
        
        // Phase 1: 動的ライブラリのみ
        connectors.insert(
            TransportType::DynamicLibrary,
            Box::new(DynamicLibraryConnector::new()),
        );
        
        Self {
            connectors,
            connections: HashMap::new(),
        }
    }
    
    pub fn load_bid(&mut self, yaml_path: &str) -> Result<(), BidError> {
        let content = fs::read_to_string(yaml_path)?;
        let bid = parse_bid(&content)?;
        
        // 適切なコネクターを選択
        let connector = self.connectors
            .get(&bid.transport.transport_type)
            .ok_or_else(|| BidError::Transport(
                format!("Unsupported transport: {:?}", bid.transport.transport_type)
            ))?;
        
        // 接続を確立
        let connection = connector.connect(&bid)?;
        
        // インターフェースごとに登録
        for interface in &bid.interfaces {
            let key = format!("{}.{}", interface.namespace, interface.name);
            self.connections.insert(key, connection.clone());
        }
        
        Ok(())
    }
}
```

#### 3.2 既存コードとの互換性層（Day 7）
```rust
// src/interpreter/objects.rs の修正
impl NyashInterpreter {
    fn execute_new(&mut self, class: &str, args: Vec<Box<dyn NyashBox>>) 
        -> Result<Box<dyn NyashBox>, RuntimeError> 
    {
        // 既存のビルトインBox処理
        if is_builtin_box(class) {
            // 従来の処理...
        }
        
        // BIDプラグインチェック
        if let Some(connection) = self.bid_loader.get_connection(class) {
            // BID経由で作成
            let bid_args: Vec<BidValue> = args.iter()
                .map(|arg| nyash_to_bid_value(arg))
                .collect::<Result<_, _>>()?;
            
            let result = connection.invoke(class, "new", &bid_args)?;
            
            return Ok(bid_to_nyash_box(result)?);
        }
        
        // ユーザー定義Box
        // 従来の処理...
    }
}
```

### Phase 9.75g-4: MIR/VM統合（3日）

#### 4.1 ExternCall命令とBID統合（Day 8）
```rust
// src/mir/builder.rs の修正
impl MirBuilder {
    fn build_method_call(&mut self, object: ASTNode, method: String, args: Vec<ASTNode>) 
        -> Result<ValueId, String> 
    {
        // オブジェクトの型を解析
        let obj_type = self.infer_type(&object)?;
        
        // BIDプラグインメソッドかチェック
        if let Some(bid_interface) = self.bid_registry.get_interface(&obj_type) {
            // ExternCall命令を生成
            let dst = self.value_gen.next();
            self.emit_instruction(MirInstruction::ExternCall {
                dst: Some(dst),
                iface_name: bid_interface.name.clone(),
                method_name: method,
                args: arg_values,
                effects: bid_interface.get_method_effects(&method),
            })?;
            
            return Ok(dst);
        }
        
        // 通常のBoxCall
        // 従来の処理...
    }
}
```

#### 4.2 VM実行時BID呼び出し（Day 9）
```rust
// src/backend/vm.rs の修正
impl VM {
    fn execute_extern_call(&mut self, 
        dst: Option<ValueId>,
        iface: &str,
        method: &str,
        args: &[ValueId],
        effects: &EffectMask,
    ) -> Result<(), VMError> {
        // BID接続を取得
        let connection = self.bid_loader
            .get_connection(iface)
            .ok_or_else(|| VMError::InterfaceNotFound(iface.to_string()))?;
        
        // 引数をBidValueに変換
        let bid_args: Vec<BidValue> = args.iter()
            .map(|id| self.vm_to_bid_value(*id))
            .collect::<Result<_, _>>()?;
        
        // 高速パスチェック（vtable利用可能か）
        if let Some(vtable) = connection.get_vtable(iface) {
            // 直接関数ポインタ呼び出し（最速）
            let result = unsafe { 
                call_vtable_method(&vtable, method, &bid_args)? 
            };
            
            if let Some(dst_id) = dst {
                self.set_value(dst_id, bid_to_vm_value(result)?);
            }
        } else {
            // 汎用invoke経路（リモート対応）
            let result = connection.invoke(iface, method, &bid_args)?;
            
            if let Some(dst_id) = dst {
                self.set_value(dst_id, bid_to_vm_value(result)?);
            }
        }
        
        Ok(())
    }
}
```

### Phase 9.75g-5: コード生成ツール（2日）

#### 5.1 BIDからRustスタブ生成（Day 10）
```bash
# CLIツール
nyash-bid-gen --input file.bid.yaml --output src/generated/
```

生成されるコード例:
```rust
// src/generated/nyash_file.rs
pub struct FileBoxClient {
    connection: Arc<dyn Connection>,
}

impl FileBoxClient {
    pub fn open(&self, path: &str, mode: &str) -> Result<FileHandle, BidError> {
        let args = vec![
            BidValue::String(path.to_string()),
            BidValue::String(mode.to_string()),
        ];
        
        let result = self.connection.invoke("nyash.file", "open", &args)?;
        
        match result {
            BidValue::Handle(h) => Ok(FileHandle(h)),
            _ => Err(BidError::TypeMismatch {
                expected: "handle".to_string(),
                actual: format!("{:?}", result),
            }),
        }
    }
}
```

#### 5.2 プラグイン側スケルトン生成（Day 11）
```rust
// 生成されるプラグイン側のスケルトン
pub trait FileBoxImpl {
    fn open(&self, path: &str, mode: &str) -> Result<FileHandle, FileError>;
    fn read(&self, handle: &FileHandle, size: usize) -> Result<Vec<u8>, FileError>;
    fn write(&self, handle: &FileHandle, data: &[u8]) -> Result<usize, FileError>;
    fn close(&self, handle: FileHandle) -> Result<(), FileError>;
}

// C ABIラッパーも自動生成
#[no_mangle]
pub extern "C" fn nyash_file_open(
    path: *const c_char,
    mode: *const c_char,
) -> *mut c_void {
    // 実装への橋渡し
}
```

## 📊 テスト計画

### 統合テスト（Day 12）
```nyash
// test_bid_integration.hako
using nyashstd

// BIDプラグインのロード
bid.load("plugins/file.bid.yaml")

// 通常のNyashコードで使用（透過的）
local file = new FileBox("test.txt", "w")
file.write("Hello from BID!")
file.close()

// 性能測定
local timer = new TimerBox()
timer.start()

local i = 0
loop(i < 1000) {
    local f = new FileBox("bench.txt", "r")
    f.read(1024)
    f.close()
    i = i + 1
}

timer.stop()
console.log("1000 file operations: " + timer.elapsed() + "ms")
```

### ベンチマーク目標
- C ABI直接呼び出し: < 100ns オーバーヘッド
- 型変換コスト: < 50ns（基本型）
- メモリ効率: 既存実装と同等以下

## 🎯 成功基準

### Phase 9.75g完了時
- [ ] BIDパーサー・型システム・エラーモデル完成
- [ ] DynamicLibraryConnector完全動作
- [ ] FileBoxがBID経由で動作
- [ ] インタープリター/VM/WASMすべてで同じBIDが使える
- [ ] コード生成ツール基本機能
- [ ] 性能目標達成（< 100ns オーバーヘッド）

### 将来の拡張準備
- [ ] Transport抽象化の拡張ポイント確保
- [ ] ストリーミング/非同期の設計考慮
- [ ] セキュリティ拡張ポイント予約

## 🔧 実装の鍵

### 1. 段階的アプローチ
- 完璧を求めない
- 動くものから始める
- フィードバックを早く得る

### 2. 既存資産の活用
- FileBoxProxyの実装を再利用
- 既存のプラグインローダーと共存

### 3. 性能最優先
- C ABI高速パスを最初に実装
- 型変換を最小化
- ゼロコピーを目指す

### 4. 開発者体験
- コード生成で型安全性
- エラーメッセージの充実
- デバッグ支援機能

## 📅 マイルストーン

- **Week 1**: BID基盤 + C ABIコネクター + FileBox移植
- **Week 2**: インタープリター/VM統合 + コード生成 + テスト

## 🚀 期待される成果

1. **統一プラグインシステム**: 全バックエンドで同じプラグインが動く
2. **ビルド時間改善**: 動的ライブラリ化で2分→15秒
3. **将来の拡張性**: REST/gRPC/Python等への道筋
4. **VM性能改善の基盤**: BID経由のプロファイリング・最適化

---

**作成**: 2025-08-17  
**作成者**: Claude (AI大会議の結論を統合)  
**優先度**: 🔥 最高（VM性能改善の前提）