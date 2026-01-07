# Tasks: Fix Message Display Issues in TUI

## Implementation Tasks

- [x] 1. Add `message_deadline: Option<Instant>` field to `App` struct
- [x] 2. Update `draw_message` function to support text wrapping:
  - Change `Line::from()` to `Text::from()` or use `Vec<Line>`
  - Add `.wrap(Wrap { trim: true })` to Paragraph
- [x] 3. Create `show_message(&mut self, msg: String)` helper method:
  - Sets `self.message = Some(msg)`
  - Sets `self.message_deadline = Some(Instant::now() + Duration::from_secs(3))`
- [x] 4. Replace all `self.message = Some(...)` assignments with `self.show_message(...)`
- [x] 5. Add timeout check in main loop (`run_tui` function):
  - Check if `message_deadline` is past
  - Clear message if timeout expired
- [x] 6. (Optional) Shorten message format to prevent overflow:
  - Use module index instead of full name
  - Format: "✅ 已更新模块 01 的任务 1"
  - Note: Kept full name but text wrapping now handles it
- [x] 7. Test with long messages:
  - Update progress with full module name
  - Verify text wraps correctly
  - Verify message auto-dismisses after 3 seconds
- [x] 8. Test with short messages:
  - Export data completion message
  - Reminder set message
  - Verify display and dismiss work correctly

## Validation Criteria
- [x] Long messages wrap within the message box borders
- [x] Messages auto-dismiss after approximately 3 seconds
- [x] User can continue操作 immediately after message appears
- [x] No manual key press required to dismiss messages
- [x] All existing messages still display correctly
