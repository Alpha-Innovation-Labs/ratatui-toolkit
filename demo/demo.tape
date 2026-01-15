# ratatui-toolkit Demo Tape
# Usage: just demo-record
# This tape demonstrates all the key features of ratatui-toolkit

# Set up the terminal environment
Set FontSize 14
Set Shell "bash"
Set TypingSpeed 40ms

# Run the showcase demo using just
Type "just run"
Enter

# Wait for the demo to start and show Layout tab
Sleep 2s

# =============================================================================
# Tab 1: Layout - Show split pane resizing
# =============================================================================

# Resize with keyboard arrows (stay on Layout longer)
Type "Right"
Sleep 150ms
Type "Right"
Sleep 150ms
Type "Right"
Sleep 150ms
Type "Left"
Sleep 150ms
Type "Left"
Sleep 150ms
Type "Left"
Sleep 300ms

# =============================================================================
# Tab 2: Tree View - Navigate tree structure
# =============================================================================
Type "Tab"
Sleep 600ms

# Navigate tree with keyboard
Type "j"
Sleep 150ms
Type "j"
Sleep 150ms
Type "j"
Sleep 150ms
Type "l"
Sleep 150ms
Type "k"
Sleep 150ms
Type "h"
Sleep 300ms

# =============================================================================
# Tab 3: Dialogs - Show different dialog types
# =============================================================================
Type "Tab"
Sleep 600ms

# Show info dialog
Type "i"
Sleep 600ms
Type "Escape"
Sleep 300ms

# Show warning dialog
Type "w"
Sleep 600ms
Type "Escape"
Sleep 300ms

# Show error dialog
Type "e"
Sleep 600ms
Type "Escape"
Sleep 300ms

# Show confirm dialog
Type "c"
Sleep 600ms
Type "Escape"
Sleep 300ms

# =============================================================================
# Tab 4: Markdown - Show markdown rendering
# =============================================================================
Type "Tab"
Sleep 600ms

# Scroll markdown content
Type "j"
Sleep 150ms
Type "j"
Sleep 150ms
Type "k"
Sleep 150ms

# =============================================================================
# Tab 5: Scrollbar - Show interactive scrollbar
# =============================================================================
Type "Tab"
Sleep 600ms

# Scroll with keyboard
Type "Down"
Sleep 200ms
Type "Down"
Sleep 200ms
Type "Down"
Sleep 200ms
Type "Up"
Sleep 200ms
Type "Up"
Sleep 200ms
Type "End"
Sleep 200ms
Type "Home"
Sleep 300ms

# =============================================================================
# Tab 6: StatusLine - Show vim mode switching
# =============================================================================
Type "Tab"
Sleep 600ms

# Switch vim modes to show status changes
Type "i"
Sleep 300ms
Type "Escape"
Sleep 300ms
Type "v"
Sleep 300ms
Type "Escape"
Sleep 300ms
Type ":"
Sleep 300ms
Type "Escape"
Sleep 300ms

# =============================================================================
# Tab 7: Terminal - Show embedded terminal with commands
# =============================================================================
Type "Tab"
Sleep 600ms

# Type some commands in the embedded terminal
Type "echo 'ratatui-toolkit demo'"
Enter
Sleep 400ms

# Clear screen
Type "Ctrl+l"
Sleep 200ms

# Exit demo
Type "q"
Sleep 300ms
