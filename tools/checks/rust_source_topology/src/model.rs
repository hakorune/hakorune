use serde::Serialize;

pub const RUST_SOURCE_TOPOLOGY_SCHEMA_V1: &str = "rust-source-topology-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RustSourceTopologyV1 {
    pub schema: &'static str,
    pub schema_version: u32,
    pub source_file: SourceFileTopologyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceFileTopologyV1 {
    pub path: String,
    pub content_digest: String,
    pub edition: SourceEditionObservationV1,
    pub parse_status: ParseStatusV1,
    pub root_module_syntax_path: String,
    pub items: Vec<ItemFactV1>,
    pub direct_call_sites: Vec<DirectCallSiteV1>,
    pub unresolved_call_sites: Vec<UnresolvedCallSiteV1>,
    pub opaque_syntax_sites: Vec<OpaqueSyntaxSiteV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceEditionObservationV1 {
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseStatusV1 {
    Success,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ItemFactV1 {
    pub item_id: String,
    pub syntax_path: String,
    pub parent_item_id: Option<String>,
    pub module_syntax_path: String,
    pub kind: ItemKindV1,
    pub impl_self_type_syntax: Option<String>,
    pub impl_trait_syntax: Option<String>,
    pub source_range: SourceRangeV1,
    pub direct_cfg_syntax: Vec<String>,
    pub inherited_cfg_syntax: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemKindV1 {
    InlineModule,
    ExternalModule,
    Function,
    Impl,
    ImplMethod,
    ImplAssociatedConst,
    Trait,
    TraitMethod,
    TraitAssociatedConst,
    Const,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirectCallSiteV1 {
    pub call_site_id: String,
    pub path: String,
    pub module_syntax_path: String,
    pub enclosing_item_id: Option<String>,
    pub enclosing_item_syntax_path: String,
    pub lexical_context: Vec<LexicalContextV1>,
    pub expression_kind: DirectCallExpressionKindV1,
    pub source_range: SourceRangeV1,
    pub source_text: String,
    pub source_digest: String,
    pub normalized_callee_syntax: String,
    pub receiver_syntax: Option<String>,
    pub explicit_receiver_type_syntax: Option<String>,
    pub direct_cfg_syntax: Vec<String>,
    pub inherited_cfg_syntax: Vec<String>,
    pub resolution: DirectCallResolutionV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectCallExpressionKindV1 {
    ExprCall,
    ExprMethodCall,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DirectCallResolutionV1 {
    Resolved {
        def_path: String,
    },
    Unresolved {
        reason: DirectCallUnresolvedReasonV1,
        evidence: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectCallUnresolvedReasonV1 {
    ResolutionDeferredToS0c,
    IndirectFunctionValue,
    ClosureInvocation,
    TraitOrMethodDispatch,
    GeneralReceiverInference,
    UnsupportedCalleeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnresolvedCallSiteV1 {
    pub call_site_id: String,
    pub reason: DirectCallUnresolvedReasonV1,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LexicalContextV1 {
    pub kind: LexicalContextKindV1,
    pub source_range: SourceRangeV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LexicalContextKindV1 {
    Closure,
    AsyncBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OpaqueSyntaxSiteV1 {
    pub opaque_site_id: String,
    pub path: String,
    pub module_syntax_path: String,
    pub enclosing_item_id: Option<String>,
    pub kind: OpaqueSyntaxKindV1,
    pub syntax_name: String,
    pub source_range: SourceRangeV1,
    pub source_text: String,
    pub source_digest: String,
    pub direct_cfg_syntax: Vec<String>,
    pub inherited_cfg_syntax: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OpaqueSyntaxKindV1 {
    MacroInvocation,
    IncludeMacro,
    ExternalModule,
    PathAttributedExternalModule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SourceRangeV1 {
    pub start: PositionV1,
    pub end: PositionV1,
    pub byte_start: usize,
    pub byte_end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PositionV1 {
    pub line: usize,
    pub column: usize,
}
