use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct App {
    counter: u8,
    should_quit: bool,
    items: Vec<String>,
    selected_index: usize,
}

impl App {
    fn new() -> Self {
        Self {
            counter: 0,
            should_quit: false,
            items: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
                "Item 4".to_string(),
                "Item 5".to_string(),
            ],
            selected_index: 0,
        }
    }

    fn tick(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char(' ') => self.tick(),
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index < self.items.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Char('a') => {
                self.items.push(format!("New Item {}", self.items.len() + 1));
            }
            KeyCode::Char('d') => {
                if !self.items.is_empty() {
                    self.items.remove(self.selected_index);
                    if self.selected_index >= self.items.len() && self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    
    loop {
        terminal.draw(|frame| render(&app, frame))?;
        
        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        }
        
        if app.should_quit {
            break;
        }
    }
    
    Ok(())
}

fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(app, frame, chunks[0]);
    render_body(app, frame, chunks[1]);
    render_footer(app, frame, chunks[2]);
}

fn render_header(_app: &App, frame: &mut Frame, area: Rect) {
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Welcome to "),
            Span::styled("Ratatui", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Example!"),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Header"))
    .alignment(Alignment::Center);
    
    frame.render_widget(header, area);
}

fn render_body(app: &App, frame: &mut Frame, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_list(app, frame, body_chunks[0]);
    render_info(app, frame, body_chunks[1]);
}

fn render_list(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let content = if i == app.selected_index {
                Line::from(vec![
                    Span::raw("> "),
                    Span::styled(item, Style::default().fg(Color::Yellow)),
                ])
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw(item),
                ])
            };
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List (↑/↓ to navigate, 'a' to add, 'd' to delete)"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(list, area);
}

fn render_info(app: &App, frame: &mut Frame, area: Rect) {
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(3)])
        .split(area);

    let info = Paragraph::new(vec![
        Line::from(format!("Counter: {}", app.counter)),
        Line::from(format!("Selected: {}", app.selected_index)),
        Line::from(format!("Items: {}", app.items.len())),
    ])
    .block(Block::default().borders(Borders::ALL).title("Info"))
    .style(Style::default().fg(Color::White));

    frame.render_widget(info, info_chunks[0]);

    let progress = app.counter as f64 / 100.0;
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent((progress * 100.0) as u16)
        .label(format!("{}%", (progress * 100.0) as u16));

    frame.render_widget(gauge, info_chunks[1]);
}

fn render_footer(_app: &App, frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::raw("Press "),
        Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" to quit, "),
        Span::styled("Space", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" to increment counter"),
    ]))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
