# TUI Messages Specification

## MODIFIED Requirements

### Requirement: Message Display and Auto-Dismiss
The TUI SHALL display temporary messages to inform users of operation results. Messages MUST automatically dismiss after a timeout period without requiring user interaction. Messages MUST support text wrapping to prevent overflow.

#### Scenario: Long message wraps within message box
**Given** a message is displayed that exceeds the width of the message box
**When** the message is rendered
**Then** the text SHALL wrap to multiple lines within the message box borders
**And** the message box SHALL maintain its size and position

#### Scenario: Message auto-dismisses after timeout
**Given** a message is displayed to the user
**When** approximately 3 seconds have elapsed
**Then** the message SHALL automatically disappear
**And** the help text SHALL be restored in the footer area

#### Scenario: User can continue操作 immediately after message
**Given** a message is displayed
**When** the user presses navigation keys
**Then** the application SHALL respond normally
**And** the message SHALL be replaced by the appropriate UI state

#### Scenario: Short message displays correctly
**Given** a short message like "📤 数据导出完成！" is displayed
**When** the message is rendered
**Then** the message SHALL be centered in the message box
**And** the message SHALL auto-dismiss after the timeout period

#### Scenario: Message updates replace previous message
**Given** a message is currently displayed
**When** a new message is triggered (e.g., another action completes)
**Then** the new message SHALL replace the previous message
**And** the timeout SHALL reset for the new message
