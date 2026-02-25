# 📡 Nyash P2P通信システム - 完全ガイド

**目標**: NyaMeshP2Pライブラリ実現のための本格的P2P通信システム

## 🏗️ システム全体像

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   P2PBox    │◄──►│ MessageBus   │◄──►│ Transport   │
│ (ユーザーAPI) │    │ (ローカル配送)  │    │ (送受信層)   │
└─────────────┘    └──────────────┘    └─────────────┘
       ▲                    ▲                   ▲
       │                    │                   │
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│ IntentBox   │    │ ハンドラ管理   │    │ InProcess   │
│ (構造化MSG)  │    │ ノード登録    │    │ WebSocket   │
└─────────────┘    └──────────────┘    │ WebRTC      │
                                       └─────────────┘
```

**核心思想**: 「P2PBox一つにTransport一つ + 共有MessageBus」

## 🧩 4つの主要Box

### 1. **IntentBox** - 構造化メッセージ 📨
```rust
pub struct IntentBoxData {
    pub name: String,           // "chat.message", "file.share"等
    pub payload: serde_json::Value,  // 任意のJSON data
}
pub type IntentBox = Arc<Mutex<IntentBoxData>>;
```

```nyash
// 使用例
msg = new IntentBox("chat.message", { text: "Hello P2P!", timestamp: 12345 })
print(msg.name)     // "chat.message"  
print(msg.payload)  // {"text":"Hello P2P!","timestamp":12345}
```

### 2. **MessageBus** - プロセス内シングルトン 🚌
```rust
pub struct MessageBusData {
    nodes: HashMap<String, BusEndpoint>,           // ノード登録
    subscribers: HashMap<String, Vec<IntentHandler>>, // "node_id:intent_name" → ハンドラー
    stats: BusStatistics,
}
pub type MessageBus = Arc<Mutex<MessageBusData>>;

// シングルトンアクセス
MessageBusData::global() -> MessageBus
```

**役割**: 同プロセス内での超高速メッセージルーティング・ハンドラ管理

### 3. **Transport** - 送受信抽象化 🔌
```rust
pub trait Transport: Send + Sync {
    fn node_id(&self) -> &str;
    fn send(&self, to: &str, intent: IntentBox, opts: SendOpts) -> Result<(), SendError>;
    fn on_receive(&mut self, callback: Box<dyn Fn(IntentEnvelope) + Send + Sync>);
}

// 3種類の実装
pub struct InProcessTransport { ... }    // 同プロセス内
pub struct WebSocketTransport { ... }    // WebSocket通信  
pub struct WebRTCTransport { ... }       // WebRTC P2P
```

### 4. **P2PBox** - 統合ユーザーAPI 🎉
```rust
pub struct P2PBoxData {
    node_id: String,
    transport: Arc<dyn Transport>,
    bus: MessageBus,  // 全P2PBoxで共有
}
pub type P2PBox = Arc<Mutex<P2PBoxData>>;

impl P2PBoxData {
    pub fn new(node_id: String, kind: TransportKind) -> P2PBox
    pub fn on(&self, intent_name: &str, handler: IntentHandler) -> Result<(), P2PError>
    pub fn send(&self, to: &str, intent: IntentBox) -> Result<(), SendError>
}
```

## 🚀 実用的使用例

### Level 1: 基本的なP2P通信
```nyash
// 2つのノード作成
node_a = new P2PBox("alice", transport: "inprocess")
node_b = new P2PBox("bob", transport: "inprocess")

// 受信ハンドラ設定
node_b.on("chat.message", function(intent, from) {
    console = new ConsoleBox()
    console.log("From " + from + ": " + intent.payload.text)
})

// メッセージ送信
msg = new IntentBox("chat.message", { text: "Hello P2P!" })
node_a.send("bob", msg)  // → "From alice: Hello P2P!"
```

### Level 2: 糖衣構文（将来実装）
```nyash
// 文字列直送（内部でIntentBox化） - 個別送信のみ
node_a.send("bob", "hello")           // → IntentBox("message", "hello")
node_a.send("bob", "chat:hello")      // → IntentBox("chat", "hello")
// 注意: ブロードキャスト機能は安全性のため除外（無限ループリスク回避）
```

### Level 3: 異なるTransport
```nyash
// 将来：異なるTransportでも同じAPI
local_node = new P2PBox("local", transport: "inprocess")
web_node   = new P2PBox("web", transport: "websocket", { url: "ws://localhost:8080" })
p2p_node   = new P2PBox("p2p", transport: "webrtc", { ice_servers: [...] })

// Transportに関係なく同じsend/onメソッド
msg = new IntentBox("file.share", { filename: "data.json" })
local_node.send("web", msg)    // WebSocket経由
web_node.send("p2p", msg)      // WebRTC経由
```

## 🔄 送受信フロー詳細

### 📤 送信フロー（node_a.send("node_b", intent)）
```
1. bus.has_node("node_b") == true？
   YES → bus.route("node_b", intent)    ← 同プロセス内の最速配送
   NO  → transport.send("node_b", intent, opts)  ← WebSocket/WebRTC経由
```

### 📥 受信フロー（ネットワークからの到着）
```  
transport.on_receive コールバックで受信
        ↓
1. envelope.to == self.node_id？
   YES → bus.dispatch_to_local(self.node_id, intent)  ← 自分宛
   NO  → 2へ
        ↓
2. bus.has_node(envelope.to)？
   YES → bus.route(envelope.to, intent)  ← 同プロセス内転送  
   NO  → bus.trace_drop(envelope)        ← ルート不明
```

## 📋 段階的実装計画

### **Phase 1: 基盤実装** （最優先）
1. **IntentBox** - メッセージBox（`src/boxes/intent_box.rs`）
2. **MessageBus** - プロセス内シングルトン（`src/messaging/message_bus.rs`）
3. **Transport trait** - 送受信抽象化（`src/transport/mod.rs`）

### **Phase 2: InProcess実装**
4. **InProcessTransport** - Bus経由の同プロセス通信（`src/transport/inprocess.rs`）
5. **P2PBox基本実装** - new, on, send メソッド（`src/boxes/p2p_box.rs`）
6. **インタープリター統合** - Nyash言語でのnew構文対応

### **Phase 3: ネットワーク拡張** （将来）
7. **WebSocketTransport** - WebSocket経由通信
8. **WebRTCTransport** - 直接P2P通信
9. **高度機能** - timeout, ACK, reconnect等

## 🧪 テスト戦略

### **基本動作確認テスト**
```nyash
// tests/phase2/p2p_basic_test.hako
node_a = new P2PBox("alice", transport: "inprocess")
node_b = new P2PBox("bob", transport: "inprocess")

node_b.on("test.ping", function(intent, from) {
    console = new ConsoleBox()
    console.log("PING from " + from)
})

msg = new IntentBox("test.ping", { timestamp: 12345 })
result = node_a.send("bob", msg)

// 期待出力: "PING from alice"
```

## 🎯 重要な設計原則

### **なぜ「非InProcessでもBusを持つ」のか？**
1. **ローカル最速配送**: Busが知ってればゼロコピー級の高速配送
2. **統一API**: ローカルもネットも外側APIは同じsend/onメソッド
3. **集約ログ**: 全P2P通信の統一ログ・メトリクス・デバッグ
4. **ハンドラ集約**: on()/subscribe登録・解除・トレースが一箇所

### **同期→非同期の段階的対応**
- **Phase 1**: `send()` は同期版 `fn` で実装開始
- **Phase 2**: 基本機能確立後に `async fn` 化
- **Phase 3**: Nyashインタープリターでのasync/await対応

### **Arc<Mutex>統一アーキテクチャ準拠**
- すべてのBoxは既存の `Arc<Mutex<_>>` パターンに準拠
- `MessageBus` も `Arc<Mutex<MessageBusData>>`
- `IntentBox` も `Arc<Mutex<IntentBoxData>>`

## 🚨 エラーハンドリング

```rust
#[derive(Debug, Clone)]
pub enum SendError {
    NodeNotFound(String),    // 宛先ノードが見つからない
    NetworkError(String),    // ネットワークエラー
    Timeout,                 // 送信タイムアウト
    SerializationError(String), // JSON変換エラー
    BusError(String),        // MessageBusエラー
}
```

## 🌟 最終目標

```nyash
// NyaMeshP2Pライブラリの実現
mesh = new NyaMesh("my_node")
mesh.join("chat_room")  
mesh.send("Hello everyone!")
mesh.on("message", function(msg, from) {
    print(from + ": " + msg)
})
```

---

**📚 関連ドキュメント**  
- **[DetailedP2PBoxSpec.md](DetailedP2PBoxSpec.md)** - ChatGPT大会議完全仕様（詳細版）
- **[MessageBusDesign.md](MessageBusDesign.md)** - MessageBus詳細設計
- **[CURRENT_TASK.md](../CURRENT_TASK.md)** - 現在の実装状況・優先順位

📝 最終更新: 2025-08-12 | 🎓 設計協力: Gemini先生・ChatGPT先生 | 🎯 目標: **Everything is Box**哲学によるP2P通信革命