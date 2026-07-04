mod fuzzy;
mod inferencia;
mod regras;
mod variaveis;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};
use std::io;

use inferencia::{GrauEntrada, Resultado, inferir};

enum Estado {
    Entrada,
    Resultado,
}

struct App {
    estado: Estado,
    campo_ativo: usize,
    input_cor: String,
    input_ph: String,
    input_turbidez: String,
    resultado: Option<Resultado>,
    erro: Option<String>,
    table_state: TableState,
    scroll: usize,
}

impl App {
    fn new() -> Self {
        Self {
            estado: Estado::Entrada,
            campo_ativo: 0,
            input_cor: "15".to_string(),
            input_ph: "7".to_string(),
            input_turbidez: "0".to_string(),
            resultado: None,
            erro: None,
            table_state: TableState::default(),
            scroll: 0,
        }
    }

    fn campo_mut(&mut self) -> &mut String {
        match self.campo_ativo {
            0 => &mut self.input_cor,
            1 => &mut self.input_ph,
            _ => &mut self.input_turbidez,
        }
    }

    fn calcular(&mut self) {
        let cor = self.input_cor.trim().parse::<f64>();
        let ph = self.input_ph.trim().parse::<f64>();
        let turbidez = self.input_turbidez.trim().parse::<f64>();

        match (cor, ph, turbidez) {
            (Ok(c), Ok(h), Ok(t)) => {
                self.resultado = Some(inferir(c, h, t));
                self.erro = None;
                self.scroll = 0;
                self.table_state = TableState::default();
                self.table_state.select(Some(0));
                self.estado = Estado::Resultado;
            }
            _ => {
                self.erro = Some("Preencha cor, pH e turbidez com números válidos.".to_string());
            }
        }
    }

    fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
            self.table_state.select(Some(self.scroll));
        }
    }

    fn scroll_down(&mut self) {
        let n = self.resultado.as_ref().map(|r| r.regras.len()).unwrap_or(0);
        if self.scroll + 1 < n {
            self.scroll += 1;
            self.table_state.select(Some(self.scroll));
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| match app.estado {
            Estado::Entrada => draw_entrada(f, &app),
            Estado::Resultado => draw_resultado(f, &mut app),
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match app.estado {
                    Estado::Entrada => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Tab | KeyCode::Down => {
                            app.campo_ativo = (app.campo_ativo + 1) % 3;
                        }
                        KeyCode::Up => {
                            app.campo_ativo = (app.campo_ativo + 2) % 3;
                        }
                        KeyCode::Backspace => {
                            app.campo_mut().pop();
                        }
                        KeyCode::Char(c) if c.is_ascii_digit() || c == '.' || c == '-' => {
                            app.campo_mut().push(c);
                        }
                        KeyCode::Enter => app.calcular(),
                        _ => {}
                    },
                    Estado::Resultado => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('b') | KeyCode::Esc => {
                            app.estado = Estado::Entrada;
                        }
                        KeyCode::Up => app.scroll_up(),
                        KeyCode::Down => app.scroll_down(),
                        _ => {}
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw_entrada(f: &mut ratatui::Frame, app: &App) {
    let area = f.area();

    let outer = Block::default()
        .title(" Lógica Fuzzy — Qualidade da Água (SABESP) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1), // título
            Constraint::Length(1), // espaço
            Constraint::Length(3), // cor
            Constraint::Length(3), // pH
            Constraint::Length(3), // turbidez
            Constraint::Length(2), // erro / dica
            Constraint::Min(0),
            Constraint::Length(1), // rodapé
        ])
        .split(inner);

    let titulo = Paragraph::new("  Informe as variáveis de entrada:")
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(titulo, chunks[0]);

    campo(
        f,
        chunks[2],
        "Cor aparente (UH, 0–30)",
        &app.input_cor,
        app.campo_ativo == 0,
    );
    campo(
        f,
        chunks[3],
        "pH (0–14)",
        &app.input_ph,
        app.campo_ativo == 1,
    );
    campo(
        f,
        chunks[4],
        "Turbidez (UT, 0–10)",
        &app.input_turbidez,
        app.campo_ativo == 2,
    );

    if let Some(erro) = &app.erro {
        let p = Paragraph::new(format!("  {}", erro)).style(Style::default().fg(Color::Red));
        f.render_widget(p, chunks[5]);
    }

    let rodape = Paragraph::new("  [Tab/↑↓] Campo   [Enter] Calcular   [q] Sair")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(rodape, chunks[7]);
}

fn campo(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    titulo: &str,
    valor: &str,
    ativo: bool,
) {
    let estilo = if ativo {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let cursor = if ativo { "_" } else { "" };
    let bloco = Block::default()
        .title(format!(" {} ", titulo))
        .borders(Borders::ALL)
        .border_style(estilo);
    let texto = Paragraph::new(format!(" {}{}", valor, cursor)).block(bloco);
    f.render_widget(texto, area);
}

fn draw_resultado(f: &mut ratatui::Frame, app: &mut App) {
    let area = f.area();

    let outer = Block::default()
        .title(" Resultado da Inferência Fuzzy ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(9), // graus de pertinência (entradas)
            Constraint::Min(5),    // regras ativadas
            Constraint::Length(5), // saída defuzzificada
            Constraint::Length(1), // rodapé
        ])
        .split(inner);

    let Some(resultado) = &app.resultado else {
        return;
    };

    // --- Graus de pertinência das entradas (três colunas) ---
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[0]);

    painel_graus(
        f,
        cols[0],
        &format!("Cor = {} UH", app.input_cor),
        &resultado.graus_cor,
    );
    painel_graus(
        f,
        cols[1],
        &format!("pH = {}", app.input_ph),
        &resultado.graus_ph,
    );
    painel_graus(
        f,
        cols[2],
        &format!("Turbidez = {} UT", app.input_turbidez),
        &resultado.graus_turbidez,
    );

    // --- Regras ativadas ---
    let header = Row::new(
        ["Cor", "pH", "Turbidez", "→ Qualidade", "Força"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            }),
    );

    let linhas: Vec<Row> = resultado
        .regras
        .iter()
        .map(|r| {
            Row::new(vec![
                Cell::from(r.cor.clone()),
                Cell::from(r.ph.clone()),
                Cell::from(r.turbidez.clone()),
                Cell::from(r.saida.clone()),
                Cell::from(format!("{:.3}", r.forca)),
            ])
        })
        .collect();

    let tabela = Table::new(
        linhas,
        [
            Constraint::Length(11),
            Constraint::Length(17),
            Constraint::Length(11),
            Constraint::Length(12),
            Constraint::Length(7),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Regras ativadas ({}) ", resultado.regras.len())),
    )
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(tabela, chunks[1], &mut app.table_state);

    // --- Saída defuzzificada ---
    let texto = vec![
        Line::from(vec![
            Span::styled(
                "  Qualidade (defuzzificada): ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:.3}", resultado.qualidade),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Classificação: ", Style::default().fg(Color::White)),
            Span::styled(
                resultado.classificacao.to_uppercase(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let saida = Paragraph::new(texto).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Defuzzificação (centroide) "),
    );
    f.render_widget(saida, chunks[2]);

    let rodape = Paragraph::new("  [↑↓] Rolar regras   [b/Esc] Voltar   [q] Sair")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(rodape, chunks[3]);
}

fn painel_graus(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    titulo: &str,
    graus: &[GrauEntrada],
) {
    let linhas: Vec<Line> = graus
        .iter()
        .map(|g| {
            let cor = if g.grau > 0.0 {
                Color::Green
            } else {
                Color::DarkGray
            };
            Line::from(Span::styled(
                format!(" {:<16} {:.2}", g.termo, g.grau),
                Style::default().fg(cor),
            ))
        })
        .collect();

    let p = Paragraph::new(linhas).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", titulo)),
    );
    f.render_widget(p, area);
}
