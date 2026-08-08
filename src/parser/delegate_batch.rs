//! Parser-private C-I0 generated-delegate batch.
//!
//! The batch is prepared against the complete unpublished postpass product,
//! then committed once. AST and inventory mutation only touches a local clone
//! until every host/expose and placement receipt has passed preflight.

use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryPlacementReceiptV1,
    BoxMethodInventoryV1, DelegateDecl, FieldDecl, PreparedGeneratedBoxMethodBatchV1,
    PreparedGeneratedBoxMethodV1,
};
use crate::parser::ParseError;

use super::delegate_source_relation::{
    ExistingTargetMethodSourceRefV1, GeneratedDelegateSourceRelationV1,
};
use super::delegate_target_index::{
    DelegateTargetIndexErrorV1, DelegateTargetIndexV1, DelegateTargetResolutionV1,
};
use super::source_authority::DelegateSourceDeclarationV1;
use super::source_path::SourceBoxDeclarationPathV1;
use super::source_seal::OpenParserPostpassProductV1;

#[derive(Debug)]
enum BatchFailureV1 {
    Rejected(String),
    Unresolved(String),
    Declined(String),
}

impl BatchFailureV1 {
    fn into_parse_error(self) -> ParseError {
        let (kind, reason) = match self {
            Self::Rejected(reason) => ("Rejected", reason),
            Self::Unresolved(reason) => ("Unresolved", reason),
            Self::Declined(reason) => ("Declined", reason),
        };
        ParseError::DelegateLowering {
            message: format!("R6-S3B-C-I0 {kind}: {reason}"),
            line: 0,
        }
    }
}

#[derive(Debug)]
struct HostView<'product> {
    path: &'product SourceBoxDeclarationPathV1,
    fields: &'product [FieldDecl],
    delegates: &'product [DelegateDecl],
    methods: &'product BoxMethodInventoryV1,
}

#[derive(Debug)]
struct PendingGeneratedRow {
    expose: DelegateSourceDeclarationV1,
    target: ExistingTargetMethodSourceRefV1,
    generated_name_provenance: BoxMethodGeneratedProvenanceV1,
    generated_method: PreparedGeneratedBoxMethodV1,
}

#[derive(Debug)]
struct StagedHostBatch {
    host_path: SourceBoxDeclarationPathV1,
    batch: PreparedGeneratedBoxMethodBatchV1,
    placements: Box<[BoxMethodInventoryPlacementReceiptV1]>,
    relations: Box<[GeneratedDelegateSourceRelationV1]>,
}

#[derive(Debug)]
struct PreparedDelegatePostpassBatchV1 {
    hosts: Box<[StagedHostBatch]>,
}

impl PreparedDelegatePostpassBatchV1 {
    fn new(hosts: Vec<StagedHostBatch>) -> Self {
        Self {
            hosts: hosts.into_boxed_slice(),
        }
    }

    fn is_empty(&self) -> bool {
        self.hosts.is_empty()
    }

    fn relation_batches(
        self,
    ) -> Vec<(
        SourceBoxDeclarationPathV1,
        Box<[GeneratedDelegateSourceRelationV1]>,
    )> {
        self.hosts
            .iter()
            .map(|batch| (batch.host_path.clone(), batch.relations.clone()))
            .collect()
    }
}

pub(super) fn lower_delegates(
    product: OpenParserPostpassProductV1,
) -> Result<OpenParserPostpassProductV1, ParseError> {
    let staged = prepare_all(&product).map_err(BatchFailureV1::into_parse_error)?;
    if staged.is_empty() {
        return Ok(product);
    }

    let final_box_paths = product.final_box_paths.clone();
    let ast = apply_staged_batches(product.ast.clone(), &final_box_paths, &staged.hosts)
        .map_err(BatchFailureV1::into_parse_error)?;
    let relation_batches = staged.relation_batches();
    product
        .commit_generated_delegate_batch(ast, relation_batches)
        .map_err(|reason| BatchFailureV1::Rejected(reason).into_parse_error())
}

fn prepare_all<'product>(
    product: &'product OpenParserPostpassProductV1,
) -> Result<PreparedDelegatePostpassBatchV1, BatchFailureV1> {
    let hosts = collect_hosts(product)?;
    if hosts.is_empty() {
        return Ok(PreparedDelegatePostpassBatchV1::new(Vec::new()));
    }
    let target_index = product
        .issue_delegate_target_index()
        .map_err(map_target_index_error)?;
    let mut staged = Vec::new();
    for host in hosts {
        let rows = source_rows_for_host(product, host.path)?;
        validate_host_coverage(&host, &rows)?;
        if rows.is_empty() {
            continue;
        }
        staged.push(stage_host(host, rows, &target_index)?);
    }
    Ok(PreparedDelegatePostpassBatchV1::new(staged))
}

fn collect_hosts<'product>(
    product: &'product OpenParserPostpassProductV1,
) -> Result<Vec<HostView<'product>>, BatchFailureV1> {
    let ASTNode::Program { statements, .. } = &product.ast else {
        return Err(BatchFailureV1::Unresolved(
            "postpass AST is not a Program".to_owned(),
        ));
    };
    let mut hosts = Vec::new();
    for (index, statement) in statements.iter().enumerate() {
        let ASTNode::BoxDeclaration {
            field_decls,
            delegates,
            methods,
            is_interface,
            is_record,
            is_static,
            ..
        } = statement
        else {
            continue;
        };
        if *is_interface || *is_record || *is_static {
            return Err(BatchFailureV1::Declined(format!(
                "Box at statement {index} is outside the ordinary C-I0 cohort"
            )));
        }
        let path = product.final_box_paths.get(hosts.len()).ok_or_else(|| {
            BatchFailureV1::Unresolved("ordinary Box source path is missing".to_owned())
        })?;
        hosts.push(HostView {
            path,
            fields: field_decls,
            delegates,
            methods,
        });
    }
    if hosts.len() != product.final_box_paths.len()
        || hosts.len() != product.source_session.prepared_source_seals.len()
    {
        return Err(BatchFailureV1::Unresolved(
            "ordinary host/source-seal/path coverage is incomplete".to_owned(),
        ));
    }
    Ok(hosts)
}

fn source_rows_for_host<'product>(
    product: &'product OpenParserPostpassProductV1,
    path: &SourceBoxDeclarationPathV1,
) -> Result<Vec<&'product DelegateSourceDeclarationV1>, BatchFailureV1> {
    let mut rows = Vec::new();
    for seal in &product.source_session.prepared_source_seals {
        if seal.box_site().path() != path {
            continue;
        }
        if seal
            .delegate_source_declarations()
            .iter()
            .any(|row| row.source_site().box_site().path() != path)
        {
            return Err(BatchFailureV1::Rejected(
                "delegate source row has a foreign host path".to_owned(),
            ));
        }
        for row in seal.delegate_source_declarations() {
            if rows
                .iter()
                .any(|previous: &&DelegateSourceDeclarationV1| same_source_row(previous, row))
            {
                return Err(BatchFailureV1::Rejected(
                    "duplicate delegate source row".to_owned(),
                ));
            }
            rows.push(row);
        }
        return Ok(rows);
    }
    Err(BatchFailureV1::Unresolved(
        "delegate host has no prepared source seal".to_owned(),
    ))
}

fn same_source_row(
    left: &DelegateSourceDeclarationV1,
    right: &DelegateSourceDeclarationV1,
) -> bool {
    left.source_site().box_site().path() == right.source_site().box_site().path()
        && left.source_site().source_member_ordinal() == right.source_site().source_member_ordinal()
        && left.expose_ordinal() == right.expose_ordinal()
}

fn validate_host_coverage(
    host: &HostView<'_>,
    rows: &[&DelegateSourceDeclarationV1],
) -> Result<(), BatchFailureV1> {
    for row in rows {
        let matches = host.delegates.iter().filter(|delegate| {
            let Some(selection) = delegate.explicit_source_selection() else {
                return false;
            };
            delegate.field_name == row.delegate_field_name()
                && row.source_site().matches_ast_selection(selection)
                && (matches!(
                    selection,
                    crate::ast::BoxMethodSourceSelectionV1::SelectedBuildGate { .. }
                ) || delegate.source_member_ordinal()
                    == Some(row.source_site().source_member_ordinal()))
                && delegate
                    .exposes
                    .get(row.expose_ordinal() as usize)
                    .is_some_and(|expose| {
                        expose.source_name == row.source_method_name()
                            && expose.exposed_name == row.exposed_method_name()
                    })
        });
        if matches.count() != 1 {
            return Err(BatchFailureV1::Rejected(
                "delegate source row does not match exactly one AST expose".to_owned(),
            ));
        }
    }

    for delegate in host.delegates {
        if delegate.explicit_source_selection().is_none() {
            return Err(BatchFailureV1::Rejected(
                "AST delegate has no parser-issued source row".to_owned(),
            ));
        }
        for (expose_ordinal, expose) in delegate.exposes.iter().enumerate() {
            let expose_ordinal = u32::try_from(expose_ordinal).map_err(|_| {
                BatchFailureV1::Rejected("delegate expose ordinal overflow".to_owned())
            })?;
            if !rows.iter().any(|row| {
                row.delegate_field_name() == delegate.field_name
                    && row.expose_ordinal() == expose_ordinal
                    && row.source_method_name() == expose.source_name
                    && row.exposed_method_name() == expose.exposed_name
            }) {
                return Err(BatchFailureV1::Rejected(
                    "AST delegate expose has no parser source row".to_owned(),
                ));
            }
        }
    }
    Ok(())
}

fn stage_host(
    host: HostView<'_>,
    rows: Vec<&DelegateSourceDeclarationV1>,
    target_index: &DelegateTargetIndexV1<'_>,
) -> Result<StagedHostBatch, BatchFailureV1> {
    let mut pending = Vec::with_capacity(rows.len());
    for row in rows {
        let target = match target_index.resolve(row) {
            DelegateTargetResolutionV1::Candidate(target) => target,
            DelegateTargetResolutionV1::Declined => {
                return Err(BatchFailureV1::Declined(format!(
                    "target for expose '{}' is outside the C-I0 cohort",
                    row.exposed_method_name()
                )))
            }
            DelegateTargetResolutionV1::Unresolved => {
                return Err(BatchFailureV1::Unresolved(format!(
                    "target evidence for expose '{}' is incomplete",
                    row.exposed_method_name()
                )))
            }
            DelegateTargetResolutionV1::Rejected => {
                return Err(BatchFailureV1::Rejected(format!(
                    "target relation for expose '{}' is contradictory",
                    row.exposed_method_name()
                )))
            }
        };
        let target_method = target_index.method_declaration(&target).ok_or_else(|| {
            BatchFailureV1::Unresolved("target method declaration is missing".to_owned())
        })?;
        let delegate = host
            .delegates
            .iter()
            .find(|delegate| {
                delegate.field_name == row.delegate_field_name()
                    && delegate
                        .exposes
                        .get(row.expose_ordinal() as usize)
                        .is_some_and(|expose| {
                            expose.source_name == row.source_method_name()
                                && expose.exposed_name == row.exposed_method_name()
                        })
            })
            .ok_or_else(|| BatchFailureV1::Rejected("delegate AST row disappeared".to_owned()))?;
        let selection = delegate
            .explicit_source_selection()
            .cloned()
            .ok_or_else(|| {
                BatchFailureV1::Rejected("compatibility delegate entered C-I0".to_owned())
            })?;
        let provenance = BoxMethodGeneratedProvenanceV1::Delegate {
            field_name: row.delegate_field_name().into(),
            exposed_name: row.exposed_method_name().into(),
            selection,
        };
        let generated_method = hakorune_frontend_parser::parser::delegate_lowering
            ::build_forwarding_method_from_declaration(
                row.delegate_field_name(),
                row.exposed_method_name(),
                target_method,
            )
            .map_err(|error| BatchFailureV1::Rejected(format!("forwarder AST: {error}")))?;
        let generated_method = PreparedGeneratedBoxMethodV1::new(
            row.exposed_method_name(),
            generated_method,
            provenance.clone(),
            crate::ast::Span::unknown(),
        )
        .map_err(|error| BatchFailureV1::Rejected(format!("generated method: {error}")))?;
        let explicit = target.method_source_relation();
        pending.push(PendingGeneratedRow {
            expose: row.clone(),
            target: ExistingTargetMethodSourceRefV1::new(
                target.target_box_path().clone(),
                explicit.source_site().clone(),
                explicit.inventory_ordinal(),
                explicit.name(),
            ),
            generated_name_provenance: provenance,
            generated_method,
        });
    }

    let batch = PreparedGeneratedBoxMethodBatchV1::try_new(
        pending.iter().map(|row| row.generated_method.clone()),
    )
    .map_err(|error| BatchFailureV1::Rejected(format!("generated batch: {error}")))?;
    let mut staging_inventory = host.methods.clone();
    let placements = staging_inventory
        .try_commit_generated_batch_with_placements(batch.clone())
        .map_err(|error| BatchFailureV1::Rejected(format!("staging placement: {error}")))?;
    if placements.len() != pending.len()
        || placements
            .iter()
            .zip(pending.iter())
            .any(|(placement, row)| placement.name() != row.expose.exposed_method_name())
    {
        return Err(BatchFailureV1::Rejected(
            "staged placement cardinality/name mismatch".to_owned(),
        ));
    }
    let relations = pending
        .into_iter()
        .zip(placements.iter().cloned())
        .map(|(row, placement)| {
            GeneratedDelegateSourceRelationV1::new(
                host.path.clone(),
                row.expose.source_site().clone(),
                row.expose.expose_ordinal(),
                row.expose.delegate_field_name(),
                row.expose.source_method_name(),
                row.expose.exposed_method_name(),
                row.target.target_box_path().clone(),
                row.target,
                placement,
                row.generated_name_provenance,
            )
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Ok(StagedHostBatch {
        host_path: host.path.clone(),
        batch,
        placements,
        relations,
    })
}

fn apply_staged_batches(
    mut ast: ASTNode,
    final_box_paths: &[SourceBoxDeclarationPathV1],
    staged: &[StagedHostBatch],
) -> Result<ASTNode, BatchFailureV1> {
    let ASTNode::Program { statements, .. } = &mut ast else {
        return Err(BatchFailureV1::Unresolved(
            "postpass AST is not a Program".to_owned(),
        ));
    };
    let mut ordinary_index = 0usize;
    for statement in statements.iter_mut() {
        let ASTNode::BoxDeclaration {
            methods,
            is_interface,
            is_record,
            is_static,
            ..
        } = statement
        else {
            continue;
        };
        if *is_interface || *is_record || *is_static {
            continue;
        }
        let path = final_box_paths.get(ordinary_index).ok_or_else(|| {
            BatchFailureV1::Unresolved("commit Box source path is missing".to_owned())
        })?;
        if let Some(batch) = staged.iter().find(|batch| &batch.host_path == path) {
            let actual = methods
                .try_commit_generated_batch_with_placements(batch.batch.clone())
                .map_err(|error| BatchFailureV1::Rejected(format!("commit placement: {error}")))?;
            if actual.as_ref() != batch.placements.as_ref() {
                return Err(BatchFailureV1::Rejected(
                    "staged-vs-actual placement receipt mismatch".to_owned(),
                ));
            }
        }
        ordinary_index += 1;
    }
    if ordinary_index != final_box_paths.len()
        || staged
            .iter()
            .any(|batch| !final_box_paths.iter().any(|path| path == &batch.host_path))
    {
        return Err(BatchFailureV1::Rejected(
            "commit host/path coverage mismatch".to_owned(),
        ));
    }
    Ok(ast)
}

fn map_target_index_error(error: DelegateTargetIndexErrorV1) -> BatchFailureV1 {
    match error {
        DelegateTargetIndexErrorV1::SourceAlignmentUnavailable => {
            BatchFailureV1::Unresolved("target source alignment is incomplete".to_owned())
        }
        DelegateTargetIndexErrorV1::ForeignBrand
        | DelegateTargetIndexErrorV1::DuplicateBoxPath
        | DelegateTargetIndexErrorV1::DuplicateBoxName
        | DelegateTargetIndexErrorV1::SealPathMismatch
        | DelegateTargetIndexErrorV1::MethodRelationMismatch => {
            BatchFailureV1::Rejected(format!("target index rejected: {error:?}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NyashParser, ParserBuildConfig};

    fn open_product(source: &str) -> OpenParserPostpassProductV1 {
        let config = ParserBuildConfig::default();
        let pre = super::super::normalize_logical_ops(source);
        let mut tokenizer =
            crate::tokenizer::NyashTokenizer::with_grammar_profile(pre, config.grammar_profile);
        let tokens = tokenizer.tokenize().expect("test source must tokenize");
        let mut parser = NyashParser::new(tokens);
        parser.build_config = config;
        let ast = parser.parse_program().expect("test source must parse");
        OpenParserPostpassProductV1::new(
            ast,
            std::mem::take(&mut parser.prepared_source_seals),
            parser.take_source_build_gate_records(),
            parser.take_metadata(),
        )
        .prune_build_gates(&parser)
        .expect("test source must produce an open postpass product")
    }

    #[test]
    fn c_i0_preflights_all_hosts_before_any_ast_mutation() {
        let product = open_product(
            r#"
box Target { run() { return 1 } }
box First {
    target: Target
    delegate target exposes { run as runAlias }
}
box Second {
    target: Target
    delegate target exposes { missing as missingAlias }
}
"#,
        );
        let before = product.ast.clone();

        let error = prepare_all(&product).expect_err("a later host must reject the whole batch");
        assert!(matches!(error, BatchFailureV1::Rejected(_)));
        assert_eq!(product.ast, before);
        let ASTNode::Program { statements, .. } = &product.ast else {
            panic!("test product must remain a Program");
        };
        assert!(statements.iter().all(|statement| {
            !matches!(
                statement,
                ASTNode::BoxDeclaration { methods, .. } if methods.get("runAlias").is_some()
            )
        }));
    }

    #[test]
    fn c_i0_rejects_generated_name_collision_during_staging() {
        let product = open_product(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    runAlias() { return 0 }
    delegate target exposes { run as runAlias }
}
"#,
        );

        let error = prepare_all(&product).expect_err("generated collision must reject staging");
        assert!(matches!(error, BatchFailureV1::Rejected(_)));
    }

    #[test]
    fn c_i0_rejects_staged_vs_actual_placement_mismatch() {
        let product = open_product(
            r#"
box Target { run() { return 1 } }
box First {
    target: Target
    delegate target exposes { run as firstAlias }
}
box Second {
    target: Target
    delegate target exposes { run as secondAlias }
}
"#,
        );
        let prepared = prepare_all(&product).expect("both hosts should stage");
        let mut hosts = prepared.hosts.into_vec();
        assert_eq!(hosts.len(), 2);
        hosts[0].placements = hosts[1].placements.clone();

        let error = apply_staged_batches(product.ast.clone(), &product.final_box_paths, &hosts)
            .expect_err("receipt mismatch must reject before product commit");
        assert!(matches!(error, BatchFailureV1::Rejected(_)));
    }

    #[test]
    fn c_i0_rejects_duplicate_parser_source_rows() {
        let mut product = open_product(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
        );
        let seal = product
            .source_session
            .prepared_source_seals
            .iter_mut()
            .find(|seal| !seal.delegate_source_declarations.is_empty())
            .expect("host source seal must contain one delegate row");
        let row = seal.delegate_source_declarations[0].clone();
        seal.delegate_source_declarations = vec![row.clone(), row].into_boxed_slice();

        let error = prepare_all(&product).expect_err("duplicate source rows must reject");
        assert!(matches!(error, BatchFailureV1::Rejected(_)));
    }
}
