# CLI到TUI界面转换规格

## ADDED Requirements

### Requirement: TUI Dashboard State
The TUI SHALL implement a dedicated dashboard state that displays detailed learning progress information within the TUI interface.

#### Scenario: User navigates to detailed dashboard
- **Given**: User is in the main TUI interface
- **When**: User selects dashboard view option
- **Then**: Application transitions to dashboard state
- **And**: Displays comprehensive learning statistics
- **And**: Shows progress charts and recommendations

### Requirement: TUI Achievement Viewer
The TUI SHALL provide an interactive achievement viewer state to replace the CLI-based achievement display.

#### Scenario: User views achievements in TUI
- **Given**: User is in the TUI main menu
- **When**: User selects achievements option
- **Then**: Application shows achievement list in TUI format
- **And**: Displays locked/unlocked status clearly
- **And**: Shows achievement details when selected

### Requirement: Integrated File Viewer
The TUI SHALL include a built-in file viewer to display file contents without switching to external applications.

#### Scenario: User opens a file from task list
- **Given**: User is viewing module details
- **When**: User selects a file to open
- **Then**: File content is displayed within TUI
- **And**: Supports markdown rendering
- **And**: Provides navigation controls

#### Scenario: File viewer displays code files
- **Given**: User opens a Rust source file
- **When**: File viewer loads the content
- **Then**: Code is displayed with basic formatting
- **And**: Line numbers are shown
- **And**: Syntax highlighting is applied if available

### Requirement: Contextual Encouragement Display
The TUI SHALL display encouragement messages within the interface rather than switching to CLI output.

#### Scenario: User completes a task
- **Given**: User marks a task as complete
- **When**: Task update is successful
- **Then**: Encouragement message appears in TUI
- **And**: Message is displayed prominently
- **And**: Message disappears after a timeout

## MODIFIED Requirements

### Requirement: Replace External Editor Dependency
The TUI SHALL replace the external VSCode dependency with an integrated file viewer while maintaining the option to use external editors.

#### Current Implementation
Currently uses `std::process::Command` to spawn VSCode process.

#### Scenario: Open file without external editor
- **Given**: User selects a file to open
- **When**: Integrated viewer is available
- **Then**: File opens in TUI viewer
- **And**: No external process is spawned
- **And**: Terminal state remains unchanged

#### Scenario: External editor option available
- **Given**: User has configured external editor preference
- **When**: User opens a file
- **Then**: System provides option to use external editor
- **And**: Falls back to integrated viewer if external fails

### Requirement: Unified Navigation Experience
The TUI SHALL ensure all features are accessible through consistent navigation patterns without mode switching.

#### Current Implementation
Some features (dashboard, achievements) only available via CLI commands.

#### Scenario: Access all features from TUI
- **Given**: User is in TUI mode
- **When**: User navigates through menus
- **Then**: All features are accessible
- **And**: No CLI mode switch is required
- **And**: Navigation remains consistent

## REMOVED Requirements

### CLI Mode Dependencies
- **Removed**: Direct calls to `std::process::Command` for UI operations
- **Removed**: Blocking print statements for user interaction
- **Removed**: Terminal mode switching between TUI and CLI

### External Application Dependencies
- **Removed**: Mandatory VSCode dependency for file viewing
- **Removed**: System-specific external program calls

## Technical Specifications

### Implementation Details

1. **Dashboard State Implementation**
   - Create `AppState::DashboardDetailed` for comprehensive view
   - Implement progress visualization with Unicode characters
   - Add interactive charts and graphs
   - Include learning recommendations

2. **Achievement Viewer Implementation**
   - Create `AppState::AchievementsDetailed` for TUI view
   - Implement achievement grid/list view
   - Add achievement detail panel
   - Include unlock progress visualization

3. **File Viewer Implementation**
   - Create `AppState::FileViewer { path: PathBuf, content: String }`
   - Support common file formats (MD, TXT, RS, JSON, etc.)
   - Implement basic markdown rendering
   - Add line numbers and navigation controls
   - Include "Open in External Editor" option

4. **Encouragement Integration**
   - Integrate messages into existing TUI states
   - Use toast/notification style display
   - Implement timed auto-dismissal
   - Add animation effects

### File Viewer Features

1. **Supported Formats**
   - Plain text files (.txt, .md, .rs, .toml, .json, .yaml)
   - Markdown with basic formatting
   - Code files with syntax highlighting plans

2. **Navigation Controls**
   - Arrow keys for scrolling
   - Page Up/Down for fast navigation
   - Home/End for file start/end
   - Esc to exit viewer

3. **Display Options**
   - Line numbers toggle
   - Word wrap toggle
   - Font size adjustment (if supported)

### Testing Requirements

1. **Functionality Tests**
   - Verify all converted features work in TUI
   - Test file viewer with various formats
   - Validate achievement display accuracy
   - Confirm dashboard data integrity

2. **Integration Tests**
   - Test navigation between all TUI states
   - Verify no CLI mode switches occur
   - Test file viewer with large files
   - Validate terminal state consistency

3. **User Experience Tests**
   - Ensure intuitive navigation
   - Verify responsive controls
   - Test error handling gracefully
   - Confirm visual feedback clarity

### Acceptance Criteria

- All previously CLI-only features work in TUI
- File viewer handles common file types
- No external dependencies required for basic functionality
- Terminal state remains consistent throughout
- User can accomplish all tasks without leaving TUI
- External editor option still available as fallback
- Performance remains acceptable with large files
- All existing TUI functionality preserved

## Migration Strategy

1. **Phase 1**: Convert dashboard and achievements to TUI
2. **Phase 2**: Implement integrated file viewer
3. **Phase 3**: Replace external editor calls
4. **Phase 4**: Integrate encouragement messages
5. **Phase 5**: Remove deprecated CLI functions
6. **Phase 6**: Comprehensive testing and optimization