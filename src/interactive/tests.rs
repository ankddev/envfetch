#[cfg(test)]
use crate::interactive::{InteractiveMode, Mode};
#[cfg(test)]
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
#[cfg(test)]
use ratatui::{Terminal, backend::TestBackend, buffer::Buffer, layout::Rect, widgets::Widget};

#[cfg(test)]
fn create_test_terminal() -> Terminal<TestBackend> {
    let backend = TestBackend::new(100, 30);
    Terminal::new(backend).unwrap()
}

#[cfg(test)]
fn create_key_event(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    }
}

// Initialization and default tests
#[test]
fn test_mode_default() {
    assert!(matches!(Mode::default(), Mode::List));
}

#[test]
fn test_interactive_mode_default() {
    let mode = InteractiveMode::default();
    assert!(!mode.exit);
    assert_eq!(mode.current_index, 0);
    assert_eq!(mode.scroll_offset, 0);
    assert_eq!(mode.value_scroll_offset, 0);
    assert_eq!(mode.visible_options, 30);
    assert_eq!(mode.truncation_len, 30);
    assert!(!mode.entries.is_empty());
}

// Navigation tests
#[test]
fn test_navigation_down() {
    let mut mode = InteractiveMode::default();
    let initial_index = mode.current_index;
    mode.down();
    assert_eq!(mode.current_index, initial_index + 1);
    assert_eq!(mode.value_scroll_offset, 0);
}

#[test]
fn test_navigation_up() {
    let mut mode = InteractiveMode::default();
    mode.down(); // Move down first
    let initial_index = mode.current_index;
    mode.up();
    assert_eq!(mode.current_index, initial_index - 1);
    assert_eq!(mode.value_scroll_offset, 0);
}

#[test]
fn test_navigation_up_at_top() {
    let mut mode = InteractiveMode::default();
    mode.up(); // Try moving up at index 0
    assert_eq!(mode.current_index, 0);
}

#[test]
fn test_navigation_down_at_bottom() {
    let mut mode = InteractiveMode::default();
    let max_index = mode.entries.len() - 1;

    // Move to bottom
    for _ in 0..max_index + 10 {
        mode.down();
    }

    assert_eq!(mode.current_index, max_index);
}

// Scrolling tests
#[test]
fn test_scroll_offset_down() {
    let mode = InteractiveMode {
        visible_options: 5, // Small window to test scrolling
        ..Default::default()
    };

    // Move down enough to trigger scroll
    let mut mode = mode;
    for _ in 0..10 {
        mode.down();
    }

    assert!(mode.scroll_offset > 0);
}

#[test]
fn test_scroll_offset_up() {
    let mode = InteractiveMode {
        visible_options: 5,
        ..Default::default()
    };

    let mut mode = mode;
    // Move down then up to test scroll
    for _ in 0..10 {
        mode.down();
    }
    let scroll_offset = mode.scroll_offset;

    for _ in 0..5 {
        mode.up();
    }

    assert!(mode.scroll_offset < scroll_offset);
}

// Value scrolling tests
#[test]
fn test_value_scroll() {
    let mut mode = InteractiveMode::default();
    mode.entries.push(("TEST".to_string(), "a".repeat(100)));
    mode.current_index = mode.entries.len() - 1;

    mode.scroll_value_right();
    assert!(mode.value_scroll_offset > 0);

    mode.scroll_value_left();
    assert_eq!(mode.value_scroll_offset, 0);
}

#[test]
fn test_value_scroll_boundaries() {
    let mut mode = InteractiveMode::default();
    mode.entries.push(("TEST".to_string(), "short".to_string()));
    mode.current_index = mode.entries.len() - 1;

    // Test right boundary
    for _ in 0..10 {
        mode.scroll_value_right();
    }
    assert!(mode.value_scroll_offset <= 5); // "short".len()

    // Test left boundary
    for _ in 0..10 {
        mode.scroll_value_left();
    }
    assert_eq!(mode.value_scroll_offset, 0);
}

// Key event handling tests
#[test]
fn test_key_events() {
    let mut mode = InteractiveMode::default();

    // Test all key combinations
    let test_keys = vec![
        (KeyCode::Down, KeyModifiers::NONE),
        (KeyCode::Up, KeyModifiers::NONE),
        (KeyCode::Left, KeyModifiers::NONE),
        (KeyCode::Right, KeyModifiers::NONE),
        (KeyCode::Char('q'), KeyModifiers::CONTROL),
        (KeyCode::Char('r'), KeyModifiers::CONTROL),
        (KeyCode::Char('x'), KeyModifiers::NONE), // Invalid key
    ];

    for (code, modifiers) in test_keys {
        mode.handle_key_event(create_key_event(code, modifiers));
    }
}

// Reload functionality tests
#[test]
fn test_reload() {
    let mode = InteractiveMode {
        current_index: 1,
        scroll_offset: 1,
        value_scroll_offset: 1,
        ..Default::default()
    };

    let mut mode = mode;
    mode.reload();

    assert_eq!(mode.current_index, 0);
    assert_eq!(mode.scroll_offset, 0);
    assert_eq!(mode.value_scroll_offset, 0);
    assert!(!mode.entries.is_empty());
}

// UI rendering tests
#[test]
fn test_render_basic() {
    let mut mode = InteractiveMode::default();
    let area = Rect::new(0, 0, 100, 30);
    let mut buffer = Buffer::empty(area);

    Widget::render(&mut mode, area, &mut buffer);

    assert!(buffer.content.iter().any(|cell| !cell.symbol().is_empty()));
}

#[test]
fn test_render_with_scroll() {
    let mode = InteractiveMode {
        scroll_offset: 5,
        ..Default::default()
    };

    let mut mode = mode;
    let area = Rect::new(0, 0, 100, 30);
    let mut buffer = Buffer::empty(area);

    Widget::render(&mut mode, area, &mut buffer);

    // Verify scroll indicators
    let content = buffer
        .content
        .iter()
        .map(|cell| cell.symbol().to_string())
        .collect::<String>();
    assert!(content.contains('↓') || content.contains('↑'));
}

#[test]
fn test_render_selected_item() {
    let mode = InteractiveMode {
        current_index: 1,
        ..Default::default()
    };

    let mut mode = mode;
    let area = Rect::new(0, 0, 100, 30);
    let mut buffer = Buffer::empty(area);

    Widget::render(&mut mode, area, &mut buffer);

    // Verify selected item indicator
    let content = buffer
        .content
        .iter()
        .map(|cell| cell.symbol().to_string())
        .collect::<String>();
    assert!(content.contains('>'));
}

#[test]
fn test_render_value_panel() {
    let mut mode = InteractiveMode::default();
    let area = Rect::new(0, 0, 100, 30);
    let mut buffer = Buffer::empty(area);

    Widget::render(&mut mode, area, &mut buffer);

    // Verify value panel elements
    let content = buffer
        .content
        .iter()
        .map(|cell| cell.symbol().to_string())
        .collect::<String>();
    assert!(content.contains("Current Value"));
}

// Terminal integration tests
#[test]
fn test_terminal_draw() {
    let mut terminal = create_test_terminal();
    let mut mode = InteractiveMode::default();

    terminal.draw(|f| mode.draw(f)).unwrap();

    let buffer = terminal.backend().buffer();
    assert!(buffer.content.iter().any(|cell| !cell.symbol().is_empty()));
}

// Edge cases
#[test]
fn test_empty_entries() {
    let mut mode = InteractiveMode::default();
    mode.entries.clear();

    mode.down();
    assert_eq!(mode.current_index, 0);

    mode.up();
    assert_eq!(mode.current_index, 0);

    let area = Rect::new(0, 0, 100, 30);
    let mut buffer = Buffer::empty(area);
    Widget::render(&mut mode, area, &mut buffer);
}

#[test]
fn test_small_terminal() {
    let mut mode = InteractiveMode::default();
    let area = Rect::new(0, 0, 10, 5); // Very small terminal
    let mut buffer = Buffer::empty(area);

    Widget::render(&mut mode, area, &mut buffer);
    assert!(buffer.content.iter().any(|cell| !cell.symbol().is_empty()));
}

#[test]
fn test_interactive_mode_init() {
    let mode = InteractiveMode::init();
    assert!(!mode.entries.is_empty());
    assert_eq!(mode.current_index, 0);
    assert_eq!(mode.scroll_offset, 0);
}

#[test]
fn test_interactive_mode_run() {
    let mut mode = InteractiveMode::default();
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    // Simulate quit event
    mode.handle_key_event(create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL));
    assert!(mode.exit);
    assert!(terminal.draw(|f| mode.draw(f)).is_ok());
}

#[test]
fn test_handle_events_quit() {
    let mut mode = InteractiveMode::default();
    let event = create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL);
    mode.handle_key_event(event);
    assert!(mode.exit);
}

#[test]
fn test_handle_events_navigation() {
    let mut mode = InteractiveMode::default();
    let initial_index = mode.current_index;

    // Test down arrow
    mode.handle_key_event(create_key_event(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(mode.current_index, initial_index + 1);

    // Test up arrow
    mode.handle_key_event(create_key_event(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(mode.current_index, initial_index);
}

#[test]
fn test_string_list_filter() {
    let entries = vec![
        ("TEST1".to_string(), "value1".to_string()),
        ("TEST2".to_string(), "value2".to_string()),
        ("OTHER".to_string(), "value3".to_string()),
    ];

    let mode = InteractiveMode {
        entries,
        ..Default::default()
    };

    assert!(!mode.entries.is_empty());
    assert_eq!(mode.entries.len(), 3);
}
