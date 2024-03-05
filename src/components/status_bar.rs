use super::Component;
use crate::tui::Frame;
use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct StatusBar {}

impl StatusBar {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for StatusBar {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        f.render_widget(Paragraph::new("Some status bar here :D"), area);
        Ok(())
    }
}
