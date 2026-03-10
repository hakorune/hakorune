use super::super::ast::{CatchV0, StmtV0};
use super::{lower_stmt_list_with_vars, new_block, BridgeEnv, LoopContext};
use crate::ast::Span;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

fn merge_throw_snapshot_vars_into_catch(
    f: &mut MirFunction,
    catch_bb: BasicBlockId,
    base_vars: &BTreeMap<String, ValueId>,
    incoming_vars: &[(BasicBlockId, BTreeMap<String, ValueId>)],
    catch_vars: &mut BTreeMap<String, ValueId>,
) -> Result<(), String> {
    use std::collections::{BTreeSet, HashSet};

    if incoming_vars.is_empty() {
        return Ok(());
    }

    let mut names: BTreeSet<String> = BTreeSet::new();
    for (_, map) in incoming_vars {
        names.extend(map.keys().cloned());
    }

    for name in names {
        let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
        for (pred_bb, map) in incoming_vars {
            if let Some(&value) = map.get(&name) {
                inputs.push((*pred_bb, value));
            } else if let Some(&base_value) = base_vars.get(&name) {
                inputs.push((*pred_bb, base_value));
            }
        }

        inputs.sort_by_key(|(bbid, _)| bbid.0);
        inputs.dedup_by_key(|(bbid, _)| bbid.0);
        if inputs.is_empty() {
            continue;
        }

        let uniq: HashSet<ValueId> = inputs.iter().map(|(_, v)| *v).collect();
        if uniq.len() == 1 {
            catch_vars.insert(name, inputs[0].1);
            continue;
        }

        let phi_dst = f.next_value_id();
        crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
            f,
            catch_bb,
            phi_dst,
            inputs,
            Span::unknown(),
        )?;
        catch_vars.insert(name, phi_dst);
    }

    Ok(())
}

pub(super) fn lower_try_stmt(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    try_body: &[StmtV0],
    catches: &[CatchV0],
    finally: &[StmtV0],
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    let try_enabled = std::env::var("NYASH_BRIDGE_TRY_ENABLE").ok().as_deref() == Some("1");
    // Result-mode lowering: structured blocks without MIR Throw/Catch
    if env.try_result_mode {
        // Only support 0 or 1 catch for MVP
        let has_catch = !catches.is_empty();
        if catches.len() > 1 {
            // Fallback to safe lowering (ignore catches) for multi-catch
            let mut tmp_vars = vars.clone();
            let mut next_bb = super::lower_stmt_list_with_vars(
                f,
                cur_bb,
                try_body,
                &mut tmp_vars,
                loop_stack,
                env,
            )?;
            if !finally.is_empty() {
                next_bb = super::lower_stmt_list_with_vars(
                    f,
                    next_bb,
                    finally,
                    &mut tmp_vars,
                    loop_stack,
                    env,
                )?;
            }
            *vars = tmp_vars;
            return Ok(next_bb);
        }

        let base_vars = vars.clone();
        let try_bb = new_block(f);
        let catch_bb_opt = if has_catch { Some(new_block(f)) } else { None };
        let finally_bb = if !finally.is_empty() {
            Some(new_block(f))
        } else {
            None
        };
        let exit_bb = new_block(f);

        f.set_jump_terminator(cur_bb, try_bb)?;
        if let Some(succ) = f.get_block_mut(try_bb) {
            succ.add_predecessor(cur_bb);
        }

        // Install thread-local throw context so nested throw expressions jump to catch_bb
        if has_catch {
            let catch_bb = catch_bb_opt.expect("catch_bb must exist when has_catch");
            if crate::config::env::cli_verbose() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[Bridge] try_result_mode: set ThrowCtx (catch_bb={:?})",
                    catch_bb
                ));
            }
            super::throw_ctx::set(catch_bb);
        } else if crate::config::env::cli_verbose() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[Bridge] try_result_mode: no catch present; ThrowCtx not set"));
        }
        let mut try_vars = base_vars.clone();
        let try_end =
            super::lower_stmt_list_with_vars(f, try_bb, try_body, &mut try_vars, loop_stack, env)?;
        let try_end_jumps_to_catch = if has_catch {
            let catch_bb = catch_bb_opt.expect("catch_bb must exist when has_catch");
            matches!(
                f.blocks.get(&try_end).and_then(|b| b.terminator.as_ref()),
                Some(MirInstruction::Jump { target, .. }) if *target == catch_bb
            )
        } else {
            false
        };
        // Take recorded incoming exceptions
        let (incoming_exc, incoming_vars) = if has_catch {
            if let Some(ctx) = super::throw_ctx::take() {
                (ctx.incoming, ctx.incoming_vars)
            } else {
                (Vec::new(), Vec::new())
            }
        } else {
            (Vec::new(), Vec::new())
        };
        let target = finally_bb.unwrap_or(exit_bb);
        f.set_jump_terminator(try_end, target)?;
        if !try_end_jumps_to_catch {
            if let Some(succ) = f.get_block_mut(target) {
                succ.add_predecessor(try_end);
            }
        }
        let try_branch_vars = if try_end_jumps_to_catch {
            None
        } else {
            Some(try_vars.clone())
        };

        // Lower catch block if present and reachable
        let (catch_end, catch_branch_vars) = if has_catch {
            let catch_bb = catch_bb_opt.expect("catch_bb must exist when has_catch");
            // Prepare catch var mapping; optionally bind param via PHI from incoming throw sites.
            let catch_clause = &catches[0];
            let mut catch_vars = base_vars.clone();
            merge_throw_snapshot_vars_into_catch(
                f,
                catch_bb,
                &base_vars,
                &incoming_vars,
                &mut catch_vars,
            )?;
            if let Some(param) = &catch_clause.param {
                // フェーズM.2: PHI統一処理（no_phi条件削除）
                if !incoming_exc.is_empty() {
                    let phi_dst = f.next_value_id();
                    if let Some(_bb) = f.get_block_mut(catch_bb) {
                        let mut inputs = incoming_exc.clone();
                        inputs.sort_by_key(|(bbid, _)| bbid.0);
                        crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                            f,
                            catch_bb,
                            phi_dst,
                            inputs,
                            Span::unknown(),
                        )?;
                    }
                    catch_vars.insert(param.clone(), phi_dst);
                }
            }
            let end = super::lower_stmt_list_with_vars(
                f,
                catch_bb,
                &catch_clause.body,
                &mut catch_vars,
                loop_stack,
                env,
            )?;
            let target = finally_bb.unwrap_or(exit_bb);
            f.set_jump_terminator(end, target)?;
            if let Some(succ) = f.get_block_mut(target) {
                succ.add_predecessor(end);
            }
            (end, catch_vars)
        } else {
            (try_end, base_vars.clone())
        };

        // Finally or direct exit; merge variables across branches
        use std::collections::{BTreeSet, HashSet};
        if let Some(finally_block) = finally_bb {
            // Compute merged var map from try_end + catch_end (if has_catch)
        let mut branch_vars: Vec<(BasicBlockId, BTreeMap<String, ValueId>)> = Vec::new();
        if let Some(map) = try_branch_vars.clone() {
            branch_vars.push((try_end, map));
        }
        if has_catch {
            branch_vars.push((catch_end, catch_branch_vars.clone()));
        }
        let mut names: BTreeSet<String> = base_vars.keys().cloned().collect();
        for (_, map) in &branch_vars {
            names.extend(map.keys().cloned());
        }
            let mut merged_vars = base_vars.clone();
            let mut phi_entries: Vec<(ValueId, Vec<(BasicBlockId, ValueId)>)> = Vec::new();
            for name in names {
                let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
                for (bbid, map) in &branch_vars {
                    if let Some(&v) = map.get(&name) {
                        inputs.push((*bbid, v));
                    }
                }
                if inputs.is_empty() {
                    if let Some(&b) = base_vars.get(&name) {
                        merged_vars.insert(name.clone(), b);
                    }
                    continue;
                }
                let uniq: HashSet<ValueId> = inputs.iter().map(|(_, v)| *v).collect();
                if uniq.len() == 1 {
                    merged_vars.insert(name.clone(), inputs[0].1);
                    continue;
                }
                let dst = f.next_value_id();
                inputs.sort_by_key(|(bbid, _)| bbid.0);
                phi_entries.push((dst, inputs));
                merged_vars.insert(name.clone(), dst);
            }
            if let Some(_bb) = f.get_block_mut(finally_block) {
                for (dst, inputs) in phi_entries {
                    crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                        f,
                        finally_block,
                        dst,
                        inputs,
                        Span::unknown(),
                    )?;
                }
            }
            let mut finally_vars = merged_vars.clone();
            let final_end = super::lower_stmt_list_with_vars(
                f,
                finally_block,
                finally,
                &mut finally_vars,
                loop_stack,
                env,
            )?;
            f.set_jump_terminator(final_end, exit_bb)?;
            if let Some(succ) = f.get_block_mut(exit_bb) {
                succ.add_predecessor(final_end);
            }
            *vars = finally_vars;
            return Ok(exit_bb);
        } else {
            // Merge at exit_bb
            let mut branch_vars: Vec<(BasicBlockId, BTreeMap<String, ValueId>)> = Vec::new();
            if let Some(map) = try_branch_vars {
                branch_vars.push((try_end, map));
            }
            if has_catch {
                branch_vars.push((catch_end, catch_branch_vars));
            }
            let mut names: BTreeSet<String> = base_vars.keys().cloned().collect();
            for (_, map) in &branch_vars {
                names.extend(map.keys().cloned());
            }
            let mut merged_vars = base_vars.clone();
            let mut phi_entries: Vec<(ValueId, Vec<(BasicBlockId, ValueId)>)> = Vec::new();
            for name in names {
                let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
                for (bbid, map) in &branch_vars {
                    if let Some(&v) = map.get(&name) {
                        inputs.push((*bbid, v));
                    }
                }
                if inputs.is_empty() {
                    if let Some(&b) = base_vars.get(&name) {
                        merged_vars.insert(name.clone(), b);
                    }
                    continue;
                }
                let uniq: HashSet<ValueId> = inputs.iter().map(|(_, v)| *v).collect();
                if uniq.len() == 1 {
                    merged_vars.insert(name.clone(), inputs[0].1);
                    continue;
                }
                let dst = f.next_value_id();
                inputs.sort_by_key(|(bbid, _)| bbid.0);
                phi_entries.push((dst, inputs));
                merged_vars.insert(name.clone(), dst);
            }
            if let Some(_bb) = f.get_block_mut(exit_bb) {
                for (dst, inputs) in phi_entries {
                    crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                        f,
                        exit_bb,
                        dst,
                        inputs,
                        Span::unknown(),
                    )?;
                }
            }
            *vars = merged_vars;
            return Ok(exit_bb);
        }
    } else if !try_enabled || catches.is_empty() || catches.len() > 1 {
        let mut tmp_vars = vars.clone();
        let mut next_bb =
            lower_stmt_list_with_vars(f, cur_bb, try_body, &mut tmp_vars, loop_stack, env)?;
        if !finally.is_empty() {
            next_bb =
                lower_stmt_list_with_vars(f, next_bb, finally, &mut tmp_vars, loop_stack, env)?;
        }
        *vars = tmp_vars;
        return Ok(next_bb);
    }

    let base_vars = vars.clone();
    let try_bb = new_block(f);
    let catch_clause = &catches[0];
    let catch_bb = new_block(f);
    let finally_bb = if !finally.is_empty() {
        let id = new_block(f);
        Some(id)
    } else {
        None
    };
    let exit_bb = new_block(f);
    let handler_target = finally_bb.unwrap_or(exit_bb);
    let exception_value = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.add_instruction(MirInstruction::Catch {
            exception_type: catch_clause.type_hint.clone(),
            exception_value,
            handler_bb: catch_bb,
        });
    }
    f.set_jump_terminator(cur_bb, try_bb)?;
    let mut try_vars = vars.clone();
    let try_end = lower_stmt_list_with_vars(f, try_bb, try_body, &mut try_vars, loop_stack, env)?;
    f.set_jump_terminator(try_end, handler_target)?;
    let try_branch_vars = try_vars.clone();

    let mut catch_vars = base_vars.clone();
    if let Some(param) = &catch_clause.param {
        catch_vars.insert(param.clone(), exception_value);
    }
    let catch_end = lower_stmt_list_with_vars(
        f,
        catch_bb,
        &catch_clause.body,
        &mut catch_vars,
        loop_stack,
        env,
    )?;
    if let Some(param) = &catch_clause.param {
        catch_vars.remove(param);
    }
    f.set_jump_terminator(catch_end, handler_target)?;
    let catch_branch_vars = catch_vars.clone();

    use std::collections::{BTreeSet, HashSet};
    let branch_vars = vec![(try_end, try_branch_vars), (catch_end, catch_branch_vars)];
    if let Some(finally_block) = finally_bb {
        let names: BTreeSet<String> = {
            let mut set: BTreeSet<String> = base_vars.keys().cloned().collect();
            for (_, map) in &branch_vars {
                set.extend(map.keys().cloned());
            }
            set
        };
        let mut merged_vars = base_vars.clone();
        let mut phi_entries: Vec<(ValueId, Vec<(BasicBlockId, ValueId)>)> = Vec::new();
        for name in names {
            let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
            for (bbid, map) in &branch_vars {
                if let Some(&val) = map.get(&name) {
                    inputs.push((*bbid, val));
                }
            }
            if inputs.is_empty() {
                if let Some(&base_val) = base_vars.get(&name) {
                    merged_vars.insert(name.clone(), base_val);
                }
                continue;
            }
            let unique: HashSet<ValueId> = inputs.iter().map(|(_, v)| *v).collect();
            if unique.len() == 1 {
                merged_vars.insert(name.clone(), inputs[0].1);
                continue;
            }
            let dst = f.next_value_id();
            inputs.sort_by_key(|(bbid, _)| bbid.0);
            phi_entries.push((dst, inputs));
            merged_vars.insert(name.clone(), dst);
        }
        // フェーズM.2: PHI統一処理（no_phi分岐削除）
        if let Some(_bb) = f.get_block_mut(finally_block) {
            for (dst, inputs) in phi_entries {
                crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                    f,
                    finally_block,
                    dst,
                    inputs,
                    Span::unknown(),
                )?;
            }
        }
        let mut finally_vars = merged_vars.clone();
        let final_end = lower_stmt_list_with_vars(
            f,
            finally_block,
            finally,
            &mut finally_vars,
            loop_stack,
            env,
        )?;
        f.set_jump_terminator(final_end, exit_bb)?;
        *vars = finally_vars;
        Ok(exit_bb)
    } else {
        let names: BTreeSet<String> = {
            let mut set: BTreeSet<String> = base_vars.keys().cloned().collect();
            for (_, map) in &branch_vars {
                set.extend(map.keys().cloned());
            }
            set
        };
        let mut merged_vars = base_vars.clone();
        let mut phi_entries: Vec<(ValueId, Vec<(BasicBlockId, ValueId)>)> = Vec::new();
        for name in names {
            let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
            for (bbid, map) in &branch_vars {
                if let Some(&val) = map.get(&name) {
                    inputs.push((*bbid, val));
                }
            }
            if inputs.is_empty() {
                if let Some(&base_val) = base_vars.get(&name) {
                    merged_vars.insert(name.clone(), base_val);
                }
                continue;
            }
            let unique: HashSet<ValueId> = inputs.iter().map(|(_, v)| *v).collect();
            if unique.len() == 1 {
                merged_vars.insert(name.clone(), inputs[0].1);
                continue;
            }
            let dst = f.next_value_id();
            inputs.sort_by_key(|(bbid, _)| bbid.0);
            phi_entries.push((dst, inputs));
            merged_vars.insert(name.clone(), dst);
        }
        // フェーズM.2: PHI統一処理（no_phi分岐削除）
        if let Some(_bb) = f.get_block_mut(exit_bb) {
            for (dst, inputs) in phi_entries {
                crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                    f,
                    exit_bb,
                    dst,
                    inputs,
                    Span::unknown(),
                )?;
            }
        }
        *vars = merged_vars;
        Ok(exit_bb)
    }
}
