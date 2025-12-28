# TUI 导航修复规格

## ADDED Requirements

### Requirement: Single-Step Navigation with Arrow Keys
The TUI SHALL ensure that arrow key navigation moves selection by exactly one position in all interfaces.
#### Scenario: User presses up arrow key in main menu
- **Given**: User is in main menu with second item selected
- **When**: User presses up arrow key once
- **Then**: First item becomes selected
- **And**: No other item is selected

#### Scenario: User presses down arrow key in dashboard
- **Given**: User is in dashboard with first module selected
- **When**: User presses down arrow key once
- **Then**: Second module becomes selected
- **And**: First module is no longer selected

### Requirement: Single Tab Switch Between Focus Areas
The TUI SHALL ensure Tab key switches focus between UI areas exactly once per key press.
#### Scenario: User presses Tab in module detail view
- **Given**: User is in module detail with TaskList focused
- **When**: User presses Tab key once
- **Then**: Focus switches to Action area
- **And**: TaskList is no longer focused

#### Scenario: User presses Tab in update progress view
- **Given**: User is in update progress with ModuleList focused
- **When**: User presses Tab key once
- **Then**: Focus switches to TaskList
- **And**: ModuleList is no longer focused

### Requirement: Event Debouncing Protection
The TUI SHALL implement protection against rapid key press processing that could cause duplicate navigation.
#### Scenario: Rapid navigation key presses
- **Given**: User rapidly presses navigation keys
- **When**: Keys are processed within 50ms interval
- **Then**: Only the last key press is processed
- **And**: Previous key presses are ignored

### Requirement: State Consistency Validation
The TUI SHALL ensure navigation state updates are applied consistently and immediately without rollback.
#### Scenario: Navigation state update
- **Given**: Navigation changes the selected item
- **When**: The state update is applied
- **Then**: UI immediately reflects the new state
- **And**: State remains unchanged until next input

## MODIFIED Requirements

### Requirement: Event Loop Processing Enhancement
The TUI SHALL add duplicate event prevention to the main event loop to ensure events are processed only once.
#### Current Implementation
The event loop in `run_tui` function processes all events immediately without validation.

#### Scenario: Event with duplicate prevention
- **Given**: A keyboard event is received
- **When**: Event processing flag is already set
- **Then**: The event is ignored
- **And**: No state change occurs

### Requirement: Key Handler Input Validation
The TUI SHALL add input validation to all key handlers to prevent duplicate processing and ensure state consistency.
#### Current Implementation
All key handlers process keys directly without duplicate checking.

#### Scenario: Valid navigation key press
- **Given**: A navigation key is pressed
- **When**: Key handler validates the input
- **Then**: State is updated exactly once
- **And**: No duplicate processing occurs

## REMOVED Requirements

### Legacy Duplicate Event Processing
- **Removed**: Multiple event listeners for same key
- **Removed**: Redundant state update calls
- **Removed**: Deprecated navigation state tracking

## Technical Specifications

### Implementation Details

1. **Event Processing Flags**
   - Add `event_processed` flag to prevent reprocessing
   - Implement per-state navigation tracking
   - Add timing validation for event handling

2. **State Management**
   - Implement atomic state updates
   - Add state validation callbacks
   - Ensure UI synchronization

3. **Debug Support**
   - Add navigation event logging
   - Implement state change tracking
   - Add performance metrics for navigation

### Testing Requirements

1. **Unit Tests**
   - Test each key handler independently
   - Verify state updates are atomic
   - Validate event processing flags

2. **Integration Tests**
   - Test complete navigation flows
   - Verify cross-state navigation
   - Validate UI consistency

3. **Performance Tests**
   - Measure navigation response time
   - Verify no performance degradation
   - Test with rapid input scenarios

### Acceptance Criteria

- All navigation keys respond with single step movement
- Tab switching works reliably across all interfaces
- No duplicate event processing observed
- Performance remains within acceptable limits
- All existing functionality preserved