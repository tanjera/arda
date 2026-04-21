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
    ratatui::run(|terminal| Map::new(100).run(terminal))
}

struct Map {
    name: String,
    land: Vec<Vec<Land>>,
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
                let value = rng.gen_range(0..4);
                inner.push(match value {
                    0 => Land::Sand,
                    1 => Land::Dirt,
                    2 => Land::Forest,
                    3 => Land::Water,
                    _ => Land::Dirt,
                });
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

        let mut para = Vec::new();

        for row in &self.land {


            let spans: Vec<Span> = row
                .iter()
                .map(|c| {
                    let style = match c {
                        Land::Sand => Style::default().fg(Color::Rgb(210, 180, 140)),
                        Land::Dirt => Style::default().fg(Color::Rgb(150, 75, 0)),
                        Land::Forest => Style::default().fg(Color::Green),
                        Land::Water => Style::default().fg(Color::Blue),
                        _ => Style::default(),
                    };

                    Span::styled("█", style)
                })
                .collect();

            let line = Line::from(spans);

            para.push(line);
        }

        let map = Paragraph::new(para)
            .block(Block::bordered()
                .title("Map")
                .padding(Padding::new(1,1,1,1)));

        frame.render_widget(map, map_area);
    }
}