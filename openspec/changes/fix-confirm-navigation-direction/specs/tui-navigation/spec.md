# TUI Navigation Specification

## MODIFIED Requirements

### Requirement: Confirm Dialog Navigation
The TUI confirm dialog SHALL support left/right arrow key navigation that matches the visual layout of options. When options are displayed horizontally, the left arrow key MUST select the leftmost option and the right arrow key MUST select the rightmost option.

#### Scenario: User navigates to "Yes" option using Left key
**Given** the user is in the "Update Progress Confirm" dialog
**When** the user presses the Left arrow key
**Then** the visually left option ("Yes") should be selected

#### Scenario: User navigates to "No" option using Right key
**Given** the user is in the "Update Progress Confirm" dialog
**When** the user presses the Right arrow key
**Then** the visually right option ("No") should be selected

#### Scenario: User confirms selection with Enter
**Given** the user has selected "Yes" in the confirm dialog
**When** the user presses Enter
**Then** the progress should be marked as complete

#### Scenario: User cancels selection with Enter
**Given** the user has selected "No" in the confirm dialog
**When** the user presses Enter
**Then** the progress should NOT be marked as complete and dialog closes

#### Scenario: Tab key continues to select "Yes"
**Given** the user is in the "Update Progress Confirm" dialog
**When** the user presses Tab or Space
**Then** "Yes" should be selected (existing behavior preserved)

#### Scenario: Esc key cancels dialog
**Given** the user is in the "Update Progress Confirm" dialog
**When** the user presses Esc
**Then** the dialog should close without saving changes
