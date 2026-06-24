# 🎨 UI Contribution Guide

## 🏗️ Clean Architecture - "The Structural Handshake"

The UI has been refactored from a single 607-line monolithic file into a **clean, contributor-friendly architecture**. When developers clone the repo, they see organized structure instead of overwhelming complexity.

## 📁 New UI Structure

```
src/ui/
├── mod.rs           ← Module coordinator & public API
├── render.rs        ← Main rendering orchestrator (entry point)
├── layout.rs        ← Core layout components (navbar, sidebar, workspace)
├── modals.rs        ← All modal dialogs & overlays
└── components.rs    ← Reusable widgets & utilities
```

### **Why This Architecture Rocks:**
- **📦 Single Responsibility** - Each file has one clear purpose
- **🎯 Easy Entry Points** - Contributors know exactly where to add features
- **🔄 No Massive Files** - Largest file is ~150 lines (was 607!)
- **🧩 Reusable Components** - Common UI patterns extracted into utilities
- **🚀 Fast Navigation** - Find what you need in seconds, not minutes

## 🎯 Contributor Workflow

### **Adding a New Modal**
```rust
// 1. Add your modal function in src/ui/modals.rs
pub fn draw_my_new_modal(f: &mut Frame, app: &App) {
    let area = centered_modal_area(f.area(), 50, 30);
    f.render_widget(Clear, area);
    
    let modal = create_option_modal(
        " My New Feature ",
        &["Option 1", "Option 2", "Option 3"]
    );
    f.render_widget(modal, area);
}

// 2. Add the mode to src/ui/render.rs
match app.input_mode {
    // ... existing modes
    InputMode::MyNewMode => {
        draw_my_new_modal(f, app);
    }
    _ => {}
}
```

### **Adding a Layout Component**
```rust
// Add to src/ui/layout.rs
pub fn draw_my_panel(f: &mut Frame, app: &App, area: Rect) {
    let content = Paragraph::new("My new panel content")
        .block(Block::default().borders(Borders::ALL).title(" My Panel "));
    f.render_widget(content, area);
}

// Use in draw_workspace() or create new layout sections
```

### **Creating Reusable Widgets**
```rust
// Add to src/ui/components.rs
pub fn create_status_indicator(status: &str, is_active: bool) -> Span {
    Span::styled(
        format!("● {}", status),
        if is_active {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        }
    )
}
```

## 🛠️ Available Utilities

### **Modal Helpers (components.rs)**
- `centered_modal_area(area, width%, height%)` - Create centered modal
- `modal_block(title)` - Standard modal styling
- `create_option_modal(title, options)` - Numbered choice dialog
- `create_confirmation_modal(title, message)` - Yes/No dialog
- `create_info_modal(title, message)` - Information display
- `create_input_field(label, value, focused)` - Text input with cursor

### **Layout Functions (layout.rs)**
- `draw_navbar(f, area)` - Top navigation bar
- `draw_sidebar(f, app, area)` - Left command list
- `draw_details_panel(f, app, area)` - Right details view
- `draw_search_results(f, app, area)` - Search results panel
- `draw_footer(f, app, area)` - Status footer

### **Modal Collection (modals.rs)**
- `draw_add_modal()` - Command creation form
- `draw_confirm_delete()` - Delete confirmation
- `draw_placeholder_modal()` - Variable filling
- `draw_copy_choice()` - Copy options
- `draw_sort_picker()` - Sort selection
- `draw_expanded_view()` - Detailed command view

## 🎨 Design Patterns

### **Consistent Styling**
```rust
// Use predefined colors and styles
Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)  // Headers
Style::default().fg(Color::Yellow)                              // Highlights  
Style::default().fg(Color::Green)                              // Success
Style::default().fg(Color::Red)                                // Danger
Style::default().fg(Color::Gray)                               // Muted
```

### **Modal Sizing Guidelines**
```rust
// Small modals (confirmations)
centered_modal_area(f.area(), 40, 25)

// Medium modals (forms, choices)  
centered_modal_area(f.area(), 50, 35)

// Large modals (expanded views)
centered_modal_area(f.area(), 80, 60)
```

### **Keyboard Shortcut Display**
```rust
Line::from(vec![
    Span::styled("[key]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    Span::raw(" Action description"),
])
```

## 🔥 Common Contribution Scenarios

### **Scenario 1: Adding a New Modal**
**Goal**: Add a "Settings" modal

1. **Add to app.rs**: `InputMode::Settings`
2. **Add to modals.rs**: `draw_settings_modal()`
3. **Add to render.rs**: Match case for `InputMode::Settings`
4. **Add to handler.rs**: Key handler to open modal

### **Scenario 2: Enhancing the Sidebar**
**Goal**: Add icons to command list

1. **Modify layout.rs**: `draw_sidebar()` function
2. **Update ListItem creation** to include icons
3. **Test with different terminal widths**

### **Scenario 3: New Reusable Component**
**Goal**: Create a progress bar widget

1. **Add to components.rs**: `create_progress_bar()` function
2. **Use in modals or layout** where progress is shown
3. **Add styling options** for different contexts

### **Scenario 4: Layout Restructuring**
**Goal**: Add a new panel to workspace

1. **Modify layout.rs**: `draw_workspace()` constraints
2. **Create new panel function**
3. **Update area splitting logic**

## 🧪 Testing Your Changes

### **Quick UI Testing**
```bash
# Test TUI works
cargo run

# Test specific modals by navigating to them:
cargo run  # Launch TUI
/          # Enter search (test search modal)
a          # Add command (test form modal)  
s          # Sort picker (test options modal)
Enter      # Expanded view (test detail modal)
```

### **Visual Regression Testing**
- Test in different terminal sizes
- Verify modals center properly
- Check text truncation works
- Ensure keyboard shortcuts display correctly

## 📋 Code Quality Standards

### **File Organization Rules**
- **render.rs**: Orchestration only, no direct widget creation
- **layout.rs**: Main UI structure, no modal logic
- **modals.rs**: Modal dialogs only, use components.rs utilities  
- **components.rs**: Reusable widgets, no app-specific logic

### **Function Naming Convention**
- `draw_*()` - Functions that render to Frame
- `create_*()` - Functions that return widgets
- `*_area()` - Functions that calculate layout areas

### **Documentation Requirements**
- Every public function needs a doc comment
- Complex layout logic needs inline comments
- New patterns should be documented in this guide

## 🚀 Before Submitting PR

### **Checklist**
- [ ] Code follows file organization rules
- [ ] New functions have documentation
- [ ] UI works in different terminal sizes
- [ ] No clippy warnings for UI code
- [ ] Keyboard shortcuts are documented
- [ ] Modal sizing follows guidelines

### **Testing Commands**
```bash
# Quality checks
cargo clippy --fix           # Auto-fix style issues
cargo fmt                    # Format code
./scripts/dev-check.sh       # Full quality check

# UI-specific testing
cargo run                    # Test TUI works
cargo run -- -l            # Test CLI still works
```

## 🎯 Future Enhancement Ideas

### **Easy Wins (Good First Issues)**
- Add more color themes
- Improve modal animations (fade in/out)
- Add keyboard shortcut hints in more places
- Better error message displays

### **Medium Complexity**
- Tabbed interface for different views
- Configurable keyboard shortcuts
- Search highlighting in results
- Undo/redo functionality UI

### **Advanced Features**
- Plugin system for custom UI components
- Theme system with hot-reloading
- Vim-style command mode
- Split-pane layouts

## 💡 Pro Tips

### **Debugging UI Issues**
```rust
// Add temporary borders to debug layout
.block(Block::default().borders(Borders::ALL).title(" DEBUG "))

// Use different colors to identify sections
.style(Style::default().bg(Color::Red))  // Temporary debugging
```

### **Performance Considerations**
- Avoid creating widgets in hot paths
- Use `&str` instead of `String` where possible
- Cache complex calculations outside render loop
- Profile with `cargo build --release` for real performance

### **Accessibility Guidelines**
- Use sufficient color contrast
- Provide keyboard alternatives for all actions
- Include helpful status messages
- Support different terminal capabilities

---

**The result**: Contributors can now easily find and modify UI components without getting lost in a massive file. The architecture supports rapid development while maintaining code quality! 🎉