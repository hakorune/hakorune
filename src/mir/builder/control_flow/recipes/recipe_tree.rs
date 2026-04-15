//! Compatibility owner surface for recipe-tree vocabulary and composers.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipe_tree::{
    build_accum_const_loop_recipe, build_arena_and_if_v2_join_root_from_single_if_stmt,
    build_arena_and_loop_v0_root_from_nested_stmt_only,
    build_arena_and_loop_v0_root_from_single_loop_stmt, build_array_join_recipe,
    build_bool_predicate_scan_recipe, build_char_map_recipe, build_if_phi_join_recipe,
    build_if_v2_join_root, build_loop_break_recipe, build_loop_continue_only_recipe,
    build_loop_simple_while_recipe, build_loop_true_early_exit_recipe, build_scan_with_init_recipe,
    build_split_scan_recipe, build_stmt_only_block, AccumConstLoopRecipe, ArrayJoinRecipe,
    BlockContractKind, BodyId, BoolPredicateScanRecipe, CharMapRecipe, ExitKind, IfContractKind,
    IfMode, IfPhiJoinRecipe, LoopBreakRecipe, LoopContinueOnlyRecipe, LoopKindV0,
    LoopSimpleWhileRecipe, LoopTrueEarlyExitRecipe, LoopV0Features, RecipeBlock, RecipeBodies,
    RecipeComposer, RecipeItem, RecipeMatcher, ScanWithInitRecipe, SplitScanRecipe,
};
