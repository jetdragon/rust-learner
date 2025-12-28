# TUI 练习界面修复规格

## ADDED Requirements

### Requirement: TUI Practice Session State
The TUI SHALL implement a new PracticeSession state to manage the practice testing experience within the TUI interface.

#### Scenario: User starts practice session
- **Given**: User is in the practice configuration screen
- **When**: User presses Enter to start practice
- **Then**: Application transitions to PracticeSession state
- **And**: Practice questions are loaded and displayed

### Requirement: Interactive Question Display
The TUI SHALL display practice questions with clear formatting and support for multiple choice and true/false questions.

#### Scenario: Display multiple choice question
- **Given**: A multiple choice question is active
- **When**: The question is rendered
- **Then**: Question text is displayed at the top
- **And**: Options are listed with numbers (1, 2, 3, 4)
- **And**: Current selection is highlighted

#### Scenario: Display true/false question
- **Given**: A true/false question is active
- **When**: The question is rendered
- **Then**: Question text is displayed
- **And**: Two options are shown (1. 正确, 2. 错误)
- **And**: Selection is clearly indicated

### Requirement: Answer Input Handling
The TUI SHALL accept numeric input (1-9) to select answers and provide visual feedback.

#### Scenario: User selects answer
- **Given**: A question with options is displayed
- **When**: User presses a number key (1-9)
- **Then**: The corresponding option is selected
- **And**: Visual feedback shows the selection
- **And**: Selection is stored for scoring

#### Scenario: User changes answer
- **Given**: An answer is already selected
- **When**: User presses a different number key
- **Then**: The new selection replaces the previous one
- **And**: Visual feedback updates immediately

### Requirement: Question Navigation
The TUI SHALL support navigation between questions using arrow keys or other designated keys.

#### Scenario: Navigate to next question
- **Given**: User is on a question (not the last one)
- **When**: User presses Right arrow or specific next key
- **Then**: Application moves to the next question
- **And**: Current question's answer is preserved

#### Scenario: Navigate to previous question
- **Given**: User is on a question (not the first one)
- **When**: User presses Left arrow or specific previous key
- **Then**: Application moves to the previous question
- **And**: Any previously selected answer is displayed

## MODIFIED Requirements

### Requirement: Practice Interface State Management
The TUI SHALL modify the existing Practice state to support launching an interactive practice session instead of switching to CLI.

#### Current Implementation
Currently, pressing Enter in the practice configuration screen calls `crate::exercise::run_practice()` which switches to CLI mode.

#### Scenario: Start interactive practice
- **Given**: User presses Enter in practice configuration
- **When**: Practice questions are available
- **Then**: TUI transitions to PracticeSession state
- **And**: First question is displayed immediately
- **And**: No CLI mode switch occurs

### Requirement: Terminal State Preservation
The TUI SHALL ensure terminal state is preserved throughout the practice session.

#### Current Implementation
Terminal raw mode is managed by the TUI event loop.

#### Scenario: Maintain terminal state during practice
- **Given**: TUI is in raw mode
- **When**: Practice session starts
- **Then**: Terminal remains in raw mode
- **And**: Event polling continues normally
- **And**: No terminal reconfiguration occurs

## REMOVED Requirements

### CLI Practice Mode
- **Removed**: Direct call to `crate::exercise::run_practice()`
- **Removed**: Switching between TUI and CLI modes
- **Removed**: Standard input/output blocking reads

## Technical Specifications

### Implementation Details

1. **Practice Session State**
   - Add `PracticeSession { questions: Vec<Question>, current_index: usize, answers: Vec<Option<usize>> }` to AppState
   - Implement state transition from Practice to PracticeSession
   - Handle graceful return to previous state

2. **Question Display Format**
   - Question number and topic at the top
   - Question text in main area
   - Numbered options below
   - Progress indicator (e.g., "Question 3 of 10")
   - Navigation hints at bottom

3. **Input Handling**
   - Number keys 1-9 for answer selection
   - Left/Right arrows for navigation
   - Enter to confirm and move next
   - Esc to exit practice session
   - Store answers in state vector

4. **Visual Feedback**
   - Highlight selected option
   - Show answer status (answered/unanswered)
   - Display navigation controls
   - Progress bar or counter

### Testing Requirements

1. **Functionality Tests**
   - Verify question display formats correctly
   - Test answer selection and storage
   - Validate navigation between questions
   - Confirm score calculation accuracy

2. **Integration Tests**
   - Test state transitions
   - Verify terminal state consistency
   - Check return to previous state
   - Validate data persistence

3. **User Experience Tests**
   - Ensure responsive input handling
   - Verify visual feedback clarity
   - Test error handling gracefully
   - Confirm intuitive navigation

### Acceptance Criteria

- Practice session runs entirely within TUI
- Questions display with proper formatting
- Answer input is responsive and accurate
- Navigation works smoothly between questions
- Terminal state remains consistent
- No window freezing or unresponsiveness
- Results are properly calculated and stored