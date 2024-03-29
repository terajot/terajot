use std::{cmp::max, cmp::min, collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    db_reader::DbReader,
    models::stack::Stack,
};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Mode {
    #[default]
    BrowseStacks,
    BrowseEntries,
    Command,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    stacks: Vec<Stack>,
    stack_state: ListState,
    mode: Mode,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_stacks(&mut self) {
        self.stacks = Stack::get_all();
        self.stack_state = ListState::default();
    }

    fn draw_stack_ui(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let list = List::new(
            self.stacks
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<String>>(),
        )
        .block(
            Block::default()
                .title("Stacks")
                .borders(Borders::ALL)
                .border_style(style::Style::default().fg(match self.mode {
                    Mode::BrowseStacks => Color::Yellow,
                    _ => Color::White,
                })),
        )
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, area, &mut self.stack_state);
    }

    fn draw_entry_ui(&mut self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Block::default().borders(Borders::ALL).title("Entries"),
            area,
        );
    }

    fn draw_status_ui(&mut self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("tooltips and status will go here"),
            area,
        );
    }

    fn handle_keys_for_stack_mode(&mut self, key: KeyEvent) {
        let mut selected_index = self.stack_state.selected().unwrap_or(0);
        match key.code {
            KeyCode::Down => {
                selected_index = min(selected_index + 1, self.stacks.len() - 1);
            }
            KeyCode::Up => {
                if selected_index > 0 {
                    selected_index -= 1;
                }
            }
            KeyCode::Enter => {
                if let Some(selected) = self.stack_state.selected() {
                    self.mode = Mode::BrowseEntries
                }
            }
            _ => {}
        }

        self.stack_state.select(Some(selected_index));
    }
    fn handle_keys_for_entry_mode(&mut self, key: KeyEvent) {}
}

impl Component for Home {
    fn init(&mut self, area: Rect) -> Result<()> {
        self.load_stacks();
        self.stack_state.select(Some(2));

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match self.mode {
            Mode::BrowseStacks => self.handle_keys_for_stack_mode(key),
            Mode::BrowseEntries => self.handle_keys_for_entry_mode(key),
            _ => {}
        };

        Ok(None)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Resume => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(4)],
        )
        .split(frame.size());

        self.draw_status_ui(frame, main_layout[1]);

        let inner_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(25), Constraint::Percentage(75)],
        )
        .split(main_layout[0]);

        self.draw_stack_ui(frame, inner_layout[0]);

        self.draw_entry_ui(frame, inner_layout[1]);

        Ok(())
    }
}
