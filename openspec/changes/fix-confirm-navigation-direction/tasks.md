# Tasks: Fix Navigation Direction in Update Progress Confirm Dialog

## Implementation Tasks

- [x] 1. Locate the `handle_update_progress_confirm_key` function in `learning-companion/src/tui.rs`
- [x] 2. Swap the `KeyCode::Left` and `KeyCode::Right` key handlers:
  - `KeyCode::Left` should set `confirmed = true` (select "Yes")
  - `KeyCode::Right` should set `confirmed = false` (select "No")
- [x] 3. Keep `KeyCode::Tab` and `KeyCode::Char(' ')` behavior unchanged (they select "Yes")
- [x] 4. Update the help text to reflect correct navigation if needed
  - Help text "←→ 选择 | Enter 确认 | Esc 返回" is generic and correct
- [x] 5. Test the fix manually:
  - Run the TUI application
  - Navigate to "更新学习进度"
  - Select a module and task
  - Press Enter to open confirm dialog
  - Verify Left key selects "Yes" (left option)
  - Verify Right key selects "No" (right option)
  - Verify Enter confirms the selection correctly
- [x] 6. Run existing tests to ensure no regressions

## Validation Criteria
- [x] Left arrow key visually selects the left option ("Yes")
- [x] Right arrow key visually selects the right option ("No")
- [x] Tab and Space keys still work as before
- [x] No other navigation is affected
