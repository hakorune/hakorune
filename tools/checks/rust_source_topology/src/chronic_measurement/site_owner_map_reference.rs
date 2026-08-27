use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Output};

use syn::{ImplItem, Item, PathArguments, Type};

use super::error::{ChronicScanErrorV1, SiteOwnerMapReferenceFailureV1};
use super::site_owner_map::{
    validate_site_owner_map_toml_syntax_v1, ChronicSiteOwnerMapRowV1, ChronicSiteOwnerMapV1,
};

#[derive(Debug, Clone, Copy)]
pub struct SiteOwnerMapReferenceContextV1<'a> {
    pub repository_root: &'a Path,
    pub review_head: &'a str,
}

pub fn validate_site_owner_map_toml_with_references_v1(
    map_input: &str,
    observation_receipt_input: &str,
    expected_source_commit: &str,
    context: SiteOwnerMapReferenceContextV1<'_>,
) -> Result<ChronicSiteOwnerMapV1, ChronicScanErrorV1> {
    let map = validate_site_owner_map_toml_syntax_v1(
        map_input,
        observation_receipt_input,
        expected_source_commit,
    )?;
    let mut resolver = PinnedReferenceResolver::new(context)?;
    for row in &map.sites {
        resolver.resolve_row(row)?;
    }
    Ok(map)
}

struct PinnedReferenceResolver<'a> {
    repository_root: &'a Path,
    review_head_oid: String,
    revision_oids: BTreeMap<String, String>,
    blobs: BTreeMap<(String, String), String>,
}

impl<'a> PinnedReferenceResolver<'a> {
    fn new(context: SiteOwnerMapReferenceContextV1<'a>) -> Result<Self, ChronicScanErrorV1> {
        validate_revision_syntax(context.review_head).map_err(|failure| {
            reference_error(
                context.review_head,
                failure,
                "review_head must be a full lowercase commit revision",
            )
        })?;
        let review_head_oid = resolve_commit(
            context.repository_root,
            context.review_head,
            context.review_head,
        )?;
        Ok(Self {
            repository_root: context.repository_root,
            review_head_oid,
            revision_oids: BTreeMap::new(),
            blobs: BTreeMap::new(),
        })
    }

    fn resolve_row(&mut self, row: &ChronicSiteOwnerMapRowV1) -> Result<(), ChronicScanErrorV1> {
        self.resolve_reference(&row.owner_ref)?;
        for reference in &row.evidence_refs {
            self.resolve_reference(reference)?;
        }
        if row.successor_status == "required" {
            self.resolve_reference(&row.successor_ref)?;
        }
        Ok(())
    }

    fn resolve_reference(&mut self, reference: &str) -> Result<(), ChronicScanErrorV1> {
        let parts = parse_reference(reference)?;
        let revision_oid = self.resolve_revision(&parts.revision, reference)?;
        let cache_key = (revision_oid.clone(), parts.path.clone());
        if !self.blobs.contains_key(&cache_key) {
            let source =
                load_tracked_blob(self.repository_root, &revision_oid, &parts.path, reference)?;
            self.blobs.insert(cache_key.clone(), source);
        }
        let Some(source) = self.blobs.get(&cache_key) else {
            return Err(reference_error(
                reference,
                SiteOwnerMapReferenceFailureV1::RepositoryUnavailable,
                "internal pinned-blob cache did not retain the loaded source",
            ));
        };
        resolve_anchor(reference, &parts.anchor, source)
    }

    fn resolve_revision(
        &mut self,
        revision: &str,
        reference: &str,
    ) -> Result<String, ChronicScanErrorV1> {
        if let Some(oid) = self.revision_oids.get(revision) {
            return Ok(oid.clone());
        }
        let oid = resolve_commit(self.repository_root, revision, reference)?;
        let output = run_git(
            self.repository_root,
            &[
                "merge-base".into(),
                "--is-ancestor".into(),
                oid.clone(),
                self.review_head_oid.clone(),
            ],
        )
        .map_err(|detail| {
            reference_error(
                reference,
                SiteOwnerMapReferenceFailureV1::RepositoryUnavailable,
                detail,
            )
        })?;
        if !output.status.success() {
            return Err(reference_error(
                reference,
                SiteOwnerMapReferenceFailureV1::RevisionNotAncestor,
                "pinned revision is not an ancestor of review_head",
            ));
        }
        self.revision_oids.insert(revision.to_string(), oid.clone());
        Ok(oid)
    }
}

struct ReferenceParts {
    path: String,
    anchor: String,
    revision: String,
}

fn parse_reference(reference: &str) -> Result<ReferenceParts, ChronicScanErrorV1> {
    let Some((body, revision)) = reference.rsplit_once('@') else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::InvalidRevision,
            "missing revision",
        ));
    };
    validate_revision_syntax(revision).map_err(|failure| {
        reference_error(
            reference,
            failure,
            "revision must be 40 lowercase hex characters",
        )
    })?;
    let Some((path, anchor)) = body.split_once('#') else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::AnchorUnsupported,
            "missing anchor",
        ));
    };
    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path.split('/').any(|part| part == "..")
        || path.chars().any(char::is_whitespace)
        || anchor.is_empty()
        || anchor.contains('#')
        || anchor.contains('@')
        || anchor.chars().any(char::is_whitespace)
    {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::AnchorUnsupported,
            "invalid tracked path or anchor",
        ));
    }
    Ok(ReferenceParts {
        path: path.to_string(),
        anchor: anchor.to_string(),
        revision: revision.to_string(),
    })
}

fn validate_revision_syntax(revision: &str) -> Result<(), SiteOwnerMapReferenceFailureV1> {
    if revision.len() != 40
        || !revision
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(SiteOwnerMapReferenceFailureV1::InvalidRevision);
    }
    Ok(())
}

fn resolve_commit(
    repository_root: &Path,
    revision: &str,
    reference: &str,
) -> Result<String, ChronicScanErrorV1> {
    let revision_expr = format!("{revision}^{{commit}}");
    let output = run_git(
        repository_root,
        &[
            "rev-parse".into(),
            "--verify".into(),
            "--quiet".into(),
            revision_expr,
        ],
    )
    .map_err(|detail| {
        reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RepositoryUnavailable,
            detail,
        )
    })?;
    if !output.status.success() {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RevisionNotCommit,
            "git could not resolve a commit object at the pinned revision",
        ));
    }
    let oid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if oid.len() != 40
        || !oid
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RevisionNotCommit,
            "git returned a non-commit object id",
        ));
    }
    Ok(oid)
}

fn load_tracked_blob(
    repository_root: &Path,
    revision_oid: &str,
    path: &str,
    reference: &str,
) -> Result<String, ChronicScanErrorV1> {
    let output = run_git(
        repository_root,
        &[
            "ls-tree".into(),
            "-z".into(),
            revision_oid.to_string(),
            "--".into(),
            path.to_string(),
        ],
    )
    .map_err(|detail| {
        reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RepositoryUnavailable,
            detail,
        )
    })?;
    if !output.status.success() {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::PathMissing,
            "git ls-tree could not inspect the pinned path",
        ));
    }
    let entry = output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
        .find_map(|entry| parse_tree_entry(entry, path));
    let Some((mode, kind, oid)) = entry else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::PathMissing,
            "pinned path is absent from the tracked tree",
        ));
    };
    if kind != "blob" || !matches!(mode.as_str(), "100644" | "100755") {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::PathNotBlob,
            format!("pinned path is not a regular blob: mode={mode} kind={kind}"),
        ));
    }
    let output =
        run_git(repository_root, &["cat-file".into(), "blob".into(), oid]).map_err(|detail| {
            reference_error(
                reference,
                SiteOwnerMapReferenceFailureV1::RepositoryUnavailable,
                detail,
            )
        })?;
    if !output.status.success() {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::PathNotBlob,
            "git cat-file could not read the tracked blob",
        ));
    }
    String::from_utf8(output.stdout).map_err(|_| {
        reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::BlobNonUtf8,
            "pinned blob is not valid UTF-8",
        )
    })
}

fn parse_tree_entry(entry: &[u8], expected_path: &str) -> Option<(String, String, String)> {
    let tab = entry.iter().position(|byte| *byte == b'\t')?;
    if entry.get(tab + 1..)? != expected_path.as_bytes() {
        return None;
    }
    let metadata = std::str::from_utf8(&entry[..tab]).ok()?;
    let mut fields = metadata.split(' ');
    let mode = fields.next()?.to_string();
    let kind = fields.next()?.to_string();
    let oid = fields.next()?.to_string();
    Some((mode, kind, oid))
}

fn resolve_anchor(reference: &str, anchor: &str, source: &str) -> Result<(), ChronicScanErrorV1> {
    if let Some(range) = anchor.strip_prefix("range:") {
        return resolve_range(reference, range, source);
    }
    if let Some(symbol) = anchor.strip_prefix("symbol:") {
        return resolve_symbol(reference, symbol, source);
    }
    if anchor.starts_with("edge:") {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::EdgeUnsupported,
            "edge anchors need a concrete unique registration owner",
        ));
    }
    Err(reference_error(
        reference,
        SiteOwnerMapReferenceFailureV1::AnchorUnsupported,
        "anchor kind is not supported",
    ))
}

fn resolve_range(reference: &str, range: &str, source: &str) -> Result<(), ChronicScanErrorV1> {
    let Some((start, end)) = range.split_once('-') else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RangeOutOfBounds,
            "range must be start-end",
        ));
    };
    let Ok(start) = start.parse::<usize>() else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RangeOutOfBounds,
            "range start is not a positive line number",
        ));
    };
    let Ok(end) = end.parse::<usize>() else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RangeOutOfBounds,
            "range end is not a positive line number",
        ));
    };
    let line_count = source.lines().count();
    if start == 0 || end == 0 || start > end || end > line_count {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::RangeOutOfBounds,
            format!("range {start}-{end} is outside 1..={line_count}"),
        ));
    }
    Ok(())
}

fn resolve_symbol(reference: &str, symbol: &str, source: &str) -> Result<(), ChronicScanErrorV1> {
    let Ok(path) = syn::parse_str::<syn::Path>(symbol) else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::AnchorUnsupported,
            "symbol anchor is not a Rust path",
        ));
    };
    if path.leading_colon.is_some()
        || path
            .segments
            .iter()
            .any(|segment| !matches!(&segment.arguments, PathArguments::None))
    {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::AnchorUnsupported,
            "symbol anchor must use plain identifiers",
        ));
    }
    let Ok(file) = syn::parse_file(source) else {
        return Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::AnchorUnsupported,
            "pinned blob is not a parseable Rust source file",
        ));
    };
    let segments: Vec<_> = path.segments.iter().collect();
    let count = match segments.as_slice() {
        [name] => count_bare_items(&file.items, &name.ident.to_string()),
        [owner, member] => count_impl_members(
            &file.items,
            &owner.ident.to_string(),
            &member.ident.to_string(),
        ),
        _ => {
            return Err(reference_error(
                reference,
                SiteOwnerMapReferenceFailureV1::AnchorUnsupported,
                "qualified symbol anchors use Type::member",
            ))
        }
    };
    match count {
        1 => Ok(()),
        0 => Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::SymbolMissing,
            format!("symbol {symbol} resolved zero items"),
        )),
        count => Err(reference_error(
            reference,
            SiteOwnerMapReferenceFailureV1::SymbolAmbiguous,
            format!("symbol {symbol} resolved {count} items"),
        )),
    }
}

fn count_bare_items(items: &[Item], name: &str) -> usize {
    items
        .iter()
        .map(|item| {
            let own = item_name(item).is_some_and(|ident| ident == name);
            let nested = match item {
                Item::Mod(module) => module
                    .content
                    .as_ref()
                    .map_or(0, |(_, items)| count_bare_items(items, name)),
                _ => 0,
            };
            usize::from(own) + nested
        })
        .sum()
}

fn count_impl_members(items: &[Item], owner: &str, member: &str) -> usize {
    items
        .iter()
        .map(|item| {
            let own = match item {
                Item::Impl(item_impl)
                    if type_name(&item_impl.self_ty).as_deref() == Some(owner) =>
                {
                    item_impl
                        .items
                        .iter()
                        .filter(|item| impl_item_name(item).is_some_and(|ident| ident == member))
                        .count()
                }
                _ => 0,
            };
            let nested = match item {
                Item::Mod(module) => module
                    .content
                    .as_ref()
                    .map_or(0, |(_, items)| count_impl_members(items, owner, member)),
                _ => 0,
            };
            own + nested
        })
        .sum()
}

fn item_name(item: &Item) -> Option<&syn::Ident> {
    match item {
        Item::Const(item) => Some(&item.ident),
        Item::Enum(item) => Some(&item.ident),
        Item::ExternCrate(item) => Some(&item.ident),
        Item::Fn(item) => Some(&item.sig.ident),
        Item::Macro(item) => item.ident.as_ref(),
        Item::Mod(item) => Some(&item.ident),
        Item::Static(item) => Some(&item.ident),
        Item::Struct(item) => Some(&item.ident),
        Item::Trait(item) => Some(&item.ident),
        Item::TraitAlias(item) => Some(&item.ident),
        Item::Type(item) => Some(&item.ident),
        Item::Union(item) => Some(&item.ident),
        _ => None,
    }
}

fn impl_item_name(item: &ImplItem) -> Option<&syn::Ident> {
    match item {
        ImplItem::Const(item) => Some(&item.ident),
        ImplItem::Fn(item) => Some(&item.sig.ident),
        ImplItem::Type(item) => Some(&item.ident),
        _ => None,
    }
}

fn type_name(ty: &Type) -> Option<String> {
    let Type::Path(path) = ty else {
        return None;
    };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

fn run_git(repository_root: &Path, args: &[String]) -> Result<Output, String> {
    Command::new("git")
        .arg("-C")
        .arg(repository_root)
        .args(args)
        .output()
        .map_err(|error| format!("git command failed to start: {error}"))
}

fn reference_error(
    reference: &str,
    failure: SiteOwnerMapReferenceFailureV1,
    detail: impl Into<String>,
) -> ChronicScanErrorV1 {
    ChronicScanErrorV1::SiteOwnerMapReference {
        reference: reference.to_string(),
        failure,
        detail: detail.into(),
    }
}
