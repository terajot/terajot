use std::{cmp::max, cmp::min, collections::HashMap, time::Duration};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    db_reader::DbReader,
    models::{entry::Entry, stack::Stack},
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

    entries: Vec<Entry>,
    entry_state: ListState,

    mode: Mode,
}

impl Home {
    const COLOR_INACTIVE: Color = Color::Rgb(100, 100, 100);
    const COLOR_ACTIVE: Color = Color::Rgb(255, 255, 255);

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
                    Mode::BrowseStacks => Self::COLOR_ACTIVE,
                    _ => Self::COLOR_INACTIVE,
                })),
        )
        //.highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("▶️ ");
        //.repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, area, &mut self.stack_state);
    }

    fn draw_entry_ui(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let list = List::new(
            self.entries
                .iter()
                .map(|s| {
                    let content = &s.content;
                    if content.chars().count() > 100 {
                        format!("{}...", &content[..100])
                    } else {
                        content.to_string()
                    }
                })
                .collect::<Vec<String>>(),
        )
        .block(
            Block::default()
                .title("Entries")
                .borders(Borders::ALL)
                .border_style(style::Style::default().fg(match self.mode {
                    Mode::BrowseEntries => Self::COLOR_ACTIVE,
                    _ => Self::COLOR_INACTIVE,
                })),
        )
        //.highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("▶️ ");
        //.repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, area, &mut self.entry_state);
    }

    fn draw_status_ui(&mut self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("tooltips and status will go here"),
            area,
        );
    }

    fn handle_keys_for_stack_mode(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let mut selected_index = self.stack_state.selected().unwrap_or(0);
        let mut return_action = None;
        match key.code {
            KeyCode::Down => {
                selected_index = min(selected_index + 1, self.stacks.len() - 1);
                return_action = Some(Action::Render);
            }
            KeyCode::Up => {
                if selected_index > 0 {
                    selected_index -= 1;
                    return_action = Some(Action::Render);
                }
            }
            KeyCode::Enter => {
                if let Some(selected) = self.stack_state.selected() {
                    self.mode = Mode::BrowseEntries;
                    self.get_enteries_for_stack(&selected_index);
                    return_action = Some(Action::Render);
                }
            }
            _ => {}
        }

        self.stack_state.select(Some(selected_index));
        Ok(return_action)
    }
    fn handle_keys_for_entry_mode(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let mut selected_index = self.entry_state.selected().unwrap_or(0);
        let mut return_action = None;
        match key.code {
            KeyCode::Down => {
                selected_index = min(selected_index + 1, self.entries.len() - 1);
                return_action = Some(Action::Render);
            }
            KeyCode::Up => {
                if selected_index > 0 {
                    selected_index -= 1;
                    return_action = Some(Action::Render);
                }
            }
            KeyCode::Esc => {
                if let Some(selected) = self.stack_state.selected() {
                    self.mode = Mode::BrowseStacks;
                    self.entries.clear();
                    return_action = Some(Action::Render);
                }
            }
            _ => {}
        }

        self.entry_state.select(Some(selected_index));
        Ok(return_action)
    }

    fn get_enteries_for_stack(&mut self, index: &usize) {
        let selected_stack = self.stacks[*index].clone();
        self.stack_state = ListState::default();

        self.entries = Entry::get_all(&selected_stack.id);
        self.entry_state.select(Some(0));
    }
}

impl Component for Home {
    fn init(&mut self, area: Rect) -> Result<()> {
        self.load_stacks();
        self.stack_state.select(Some(0));

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let a = match self.mode {
            Mode::BrowseStacks => self.handle_keys_for_stack_mode(key),
            Mode::BrowseEntries => self.handle_keys_for_entry_mode(key),
            _ => Ok(None),
        };

        a
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
