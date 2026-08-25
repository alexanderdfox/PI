//! Colorful TUI: Leibniz π series → last digits of 10-decimal approx
//! drawn as text on a circular path in the terminal.

use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};

// ── series state (mirrors the original Rust snippet) ───────────────────────
struct Series {
    sum: f64,
    sign: f64,
    n: u64,
    terms: u64,
    digits: String,
    max_digits: usize,
}

impl Series {
    fn new(max_digits: usize) -> Self {
        Self {
            sum: 0.0,
            sign: 1.0,
            n: 1,
            terms: 0,
            digits: String::new(),
            max_digits,
        }
    }

    fn step(&mut self) -> f64 {
        self.sum += self.sign / self.n as f64;
        let pi_approx = 4.0 * self.sum;
        // identical to format!("{:.10}", pi_approx).chars().last()
        let s = format!("{:.10}", pi_approx);
        if let Some(last) = s.chars().last() {
            self.digits.push(last);
            if self.digits.len() > self.max_digits {
                self.digits = self.digits[self.digits.len() - self.max_digits..].to_string();
            }
        }
        self.n += 2;
        self.sign = -self.sign;
        self.terms += 1;
        pi_approx
    }

    fn reset(&mut self) {
        self.sum = 0.0;
        self.sign = 1.0;
        self.n = 1;
        self.terms = 0;
        self.digits.clear();
    }
}

struct App {
    series: Series,
    running: bool,
    spinning: bool,
    speed: u32,
    angle: f64,
    pi: f64,
}

impl App {
    fn new() -> Self {
        Self {
            series: Series::new(160),
            running: true,
            spinning: true,
            speed: 3,
            angle: 0.0,
            pi: 4.0,
        }
    }

    fn on_tick(&mut self) {
        if self.running {
            for _ in 0..self.speed {
                self.pi = self.series.step();
            }
        }
        if self.spinning {
            self.angle += 0.035;
            if self.angle > std::f64::consts::TAU {
                self.angle -= std::f64::consts::TAU;
            }
        }
    }
}

fn digit_color(_d: char, idx: usize, newest: bool) -> Color {
    if newest {
        return Color::Rgb(255, 215, 0);
    }
    let hue = (idx as f64 * 18.0) % 360.0;
    hsv_to_rgb(hue, 0.85, 1.0)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color::Rgb(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn draw_digit_circle(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(0, 180, 220)))
        .title(Span::styled(
            " π  ·  digits on circular path ",
            Style::default()
                .fg(Color::Rgb(0, 240, 255))
                .add_modifier(Modifier::BOLD),
        ));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let digits = &app.series.digits;
    let count = digits.len();
    if count == 0 {
        return;
    }

    let cx = inner.x as f64 + inner.width as f64 / 2.0;
    let cy = inner.y as f64 + inner.height as f64 / 2.0;
    let rx = (inner.width as f64 / 2.0 - 2.0).max(4.0);
    let ry = (inner.height as f64 / 2.0 - 2.0).max(3.0);

    let angle_step = std::f64::consts::TAU / count as f64;

    for (i, ch) in digits.chars().enumerate() {
        let a = -std::f64::consts::FRAC_PI_2 + app.angle + i as f64 * angle_step;
        let x = cx + a.cos() * rx;
        let y = cy + a.sin() * ry;

        let col = x.round() as u16;
        let row = y.round() as u16;

        if col < inner.x || col >= inner.x + inner.width {
            continue;
        }
        if row < inner.y || row >= inner.y + inner.height {
            continue;
        }

        let newest = i + 1 == count;
        let color = digit_color(ch, i, newest);
        let style = if newest {
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
        } else {
            Style::default().fg(color)
        };

        let cell_area = Rect {
            x: col,
            y: row,
            width: 1,
            height: 1,
        };
        frame.render_widget(
            Paragraph::new(Span::styled(ch.to_string(), style)),
            cell_area,
        );
    }

    // centre readout
    let pi_str = format!("{:.10}", app.pi);
    let centre_lines = vec![
        Line::from(Span::styled(
            "π",
            Style::default()
                .fg(Color::Rgb(0, 240, 255))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::raw("≈ "),
            Span::styled(
                pi_str,
                Style::default()
                    .fg(Color::Rgb(255, 215, 0))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            format!("terms {}", app.series.terms),
            Style::default().fg(Color::Rgb(120, 140, 160)),
        )),
    ];
    let centre_h = 3u16;
    let centre_w = 18u16;
    let centre_area = Rect {
        x: (cx as u16).saturating_sub(centre_w / 2),
        y: (cy as u16).saturating_sub(centre_h / 2),
        width: centre_w,
        height: centre_h,
    };
    if centre_area.x >= inner.x
        && centre_area.y >= inner.y
        && centre_area.x + centre_area.width <= inner.x + inner.width
        && centre_area.y + centre_area.height <= inner.y + inner.height
    {
        frame.render_widget(Clear, centre_area);
        frame.render_widget(
            Paragraph::new(centre_lines).alignment(Alignment::Center),
            centre_area,
        );
    }
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let status = format!(
        "  digits {}  ·  speed ×{}  ·  {}  ·  {}  ·  [Space] pause  [r] reset  [s] spin  [+/-] speed  [q] quit  ",
        app.series.digits.len(),
        app.speed,
        if app.running { "RUNNING" } else { "PAUSED " },
        if app.spinning { "SPIN" } else { "STATIC" },
    );
    let style = Style::default()
        .fg(Color::Rgb(180, 220, 255))
        .bg(Color::Rgb(10, 20, 35));
    frame.render_widget(
        Paragraph::new(status).style(style).alignment(Alignment::Left),
        area,
    );
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)].as_ref())
        .split(frame.size());

    draw_digit_circle(frame, chunks[0], app);
    draw_status(frame, chunks[1], app);
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.running = !app.running,
                        KeyCode::Char('r') => {
                            app.series.reset();
                            app.pi = 4.0;
                        }
                        KeyCode::Char('s') => app.spinning = !app.spinning,
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.speed = (app.speed + 1).min(50);
                        }
                        KeyCode::Char('-') => {
                            app.speed = app.speed.saturating_sub(1).max(1);
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
