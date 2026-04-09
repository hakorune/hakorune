use super::{NyashTokenizer, TokenType};
use crate::grammar::engine;

impl NyashTokenizer {
    /// キーワードまたは識別子を読み取り
    pub(crate) fn read_keyword_or_identifier(&mut self) -> TokenType {
        let mut identifier = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // キーワードチェック
        let mut tok = match identifier.as_str() {
            "box" => TokenType::BOX,
            "enum" => TokenType::ENUM,
            "global" => TokenType::GLOBAL,
            "singleton" => TokenType::SINGLETON,
            "new" => TokenType::NEW,
            "match" => TokenType::MATCH,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "loop" => TokenType::LOOP,
            "break" => TokenType::BREAK,
            "continue" => TokenType::CONTINUE,
            "return" => TokenType::RETURN,
            "function" => TokenType::FUNCTION,
            "fn" => TokenType::FN,
            "print" => TokenType::PRINT,
            "this" => TokenType::THIS,
            "me" => TokenType::ME,
            "init" => TokenType::INIT,
            "pack" => TokenType::PACK,
            "birth" => TokenType::BIRTH,
            "nowait" => TokenType::NOWAIT,
            "await" => TokenType::AWAIT,
            "interface" => TokenType::INTERFACE,
            // "include" keyword removed (use `using` instead)
            "import" => TokenType::IMPORT,
            "try" => TokenType::TRY,
            "catch" => TokenType::CATCH,
            "cleanup" => TokenType::CLEANUP,
            "fini" => TokenType::FINI,
            "throw" => TokenType::THROW,
            "local" => TokenType::LOCAL,
            "flow" => TokenType::FLOW,
            "static" => TokenType::STATIC,
            "outbox" => TokenType::OUTBOX,
            "not" => TokenType::NOT,
            "override" => TokenType::OVERRIDE,
            "from" => TokenType::FROM,
            "weak" => TokenType::WEAK,
            "using" => TokenType::USING,
            "and" => TokenType::AND,
            "or" => TokenType::OR,
            // Stage-3 loop keywords (gated below)
            "while" => TokenType::WHILE,
            "for" => TokenType::FOR,
            "in" => TokenType::IN,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "null" => TokenType::NULL,
            "void" => TokenType::VOID,
            _ => TokenType::IDENTIFIER(identifier.clone()),
        };

        // Stage-3 gate: LOCAL/FLOW/TRY/CATCH/THROW require Stage-3 parser (default ON)
        let stage3_enabled = crate::config::env::parser_stage3_enabled();
        if !stage3_enabled {
            let is_stage3 = matches!(
                tok,
                TokenType::LOCAL
                    | TokenType::FLOW
                    | TokenType::TRY
                    | TokenType::CATCH
                    | TokenType::FINI
                    | TokenType::THROW
                    | TokenType::WHILE
                    | TokenType::FOR
                    | TokenType::IN
            );
            if is_stage3 {
                if crate::config::env::tok_trace() {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[tok-stage3] Degrading {:?} to IDENTIFIER (stage3_enabled={})",
                        tok, stage3_enabled
                    ));
                }
                tok = TokenType::IDENTIFIER(identifier.clone());
            }
        } else {
            if crate::config::env::tok_trace() {
                let is_stage3 = matches!(
                    tok,
                    TokenType::LOCAL
                        | TokenType::FLOW
                        | TokenType::TRY
                        | TokenType::CATCH
                        | TokenType::FINI
                        | TokenType::THROW
                        | TokenType::WHILE
                        | TokenType::FOR
                        | TokenType::IN
                );
                if is_stage3 {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[tok-stage3] Keeping {:?} as keyword (stage3_enabled={})",
                        tok, stage3_enabled
                    ));
                }
            }
        }

        // 12.7 Strict mode: fallback extended keywords to IDENTIFIER
        if Self::strict_12_7() {
            let is_extended = matches!(
                tok,
                TokenType::INTERFACE
                    | TokenType::USING
                    | TokenType::OUTBOX
                    | TokenType::NOWAIT
                    | TokenType::OVERRIDE
                    | TokenType::WEAK
                    | TokenType::PACK
            );
            if is_extended {
                tok = TokenType::IDENTIFIER(identifier.clone());
            }
        }

        // 統一文法エンジンとの差分チェック（動作は変更しない）
        if crate::config::env::grammar_diff() {
            if let Some(kw) = engine::get().is_keyword_str(&identifier) {
                if let TokenType::IDENTIFIER(_) = tok {
                    crate::runtime::get_global_ring0().log.warn(&format!(
                        "[GRAMMAR-DIFF] tokenizer=IDENT, grammar=KEYWORD({}) word='{}'",
                        kw, identifier
                    ));
                }
            } else if !matches!(tok, TokenType::IDENTIFIER(_)) {
                crate::runtime::get_global_ring0().log.warn(&format!(
                    "[GRAMMAR-DIFF] tokenizer=KEYWORD, grammar=IDENT word='{}'",
                    identifier
                ));
            }
        }

        tok
    }
}
