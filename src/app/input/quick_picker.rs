use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::state::AppState;

use super::modal::leave_modal;

pub(crate) fn handle_quick_picker_key(
    state: &mut AppState,
    terminal_runtimes: &crate::terminal::TerminalRuntimeRegistry,
    key: KeyEvent,
) {
    match key.code {
        KeyCode::Esc => leave_modal(state),
        KeyCode::Enter => {
            state.accept_quick_picker_selection_from(terminal_runtimes);
        }
        KeyCode::Up => state.move_quick_picker_selection_from(terminal_runtimes, -1),
        KeyCode::Down => state.move_quick_picker_selection_from(terminal_runtimes, 1),
        KeyCode::Char('n') if key.modifiers == KeyModifiers::CONTROL => {
            state.move_quick_picker_selection_from(terminal_runtimes, 1)
        }
        KeyCode::Char('p') if key.modifiers == KeyModifiers::CONTROL => {
            state.move_quick_picker_selection_from(terminal_runtimes, -1)
        }
        KeyCode::Backspace => {
            state.quick_picker.query.pop();
            state.clamp_quick_picker_selection_from(terminal_runtimes);
        }
        KeyCode::Char('u') if key.modifiers == KeyModifiers::CONTROL => {
            state.quick_picker.query.clear();
            state.clamp_quick_picker_selection_from(terminal_runtimes);
        }
        KeyCode::Char(c) if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT => {
            state.quick_picker.query.push(c);
            state.clamp_quick_picker_selection_from(terminal_runtimes);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app::state::Mode, terminal::TerminalRuntimeRegistry, workspace::Workspace};
    use ratatui::layout::Direction;

    #[test]
    fn ctrl_n_and_ctrl_p_move_picker_selection() {
        let mut state = AppState::test_new();
        state.workspaces = vec![Workspace::test_new("one"), Workspace::test_new("two")];
        state.ensure_test_terminals();
        state.open_quick_picker();

        let runtimes = TerminalRuntimeRegistry::new();
        handle_quick_picker_key(
            &mut state,
            &runtimes,
            KeyEvent::new(KeyCode::Char('n'), KeyModifiers::CONTROL),
        );
        assert_eq!(state.quick_picker.selected, 1);

        handle_quick_picker_key(
            &mut state,
            &runtimes,
            KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL),
        );
        assert_eq!(state.quick_picker.selected, 0);
    }

    #[test]
    fn enter_accepts_selected_quick_picker_pane() {
        let mut state = AppState::test_new();
        let mut workspace = Workspace::test_new("one");
        let target = workspace.test_split(Direction::Horizontal);
        state.workspaces = vec![workspace];
        state.ensure_test_terminals();
        state.active = Some(0);
        state.selected = 0;

        let terminal_id = state.workspaces[0].terminal_id(target).cloned().unwrap();
        state
            .terminals
            .get_mut(&terminal_id)
            .unwrap()
            .set_agent_name("planner".into());
        state.open_quick_picker();
        state.quick_picker.query = "planner".into();

        let runtimes = TerminalRuntimeRegistry::new();
        handle_quick_picker_key(
            &mut state,
            &runtimes,
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
        );

        assert_eq!(state.workspaces[0].focused_pane_id(), Some(target));
        assert_eq!(state.mode, Mode::Terminal);
    }
}
