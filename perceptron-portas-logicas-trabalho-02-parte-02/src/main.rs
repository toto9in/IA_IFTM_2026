mod perceptron;
mod gates;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Terminal,
};

use perceptron::{ConfigPerceptron, IteracaoTreino, Perceptron};
use gates::{amostras_para, PortaLogica};

const PORTAS: [PortaLogica; 5] = [
    PortaLogica::AND,
    PortaLogica::NAND,
    PortaLogica::OR,
    PortaLogica::NOR,
    PortaLogica::XOR,
];

enum Estado {
    Menu,
    Resultados,
}

struct App {
    estado: Estado,
    cursor: usize,
    porta_selecionada: Option<PortaLogica>,
    historico: Vec<IteracaoTreino>,
    table_state: TableState,
    scroll: usize,
    convergiu: bool,
}

impl App {
    fn new() -> Self {
        Self {
            estado: Estado::Menu,
            cursor: 0,
            porta_selecionada: None,
            historico: Vec::new(),
            table_state: TableState::default(),
            scroll: 0,
            convergiu: false,
        }
    }

    fn treinar_porta(&mut self, porta: PortaLogica) {
        let amostras = amostras_para(porta);
        let config = ConfigPerceptron {
            num_entradas: 2,
            taxa_aprendizagem: 0.1,
            max_epocas: 100,
            bias_inicial: 0.3256,
        };
        let mut p = Perceptron::new(config);
        self.historico = p.treinar(&amostras);

        let ultima = self.historico.last();
        self.convergiu = ultima.map(|it| it.erros == 0).unwrap_or(false);

        self.porta_selecionada = Some(porta);
        self.scroll = 0;
        self.table_state = TableState::default();
        if !self.historico.is_empty() {
            self.table_state.select(Some(0));
        }
        self.estado = Estado::Resultados;
    }

    fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
            self.table_state.select(Some(self.scroll));
        }
    }

    fn scroll_down(&mut self) {
        if self.scroll + 1 < self.historico.len() {
            self.scroll += 1;
            self.table_state.select(Some(self.scroll));
        }
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            match app.estado {
                Estado::Menu => draw_menu(f, &app),
                Estado::Resultados => draw_resultados(f, &mut app),
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match app.estado {
                    Estado::Menu => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => {
                            if app.cursor > 0 {
                                app.cursor -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.cursor + 1 < PORTAS.len() {
                                app.cursor += 1;
                            }
                        }
                        KeyCode::Enter => {
                            let porta = PORTAS[app.cursor];
                            app.treinar_porta(porta);
                        }
                        _ => {}
                    },
                    Estado::Resultados => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('b') => {
                            app.estado = Estado::Menu;
                        }
                        KeyCode::Up => app.scroll_up(),
                        KeyCode::Down => app.scroll_down(),
                        _ => {}
                    },
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw_menu(f: &mut ratatui::Frame, app: &App) {
    let area = f.area();

    let block = Block::default()
        .title(" Perceptron — Portas Lógicas ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),  // título
            Constraint::Length(1),  // espaço
            Constraint::Min(5),     // lista de portas
            Constraint::Length(1),  // espaço
            Constraint::Length(1),  // rodapé
        ])
        .split(inner);

    let titulo = Paragraph::new("  Selecione uma porta lógica:")
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(titulo, chunks[0]);

    // Lista de portas
    let items: Vec<Line> = PORTAS
        .iter()
        .enumerate()
        .map(|(i, porta)| {
            if i == app.cursor {
                Line::from(vec![
                    Span::styled(
                        format!("  ▶ {}", porta.nome()),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![Span::styled(
                    format!("    {}", porta.nome()),
                    Style::default().fg(Color::White),
                )])
            }
        })
        .collect();

    let lista = Paragraph::new(items);
    f.render_widget(lista, chunks[2]);

    let rodape = Paragraph::new("  [↑↓] Navegar   [Enter] Treinar   [q] Sair")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(rodape, chunks[4]);
}

fn draw_resultados(f: &mut ratatui::Frame, app: &mut App) {
    let area = f.area();

    let porta_nome = app
        .porta_selecionada
        .map(|p| p.nome())
        .unwrap_or("?");

    let outer_block = Block::default()
        .title(format!(" Resultados — Porta {} ", porta_nome))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),  // bloco de entradas
            Constraint::Min(5),     // tabela de iterações
            Constraint::Length(4),  // bloco de valores finais
            Constraint::Length(1),  // rodapé
        ])
        .split(inner);

    // ── Bloco superior: entradas ─────────────────────────────────────────────
    if let Some(porta) = app.porta_selecionada {
        let amostras = amostras_para(porta);
        let linha1 = format!(
            "  ({}, {}) → {:+.0}    ({}, {}) → {:+.0}",
            amostras[0].0[0] as i32, amostras[0].0[1] as i32, amostras[0].1,
            amostras[1].0[0] as i32, amostras[1].0[1] as i32, amostras[1].1,
        );
        let linha2 = format!(
            "  ({}, {}) → {:+.0}    ({}, {}) → {:+.0}",
            amostras[2].0[0] as i32, amostras[2].0[1] as i32, amostras[2].1,
            amostras[3].0[0] as i32, amostras[3].0[1] as i32, amostras[3].1,
        );
        let entradas_text = vec![
            Line::from(Span::styled(
                "  Entradas (x1, x2) → Alvo",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(linha1),
            Line::from(linha2),
        ];
        let entradas = Paragraph::new(entradas_text)
            .block(Block::default().borders(Borders::ALL).title(" Entradas "));
        f.render_widget(entradas, chunks[0]);
    }

    // ── Bloco central: tabela de iterações ───────────────────────────────────
    let header_cells = ["Época", "w1", "w2", "bias", "Erros"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });
    let header = Row::new(header_cells).height(1).bottom_margin(0);

    let rows: Vec<Row> = app
        .historico
        .iter()
        .map(|it| {
            let cells = vec![
                Cell::from(format!("{:>5}", it.epoca)),
                Cell::from(format!("{:>9.4}", it.pesos[0])),
                Cell::from(format!("{:>9.4}", it.pesos[1])),
                Cell::from(format!("{:>9.4}", it.bias)),
                Cell::from(format!("{:>6}", it.erros)),
            ];
            Row::new(cells)
        })
        .collect();

    let tabela = Table::new(
        rows,
        [
            Constraint::Length(7),
            Constraint::Length(11),
            Constraint::Length(11),
            Constraint::Length(11),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Histórico de Treinamento "),
    )
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(tabela, chunks[1], &mut app.table_state);

    // ── Bloco inferior: valores finais ───────────────────────────────────────
    let linha_status = if app.convergiu {
        let epocas = app.historico.len();
        format!("  Convergiu em {} época(s).", epocas)
    } else {
        format!(
            "  Não convergiu após {} épocas (XOR não é linearmente separável).",
            app.historico.len()
        )
    };

    let finais_text = if let Some(ultimo) = app.historico.last() {
        vec![
            Line::from(format!(
                "  Pesos finais:  w1 = {:.4}   w2 = {:.4}",
                ultimo.pesos[0], ultimo.pesos[1]
            )),
            Line::from(format!("  Bias final:    {:.4}", ultimo.bias)),
            Line::from(""),
            Line::from(Span::styled(
                linha_status,
                if app.convergiu {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            )),
        ]
    } else {
        vec![Line::from("  Sem dados.")]
    };

    let finais = Paragraph::new(finais_text)
        .block(Block::default().borders(Borders::ALL).title(" Resultado Final "));
    f.render_widget(finais, chunks[2]);

    // ── Rodapé ───────────────────────────────────────────────────────────────
    let rodape = Paragraph::new("  [↑↓] Scroll   [b] Voltar   [q] Sair")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(rodape, chunks[3]);
}
