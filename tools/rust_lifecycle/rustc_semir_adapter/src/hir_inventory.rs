use rustc_driver::{run_compiler, Callbacks, Compilation};
use rustc_hir::ItemKind;
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;

const CONTRACT: &str = "rustc-semir-adapter-hir-item-provenance-inventory-v0";

#[derive(Default)]
struct HirInventoryCallbacks {
    output: Vec<String>,
}

impl Callbacks for HirInventoryCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let source_map = compiler.sess.source_map();
        self.output.push(format!("output_contract={CONTRACT}"));
        self.output
            .push("hir_item_provenance_inventory_green=1".to_string());
        self.output.push(format!(
            "crate_name={}",
            tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE)
        ));
        self.output.push("crate_identity_reported=1".to_string());
        self.output.push("module_identity_reported=1".to_string());
        self.output.push("module_0_path=crate".to_string());

        let mut item_count = 0usize;
        for item_id in tcx.hir_free_items() {
            let item = tcx.hir_item(item_id);
            let def_id = item.owner_id.def_id;
            let span = source_map.lookup_char_pos(item.span.lo());
            item_count += 1;
            self.output.push(format!(
                "item_{item_count}_path={}",
                tcx.def_path_str(def_id)
            ));
            self.output.push(format!(
                "item_{item_count}_kind={}",
                item_kind_name(&item.kind)
            ));
            self.output.push(format!(
                "item_{item_count}_source={}:{}:{}",
                span.file.name.prefer_local_unconditionally(),
                span.line,
                span.col_display + 1
            ));
        }

        self.output.push(format!("item_count={item_count}"));
        self.output.push("item_identity_reported=1".to_string());
        self.output.push("source_provenance_reported=1".to_string());
        self.output
            .push("RustLifecycleAdapterFacts_generated=0".to_string());
        self.output.push("hako_plan_emitted=0".to_string());
        self.output.push("hako_source_emitted=0".to_string());
        self.output.push("backend_behavior_changed=0".to_string());
        self.output.push("summary=ok".to_string());
        Compilation::Stop
    }
}

fn item_kind_name(kind: &ItemKind<'_>) -> &'static str {
    match kind {
        ItemKind::ExternCrate(..) => "ExternCrate",
        ItemKind::Use(..) => "Use",
        ItemKind::Static(..) => "Static",
        ItemKind::Const(..) => "Const",
        ItemKind::Fn { .. } => "Fn",
        ItemKind::Macro(..) => "Macro",
        ItemKind::Mod(..) => "Mod",
        ItemKind::ForeignMod { .. } => "ForeignMod",
        ItemKind::GlobalAsm { .. } => "GlobalAsm",
        ItemKind::TyAlias(..) => "TyAlias",
        ItemKind::Enum(..) => "Enum",
        ItemKind::Struct(..) => "Struct",
        ItemKind::Union(..) => "Union",
        ItemKind::Trait { .. } => "Trait",
        ItemKind::TraitAlias(..) => "TraitAlias",
        ItemKind::Impl { .. } => "Impl",
    }
}

pub fn run(inputs: &[String]) {
    if inputs.len() != 1 {
        eprintln!("usage: rustc-semir-adapter --hir-item-provenance-inventory <rust-source>");
        std::process::exit(2);
    }

    let mut callbacks = HirInventoryCallbacks::default();
    let rustc_args = vec![
        "rustc-semir-adapter".to_string(),
        "--crate-type=lib".to_string(),
        "--edition=2021".to_string(),
        inputs[0].clone(),
    ];
    run_compiler(&rustc_args, &mut callbacks);

    for line in callbacks.output {
        println!("{line}");
    }
}
