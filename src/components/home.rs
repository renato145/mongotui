use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        *,
    },
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
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

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let widget = Paragraph::new("Hello there")
            .block(Block::default().title("Mongo TUI").borders(Borders::ALL));
        f.render_widget(widget, area);
        Ok(())
    }
}
