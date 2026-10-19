# Restart Quick Resume

Status: reference pointer
Scope: optional restart entry; operations belong to RULES.

Read [RULES](../../RULES.md),
`docs/development/current/main/CURRENT_STATE.toml`, then `latest_card_path`.
`active_lane` and the selected app determine work; `current_blocker_token`
names its first stop. Workstream/north-star/policy are references as needed.

Check `git status -sb` and `bash tools/checks/current_state_pointer_guard.sh`.
Use the existing manifest's exact smoke path and explicit binary/runtime.
Keep one app selected; internal commits do not create rows/cards.

When `work_mode = "design_stop"`, pause implementation and resolve the named design dependency.
Goal status follows the session contract. User pause remains until resume permission.
Full finite goal and parked-lane reopen conditions remain unchanged.
