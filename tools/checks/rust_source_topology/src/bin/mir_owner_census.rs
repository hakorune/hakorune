//! Syntax census, not a semantic proof of sole ownership. No compiler build needed.
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use syn::parse::Parser;
use syn::visit::{self, Visit};
use syn::{Attribute, Expr, Meta, Token};

// All counted patterns live here. Macro/scan candidates are explicitly separate
// from confirmed expression constructors and function-call sites.
const PATTERNS: &[(&str, &str)] = &[
    (
        "evaluation_constructors",
        "ExprStruct CheckedExpressionEvaluationV1 (including Self/aliases)",
    ),
    (
        "finish_continuations",
        "fn finish_*_continuation* (free/impl/trait)",
    ),
    ("emitted_projection_types", "struct Emitted*ProjectionV1"),
    (
        "dominator_calls",
        "ExprCall compute_dominators or compute_dominators_with_predecessors",
    ),
    ("predecessor_calls", "ExprCall compute_predecessors"),
    (
        "analysis_function_references",
        "analysis function ExprPath outside a direct call",
    ),
    (
        "raw_binop_constructors",
        "ExprStruct BinOp (all raw domains, conservative integer superset)",
    ),
    (
        "raw_compare_constructors",
        "ExprStruct Compare (all raw domains)",
    ),
    (
        "raw_unaryop_constructors",
        "ExprStruct UnaryOp (all raw domains)",
    ),
    (
        "qualified_alias_constructor_candidates",
        "qualified constructor with an unresolved imported alias spelling",
    ),
    (
        "opaque_operator_forms",
        "unparsed macro tokens with operator/evaluation struct forms",
    ),
    (
        "opaque_owner_forms",
        "unparsed macro tokens naming analysis/finish/projection owners",
    ),
    (
        "definition_scan_candidates",
        "function body: blocks + instructions + dst search (syntactic candidates)",
    ),
];

fn cfg_value(meta: &Meta) -> Option<bool> {
    match meta {
        Meta::Path(path) if path.is_ident("test") => Some(false),
        Meta::List(list) => {
            let args = syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated
                .parse2(list.tokens.clone())
                .ok()?;
            let values: Vec<_> = args.iter().map(cfg_value).collect();
            if list.path.is_ident("all") {
                if values.contains(&Some(false)) {
                    Some(false)
                } else if values.iter().all(|v| *v == Some(true)) {
                    Some(true)
                } else {
                    None
                }
            } else if list.path.is_ident("any") {
                if values.contains(&Some(true)) {
                    Some(true)
                } else if values.iter().all(|v| *v == Some(false)) {
                    Some(false)
                } else {
                    None
                }
            } else if list.path.is_ident("not") && values.len() == 1 {
                values[0].map(|value| !value)
            } else {
                None
            }
        }
        _ => None,
    }
}
fn test_only(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("test")
            || (attr.path().is_ident("cfg")
                && attr
                    .parse_args::<Meta>()
                    .ok()
                    .is_some_and(|meta| cfg_value(&meta) == Some(false)))
    })
}
fn expr_test_only(expression: &Expr) -> bool {
    let attrs = match expression {
        Expr::Array(row) => &row.attrs,
        Expr::Assign(row) => &row.attrs,
        Expr::Async(row) => &row.attrs,
        Expr::Await(row) => &row.attrs,
        Expr::Binary(row) => &row.attrs,
        Expr::Block(row) => &row.attrs,
        Expr::Break(row) => &row.attrs,
        Expr::Call(row) => &row.attrs,
        Expr::Cast(row) => &row.attrs,
        Expr::Closure(row) => &row.attrs,
        Expr::Const(row) => &row.attrs,
        Expr::Continue(row) => &row.attrs,
        Expr::Field(row) => &row.attrs,
        Expr::ForLoop(row) => &row.attrs,
        Expr::Group(row) => &row.attrs,
        Expr::If(row) => &row.attrs,
        Expr::Index(row) => &row.attrs,
        Expr::Infer(row) => &row.attrs,
        Expr::Let(row) => &row.attrs,
        Expr::Lit(row) => &row.attrs,
        Expr::Loop(row) => &row.attrs,
        Expr::Macro(row) => &row.attrs,
        Expr::Match(row) => &row.attrs,
        Expr::MethodCall(row) => &row.attrs,
        Expr::Paren(row) => &row.attrs,
        Expr::Path(row) => &row.attrs,
        Expr::Range(row) => &row.attrs,
        Expr::RawAddr(row) => &row.attrs,
        Expr::Reference(row) => &row.attrs,
        Expr::Repeat(row) => &row.attrs,
        Expr::Return(row) => &row.attrs,
        Expr::Struct(row) => &row.attrs,
        Expr::Try(row) => &row.attrs,
        Expr::TryBlock(row) => &row.attrs,
        Expr::Tuple(row) => &row.attrs,
        Expr::Unary(row) => &row.attrs,
        Expr::Unsafe(row) => &row.attrs,
        Expr::While(row) => &row.attrs,
        Expr::Yield(row) => &row.attrs,
        _ => return false,
    };
    test_only(attrs)
}
fn item_test_only(item: &syn::Item) -> bool {
    let attrs = match item {
        syn::Item::Const(x) => &x.attrs,
        syn::Item::Enum(x) => &x.attrs,
        syn::Item::ExternCrate(x) => &x.attrs,
        syn::Item::Fn(x) => &x.attrs,
        syn::Item::ForeignMod(x) => &x.attrs,
        syn::Item::Impl(x) => &x.attrs,
        syn::Item::Macro(x) => &x.attrs,
        syn::Item::Mod(x) => &x.attrs,
        syn::Item::Static(x) => &x.attrs,
        syn::Item::Struct(x) => &x.attrs,
        syn::Item::Trait(x) => &x.attrs,
        syn::Item::TraitAlias(x) => &x.attrs,
        syn::Item::Type(x) => &x.attrs,
        syn::Item::Union(x) => &x.attrs,
        syn::Item::Use(x) => &x.attrs,
        _ => return false,
    };
    test_only(attrs)
}
fn test_file(path: &Path) -> bool {
    let name = path.file_name().unwrap().to_string_lossy();
    path.components().any(|c| c.as_os_str() == "tests")
        || name == "tests.rs"
        || name == "test_support.rs"
        || name.ends_with("_tests.rs")
        || name.ends_with("_test_support.rs")
}
fn files(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            files(&path, output)?;
        } else if path.extension().is_some_and(|x| x == "rs") && !test_file(&path) {
            output.push(path);
        }
    }
    Ok(())
}
#[derive(Default)]
struct Aliases(BTreeMap<String, BTreeSet<String>>);
impl Aliases {
    fn add(&mut self, tree: &syn::UseTree) {
        match tree {
            syn::UseTree::Path(path) => self.add(&path.tree),
            syn::UseTree::Rename(rename) => {
                self.0
                    .entry(rename.rename.to_string())
                    .or_default()
                    .insert(rename.ident.to_string());
            }
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    self.add(item);
                }
            }
            _ => {}
        }
    }
}
impl<'a> Visit<'a> for Aliases {
    fn visit_expr(&mut self, expression: &'a Expr) {
        if !expr_test_only(expression) {
            visit::visit_expr(self, expression);
        }
    }

    fn visit_item(&mut self, item: &'a syn::Item) {
        if !item_test_only(item) {
            visit::visit_item(self, item);
        }
    }

    fn visit_item_use(&mut self, item: &'a syn::ItemUse) {
        if !test_only(&item.attrs) {
            self.add(&item.tree);
        }
    }
    fn visit_item_type(&mut self, item: &'a syn::ItemType) {
        if !test_only(&item.attrs) {
            if let syn::Type::Path(path) = &*item.ty {
                if let Some(last) = path.path.segments.last() {
                    self.0
                        .entry(item.ident.to_string())
                        .or_default()
                        .insert(last.ident.to_string());
                }
            }
        }
    }
    fn visit_item_fn(&mut self, item: &'a syn::ItemFn) {
        if !test_only(&item.attrs) {
            visit::visit_item_fn(self, item);
        }
    }
    fn visit_item_impl(&mut self, item: &'a syn::ItemImpl) {
        if !test_only(&item.attrs) {
            visit::visit_item_impl(self, item);
        }
    }
    fn visit_impl_item_fn(&mut self, item: &'a syn::ImplItemFn) {
        if !test_only(&item.attrs) {
            visit::visit_impl_item_fn(self, item);
        }
    }
    fn visit_item_mod(&mut self, item: &'a syn::ItemMod) {
        if !test_only(&item.attrs) {
            visit::visit_item_mod(self, item);
        }
    }
}
struct Census<'a> {
    counts: BTreeMap<&'static str, usize>,
    aliases: &'a Aliases,
    impl_type: Option<String>,
    path: &'a Path,
    details: Vec<String>,
}
impl Census<'_> {
    fn hit(&mut self, key: &'static str, span: proc_macro2::Span) {
        *self.counts.get_mut(key).unwrap() += 1;
        self.details.push(format!(
            "{key}\t{}:{}",
            self.path.display(),
            span.start().line
        ));
    }
    fn names(&self, path: &syn::Path) -> BTreeSet<String> {
        let name = path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        if path.segments.len() > 1 {
            BTreeSet::from([name])
        } else {
            self.expand(&name)
        }
    }
    fn expand(&self, name: &str) -> BTreeSet<String> {
        // A conservative alias closure cannot lose a target to an unrelated
        // same-spelled alias in another module; no type inference is claimed.
        let mut pending = vec![if name == "Self" {
            self.impl_type.as_deref().unwrap_or(name).to_owned()
        } else {
            name.to_owned()
        }];
        let mut result = BTreeSet::new();
        while let Some(name) = pending.pop() {
            if !result.insert(name.clone()) {
                continue;
            }
            if let Some(next) = self.aliases.0.get(&name) {
                pending.extend(next.iter().cloned());
            }
        }
        result
    }
    fn analysis_key(name: &str) -> Option<&'static str> {
        match name {
            "compute_dominators" | "compute_dominators_with_predecessors" => {
                Some("dominator_calls")
            }
            "compute_predecessors" => Some("predecessor_calls"),
            _ => None,
        }
    }
    fn function(&mut self, sig: &syn::Signature, body: Option<&syn::Block>) {
        let name = sig.ident.to_string();
        if name.starts_with("finish_") && name.contains("_continuation") {
            self.hit("finish_continuations", sig.ident.span());
        }
        if let Some(body) = body {
            let text = body.to_token_stream().to_string();
            let canonical_index =
                self.path.ends_with("value_origin.rs") && name == "indexed_value_definitions";
            if !canonical_index
                && text.contains("blocks")
                && text.contains("instruction")
                && (text.contains("dst_value") || text.contains("dst"))
            {
                self.hit("definition_scan_candidates", sig.ident.span());
            }
        }
    }
    fn constructor_key(name: &str) -> Option<&'static str> {
        match name {
            "CheckedExpressionEvaluationV1" => Some("evaluation_constructors"),
            "BinOp" => Some("raw_binop_constructors"),
            "Compare" => Some("raw_compare_constructors"),
            "UnaryOp" => Some("raw_unaryop_constructors"),
            _ => None,
        }
    }
    fn opaque_forms(&mut self, tokens: TokenStream, span: proc_macro2::Span) {
        let mut previous = None;
        for token in tokens {
            if let proc_macro2::TokenTree::Ident(ident) = &token {
                let name = ident.to_string();
                if Self::analysis_key(&name).is_some()
                    || (name.starts_with("finish_") && name.contains("_continuation"))
                    || (name.starts_with("Emitted") && name.ends_with("ProjectionV1"))
                {
                    self.hit("opaque_owner_forms", span);
                }
            }

            if let proc_macro2::TokenTree::Group(group) = &token {
                if group.delimiter() == proc_macro2::Delimiter::Brace
                    && previous.as_deref().is_some_and(|name| {
                        self.expand(name)
                            .iter()
                            .any(|name| Self::constructor_key(name).is_some())
                    })
                {
                    self.hit("opaque_operator_forms", span);
                }
                self.opaque_forms(group.stream(), span);
            }
            previous = if let proc_macro2::TokenTree::Ident(id) = token {
                Some(id.to_string())
            } else {
                None
            };
        }
    }
}
impl<'a> Visit<'a> for Census<'_> {
    fn visit_expr(&mut self, expression: &'a Expr) {
        if !expr_test_only(expression) {
            visit::visit_expr(self, expression);
        }
    }

    fn visit_item(&mut self, item: &'a syn::Item) {
        if !item_test_only(item) {
            visit::visit_item(self, item);
        }
    }

    fn visit_item_mod(&mut self, item: &'a syn::ItemMod) {
        if !test_only(&item.attrs) {
            visit::visit_item_mod(self, item);
        }
    }
    fn visit_item_fn(&mut self, item: &'a syn::ItemFn) {
        if !test_only(&item.attrs) {
            self.function(&item.sig, Some(&item.block));
            visit::visit_item_fn(self, item);
        }
    }
    fn visit_impl_item_fn(&mut self, item: &'a syn::ImplItemFn) {
        if !test_only(&item.attrs) {
            self.function(&item.sig, Some(&item.block));
            visit::visit_impl_item_fn(self, item);
        }
    }
    fn visit_trait_item_fn(&mut self, item: &'a syn::TraitItemFn) {
        if !test_only(&item.attrs) {
            self.function(&item.sig, item.default.as_ref());
            visit::visit_trait_item_fn(self, item);
        }
    }
    fn visit_item_impl(&mut self, item: &'a syn::ItemImpl) {
        if test_only(&item.attrs) {
            return;
        }
        let old = self.impl_type.take();
        self.impl_type = if let syn::Type::Path(ty) = &*item.self_ty {
            ty.path.segments.last().map(|s| s.ident.to_string())
        } else {
            None
        };
        visit::visit_item_impl(self, item);
        self.impl_type = old;
    }
    fn visit_item_struct(&mut self, item: &'a syn::ItemStruct) {
        if test_only(&item.attrs) {
            return;
        }
        let name = item.ident.to_string();
        if name.starts_with("Emitted") && name.ends_with("ProjectionV1") {
            self.hit("emitted_projection_types", item.ident.span());
        }
        visit::visit_item_struct(self, item);
    }
    fn visit_expr_struct(&mut self, item: &'a syn::ExprStruct) {
        if test_only(&item.attrs) {
            return;
        }
        if item.path.segments.len() > 1 {
            let last = item.path.segments.last().unwrap().ident.to_string();
            if Self::constructor_key(&last).is_none()
                && self
                    .expand(&last)
                    .iter()
                    .any(|name| Self::constructor_key(name).is_some())
            {
                self.hit(
                    "qualified_alias_constructor_candidates",
                    item.path.segments.last().unwrap().ident.span(),
                );
            }
        }
        let keys: BTreeSet<_> = self
            .names(&item.path)
            .iter()
            .filter_map(|name| Self::constructor_key(name))
            .collect();
        for key in keys {
            self.hit(key, item.path.segments.last().unwrap().ident.span());
        }
        visit::visit_expr_struct(self, item);
    }
    fn visit_expr_call(&mut self, item: &'a syn::ExprCall) {
        if test_only(&item.attrs) {
            return;
        }
        if let Expr::Path(path) = &*item.func {
            let keys: BTreeSet<_> = self
                .names(&path.path)
                .iter()
                .filter_map(|name| Self::analysis_key(name))
                .collect();
            if !keys.is_empty() {
                for key in keys {
                    self.hit(key, path.path.segments.last().unwrap().ident.span());
                }
                for arg in &item.args {
                    self.visit_expr(arg);
                }
                return;
            }
        }
        visit::visit_expr_call(self, item);
    }
    fn visit_expr_method_call(&mut self, item: &'a syn::ExprMethodCall) {
        if test_only(&item.attrs) {
            return;
        }
        if let Some(key) = Self::analysis_key(&item.method.to_string()) {
            self.hit(key, item.method.span());
        }
        visit::visit_expr_method_call(self, item);
    }
    fn visit_expr_block(&mut self, item: &'a syn::ExprBlock) {
        if !test_only(&item.attrs) {
            visit::visit_expr_block(self, item);
        }
    }
    fn visit_local(&mut self, item: &'a syn::Local) {
        if !test_only(&item.attrs) {
            visit::visit_local(self, item);
        }
    }
    fn visit_expr_path(&mut self, item: &'a syn::ExprPath) {
        if test_only(&item.attrs) {
            return;
        }
        let last = item.path.segments.last().unwrap().ident.to_string();
        if self
            .expand(&last)
            .iter()
            .any(|name| Self::analysis_key(name).is_some())
        {
            self.hit(
                "analysis_function_references",
                item.path.segments.last().unwrap().ident.span(),
            );
        }
        visit::visit_expr_path(self, item);
    }
    fn visit_macro(&mut self, item: &'a syn::Macro) {
        // Parsing expressions never turns Pat::Struct into Expr::Struct.
        if let Ok(expr) = syn::parse2::<Expr>(item.tokens.clone()) {
            self.visit_expr(&expr);
            return;
        }
        if let Ok(args) = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated
            .parse2(item.tokens.clone())
        {
            for expr in args {
                self.visit_expr(&expr);
            }
            return;
        }
        let tokens = &item.tokens;
        if let Ok(block) = syn::parse2::<syn::Block>(quote::quote!({ #tokens })) {
            self.visit_block(&block);
            return;
        }
        self.opaque_forms(
            item.tokens.clone(),
            item.path.segments.last().unwrap().ident.span(),
        );
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("[mir-owner-count] {error}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let root = PathBuf::from(
        args.next()
            .ok_or("usage: mir_owner_census ROOT [--details]")?,
    );
    let details = args.next().is_some_and(|arg| arg == "--details");
    let mut paths = Vec::new();
    files(&root.join("src/mir"), &mut paths)?;
    paths.sort();
    let mut totals: BTreeMap<_, usize> = PATTERNS.iter().map(|(key, _)| (*key, 0)).collect();
    let mut aliases = Aliases::default();
    for path in &paths {
        let source = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let file = syn::parse_file(&source).map_err(|e| format!("{}: {e}", path.display()))?;
        aliases.visit_file(&file);
    }
    for path in paths {
        let source =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let file = syn::parse_file(&source).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut scan = Census {
            counts: PATTERNS.iter().map(|(key, _)| (*key, 0)).collect(),
            aliases: &aliases,
            impl_type: None,
            path: &path,
            details: Vec::new(),
        };
        scan.visit_file(&file);
        for (key, count) in scan.counts {
            *totals.get_mut(key).unwrap() += count;
        }
        if details {
            for line in scan.details {
                eprintln!("{line}");
            }
        }
    }
    for (key, pattern) in PATTERNS {
        println!("{key}\t{}\t{pattern}", totals[key]);
    }
    Ok(())
}
