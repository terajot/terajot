use std::{collections::HashMap, time::Duration};

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

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    stacks: Vec<Stack>,
    stack_state: ListState,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_stacks(&mut self) {
        self.stacks = Stack::get_all();
        self.stack_state = ListState::default();
    }
}

impl Component for Home {

  fn init(&mut self, area: Rect) -> Result<()> {
      self.load_stacks();
      Ok(())
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
            Action::Tick => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(3)],
        )
        .split(frame.size());

        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("tooltips and status will go here"),
            main_layout[1],
        );

        let inner_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(25), Constraint::Percentage(75)],
        )
        .split(main_layout[0]);

        self.stack_state.select(Some(2));

        let list = List::new(
            self.stacks
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<_>>(),
        )
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
        frame.render_stateful_widget(list, inner_layout[0], &mut self.stack_state);
        frame.render_widget(
            Block::default().borders(Borders::ALL).title("Entries"),
            inner_layout[1],
        );
        Ok(())
    }
}
