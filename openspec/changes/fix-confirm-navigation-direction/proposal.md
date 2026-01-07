# Proposal: Fix Navigation Direction in Update Progress Confirm Dialog

## Status
**Proposed** | 2025-01-07

## Context
In the "Update Progress Confirm" dialog (确认更新进度界面), the left/right arrow keys navigate in the opposite direction of the visual layout. Specifically:
- The "Yes (Y)" option is displayed on the **left**
- The "No (N)" option is displayed on the **right**
- But pressing **Left** selects "No" (the right option)
- And pressing **Right** selects "Yes" (the left option)

This is confusing for users as the navigation doesn't match the visual position of the options.

## Current Implementation
The bug is in `learning-companion/src/tui.rs` in the `handle_update_progress_confirm_key` function:

```rust
KeyCode::Left => {
    *confirmed = false;  // Selects "No" (visually on right)
}
KeyCode::Right | KeyCode::Tab | KeyCode::Char(' ') => {
    *confirmed = true;   // Selects "Yes" (visually on left)
}
```

## Proposed Solution
Swap the key mappings so they match the visual layout:
- **Left key** → select "Yes" (left option)
- **Right key** → select "No" (right option)

## Scope
- **File**: `learning-companion/src/tui.rs`
- **Function**: `handle_update_progress_confirm_key`
- **Change**: Swap the logic for `KeyCode::Left` and `KeyCode::Right`

## Alternatives Considered
1. **Swap visual positions**: Keep key mapping but swap "Yes" and "No" positions in UI
   - **Rejected**: Would break user expectations (Yes/No typically Yes-left, No-right in Chinese UI)

2. **Add up/down keys**: Add vertical navigation as alternative
   - **Rejected**: Adds complexity; left/right should work correctly first

## Related Changes
None - this is an isolated bug fix.

## Acceptance Criteria
1. Pressing Left key selects the visually left option ("Yes")
2. Pressing Right key selects the visually right option ("No")
3. Tab key and Space key continue to work (currently they select "Yes")
4. All tests pass
