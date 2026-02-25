/*!
 * TypeScript Target Generator - Generate TypeScript FFI wrappers
 *
 * ⚠️ **DEPRECATION STATUS**: This is a placeholder stub.
 *
 * **Status**: Not implemented (NOT on Phase 15 critical path)
 *
 * **Replacement Path**:
 * - For production TypeScript FFI: Consider using LLVM harness + language bindings
 * - For development: Use VM interpreter with TypeScript wrappers (manual implementation)
 *
 * **Future**: If TypeScript support needed, implement via WASM target or language-specific bindings.
 *
 * **Decision**: This stub remains in place to preserve CodeGenTarget API surface.
 * Remove if TypeScript code generation is officially deprecated (Phase 150+).
 */

use crate::bid::{BidDefinition, BidResult};
use crate::bid::codegen::{CodeGenOptions, GeneratedFile};

pub struct TypeScriptGenerator;

impl TypeScriptGenerator {
    /// Generate TypeScript wrappers
    ///
    /// ⚠️ Not implemented - returns empty vec
    pub fn generate(bid: &BidDefinition, _options: &CodeGenOptions) -> BidResult<Vec<GeneratedFile>> {
        // TODO: Implement TypeScript code generation
        eprintln!("⚠️  TypeScript code generation not yet implemented for {}", bid.name());
        eprintln!("   See: docs/development/current/main/REFACTORING_OPPORTUNITIES.md#bid-codegen-stubs-decision");
        Ok(vec![])
    }
}