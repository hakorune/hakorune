use crate::ast::{ASTNode, ContractClause, ContractKind};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(crate) fn parse_signature_metadata_until_body(
        &mut self,
    ) -> Result<(Vec<String>, Vec<ContractClause>), ParseError> {
        let mut uses = Vec::new();
        let mut contracts = Vec::new();
        loop {
            while self.match_token(&TokenType::NEWLINE) {
                self.advance();
            }
            if self.current_context_word("uses") {
                self.advance();
                uses.extend(self.parse_uses_clause_items()?);
                if self.match_token(&TokenType::SEMICOLON) {
                    self.advance();
                }
                continue;
            }
            if let Some(kind) = self.current_contract_kind() {
                self.advance();
                let condition = self.parse_expression()?;
                contracts.push(ContractClause { kind, condition });
                if self.match_token(&TokenType::SEMICOLON) {
                    self.advance();
                }
                continue;
            }
            break;
        }
        Ok((uses, contracts))
    }

    pub(crate) fn try_parse_invariant_clause(&mut self) -> Result<Option<ASTNode>, ParseError> {
        while self.match_token(&TokenType::NEWLINE) {
            self.advance();
        }
        if !self.current_context_word("invariant") {
            return Ok(None);
        }
        self.advance();
        let condition = self.parse_expression()?;
        if self.match_token(&TokenType::SEMICOLON) {
            self.advance();
        }
        Ok(Some(condition))
    }

    fn parse_uses_clause_items(&mut self) -> Result<Vec<String>, ParseError> {
        let mut out = Vec::new();
        loop {
            match &self.current_token().token_type {
                TokenType::IDENTIFIER(name) => {
                    out.push(name.clone());
                    self.advance();
                }
                other => {
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "capability name after uses".to_string(),
                        line: self.current_token().line,
                    });
                }
            }
            if self.match_token(&TokenType::COMMA) {
                self.advance();
                continue;
            }
            break;
        }
        Ok(out)
    }

    fn current_contract_kind(&self) -> Option<ContractKind> {
        match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) if name == "requires" => Some(ContractKind::Requires),
            TokenType::IDENTIFIER(name) if name == "ensures" => Some(ContractKind::Ensures),
            _ => None,
        }
    }

    fn current_context_word(&self, word: &str) -> bool {
        matches!(&self.current_token().token_type, TokenType::IDENTIFIER(name) if name == word)
    }
}
