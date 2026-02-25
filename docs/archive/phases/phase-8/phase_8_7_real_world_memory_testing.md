# Phase 8.7: Real-world Memory Management Testing + VM BoxCall修正（統合版）

## 🎯 Issue概要

**主目的**: 実用アプリケーション開発によるNyashメモリ管理システムの実証テスト

**統合目的**: VM BoxCall戻り値問題の修正を実用アプリ実装と同時に実施

**戦略的背景**: 
- Phase 8.4完了でAST→MIR Lowering完成
- Phase 8.5完了でMIR 25命令階層化完成  
- **発見された課題**: VM BoxCall実行後の戻り値が`void`になる問題
- **合理的統合**: kilo実装とBoxCall修正を同時実施で効率最大化

**統合効果**: 
```
kilo実装 = ユーザー定義Box + メソッド呼び出し重用
         ↓
BoxCall正常動作 = kilo正常動作の前提条件
         ↓  
統合実装 = 一石二鳥の効率性
```

## 🎯 Phase 8.7A: kilo（テキストエディタ）

### 技術的特徴
- **サイズ**: <1k LOC（超小型）
- **メモリパターン**: Editor -> (Rows -> Syntax) 木構造＋相互参照
- **fini戦略**: Editor削除でRows自動解放、逆参照をweak化
- **BoxCall実証**: ユーザー定義Boxメソッド呼び出しでVM戻り値正常化確認
- **統合検証**: メモリ管理 + VM BoxCall動作の同時実証

### 実装仕様

#### 基本構造
```nyash
box Editor {
    init { rows, current_row, screen_rows, filename }
    
    pack() {
        me.rows = new ArrayBox()
        me.current_row = 0
        me.screen_rows = 24
        me.filename = ""
    }
    
    fini() {
        // ArrayBox自動解放でRows全解放
        // weak参照は自動null化される
    }
}

box Row {
    init { text, size, editor }  // editor: weak参照
    
    pack(text_content, parent_editor) {
        me.text = text_content
        me.size = text_content.length()
        me.editor = weak parent_editor  // 循環参照回避
    }
    
    render() {
        if me.editor == null {
            return "ERROR: Editor already freed"
        }
        return me.text
    }
}

box EditorState {
    init { cursor_x, cursor_y, editor }  // editor: weak参照
    
    pack(editor_ref) {
        me.cursor_x = 0
        me.cursor_y = 0
        me.editor = weak editor_ref
    }
}
```

#### メイン処理
```nyash
static box Main {
    main() {
        local editor = new Editor()
        
        // ファイル読み込み
        editor.loadFile("test.txt")
        
        // 編集操作
        editor.insertLine(0, "Hello Nyash Editor!")
        editor.insertLine(1, "This tests memory management")
        
        // 状態作成
        local state = new EditorState(editor)
        
        // editor削除 → Rows自動解放、state.editorは自動null化
        editor.fini()
        
        // weak参照確認
        if state.editor == null {
            print("✅ Editor properly freed, weak ref nullified")
            return 1
        } else {
            print("❌ Memory leak detected!")
            return 0
        }
    }
}
```

### 🧪 検証テストケース

#### Test 1: 基本メモリ管理
```nyash
// test_kilo_basic_memory.hako
box Editor {
    init { rows }
    pack() { me.rows = new ArrayBox() }
    fini() { print("Editor freed") }
}

box Row {
    init { editor }
    pack(ed) { me.editor = weak ed }
}

static box Main {
    main() {
        local editor = new Editor()
        local row = new Row(editor)
        
        // editor削除
        editor.fini()
        
        // weak参照確認
        return row.editor == null ? 1 : 0
    }
}
```

#### Test 2: 複雑な相互参照
```nyash
// test_kilo_circular_refs.hako
box Editor {
    init { rows, state }
    pack() {
        me.rows = new ArrayBox()
        me.state = new EditorState(me)  // 循環参照テスト
    }
}

box EditorState {
    init { editor }
    pack(ed) { me.editor = weak ed }
}

static box Main {
    main() {
        local editor = new Editor()
        editor.pack()
        
        // 循環参照があっても正常解放されるか
        editor.fini()
        
        return 1  // メモリリークなしで完了すればOK
    }
}
```

#### Test 3: 大量オブジェクト管理
```nyash
// test_kilo_mass_objects.hako
static box Main {
    main() {
        local editor = new Editor()
        
        // 大量行作成
        loop(i < 1000) {
            editor.addRow("Line " + i)
        }
        
        print("Created 1000 rows")
        
        // 一括削除
        editor.fini()
        
        print("Editor freed with all rows")
        return 1
    }
}
```

### ✅ 成功基準（統合版）

#### 必須基準（メモリ管理）
- [ ] 全テストケースでメモリリークなし
- [ ] weak参照の自動null化動作確認
- [ ] fini()伝播の正確性確認
- [ ] 循環参照でも正常解放確認

#### 必須基準（VM BoxCall修正）
- [ ] VM BoxCall実行後の戻り値が正常に返される
- [ ] ユーザー定義Boxメソッド呼び出しがVMで正常動作
- [ ] Interpreter/VM/WASMで同一BoxCall動作
- [ ] kilo実装でBoxCallが期待通り動作

#### 理想基準
- [ ] 1000+オブジェクトでも高速動作
- [ ] WASM実行でもメモリ管理正常
- [ ] ベンチマーク性能劣化なし
- [ ] VM BoxCall性能がInterpreterと同等以上

## 🚀 Phase 9.5: tiny-web-server（将来実装）

### 技術的特徴
- **複雑度**: 中〜高
- **メモリパターン**: Server -> Clients -> Requests（並行処理）
- **I/O管理**: ソケット・ファイルハンドルの確実解放

### 基本設計
```nyash
box Server {
    init { clients, port }
    fini() {
        // 全クライアント接続を確実切断
        me.clients.forEach(client => client.fini())
    }
}

box Client {
    init { socket, server }  // server: weak参照
    fini() {
        me.socket.close()  // 確実なソケット解放
    }
}
```

## 🤖 Copilot向け実装ガイド

### 実装順序（統合版）
1. **Phase 1**: VM BoxCall戻り値修正 + Editor/Row基本構造実装
2. **Phase 2**: weak参照・fini()システム統合 + BoxCall動作確認
3. **Phase 3**: テストケース実装・検証（メモリ管理 + BoxCall統合テスト）
4. **Phase 4**: パフォーマンス最適化・3バックエンド互換性確認

### 重要注意点
- **weak参照構文**: `me.editor = weak editor_ref`
- **fini()自動呼び出し**: ガベージコレクション時
- **メモリリーク検出**: デバッグ出力で確認
- **WASM互換性**: ブラウザ環境でも動作

### デバッグ支援（統合版）
```bash
# メモリ使用量監視
./target/release/nyash --debug-memory test_kilo_basic.hako

# weak参照追跡
./target/release/nyash --trace-weak test_kilo_circular.hako

# fini呼び出し追跡
./target/release/nyash --trace-fini test_kilo_mass.hako

# BoxCall戻り値デバッグ（新規）
./target/release/nyash --debug-boxcall test_kilo_basic.hako

# VM/Interpreter/WASM BoxCall比較（新規）
./target/release/nyash --compare-boxcall test_kilo_basic.hako

# 統合デバッグ（メモリ + BoxCall）
./target/release/nyash --debug-all test_kilo_basic.hako
```

## 📊 期待される効果（統合版）

### 技術的効果
- **メモリ管理実証**: Nyashメモリ管理システムの実用性実証
- **VM実行基盤確立**: BoxCall正常動作によるVM実用性確保
- **Everything is Box実証**: Box哲学の実用レベル確認
- **fini/weak参照実証**: システムの堅牢性確認
- **3バックエンド統一**: Interpreter/VM/WASMでの一貫動作

### 開発体験向上
- **実用アプリ開発実現**: kiloエディタによる実証
- **メモリ安全パターン**: プログラミングパターン確立
- **デバッグ環境整備**: 包括的デバッグ支援機能
- **移行容易性**: 他言語からの移行促進
- **Phase 9準備完了**: JIT実装への安全な基盤確立

---

**優先度**: 🚨 Critical（Phase 8.5完了直後の最優先）
**期間**: 2週間（Phase 8.6統合により3日短縮）
**担当**: Copilot + Claude協調実装
**統合目標**: 
- ✅ メモリ安全な実用アプリケーション完成（kilo）
- ✅ VM BoxCall戻り値問題完全解決  
- ✅ Phase 9 JIT実装への安全な基盤確立

**戦略的価値**: 効率性最大化（統合実装）+ 品質保証（実証テスト）+ Phase 9準備完了