use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, List, ListItem, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use rand::Rng;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| Map::new(10).run(terminal))
}

struct Map {
    name: String,
    land: Vec<Vec<i32>>,
}

enum Land {
    Sand,
    Dirt,
    Forest,
    Water,
}

impl Map {
    fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut outer = Vec::with_capacity(size);

        for _ in 0..size {
            let mut inner = Vec::with_capacity(size);

            for _ in 0..size {
                let value = rng.gen_range(0..10);
                inner.push(value);
            }
            outer.push(inner);
        }

        Map {
            name: String::new(),
            land: outer
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [instructions_area, map_area] = frame.area().layout(&layout);

        let instructions = Paragraph::new("Press `q` to quit")
            .block(Block::bordered()
                .title("Instructions")
                .padding(Padding::new(1,1,0,0)));

        (frame).render_widget(instructions, instructions_area);

        let mut paragraph = String::new();
        for row in &self.land {
            let mut line = String::new();
            for col in row {
                line.push_str(col.to_string().as_str());
            }
            paragraph.push_str(&line);
            paragraph.push('\n');
        }

        let map = Paragraph::new(paragraph)
            .block(Block::bordered()
                .title("Map")
                .padding(Padding::new(1,1,1,1)));

        frame.render_widget(map, map_area);
    }
}