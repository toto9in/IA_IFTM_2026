mod alphabet;
mod app;
mod madaline;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
        MouseButton, MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
};
use std::io;

use app::{App, Estado};

// Opções do menu: (label, requer treino)
const MENU_OPCOES: [(&str, bool); 3] = [
    ("Treinar rede neural", false),
    ("Desenhar letra", true),
    ("Sair", false),
];

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let resultado = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    resultado
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| render(app, f))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match app.estado {
                        Estado::Menu => {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                                KeyCode::Up => {
                                    if app.cursor > 0 {
                                        app.cursor -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if app.cursor + 1 < MENU_OPCOES.len() {
                                        app.cursor += 1;
                                    }
                                }
                                KeyCode::Enter => {
                                    let (_, requer_treino) = MENU_OPCOES[app.cursor];
                                    if requer_treino && !app.ja_treinou() {
                                        // ignora: opção desabilitada
                                    } else {
                                        match app.cursor {
                                            0 => app.treinar(),
                                            1 => {
                                                app.limpar_grid();
                                                app.estado = Estado::Desenhando;
                                            }
                                            _ => return Ok(()),
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Estado::Resultados => {
                            match key.code {
                                KeyCode::Enter | KeyCode::Char('q') | KeyCode::Esc => {
                                    app.estado = Estado::Menu;
                                    app.cursor = 0;
                                }
                                _ => {}
                            }
                        }
                        Estado::Desenhando => {
                            match key.code {
                                KeyCode::Esc | KeyCode::Char('q') => {
                                    app.estado = Estado::Menu;
                                    app.cursor = 0;
                                }
                                KeyCode::Char('l') | KeyCode::Char('L') => {
                                    app.limpar_grid();
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    if matches!(app.estado, Estado::Desenhando) {
                        match mouse.kind {
                            MouseEventKind::Down(MouseButton::Left) => {
                                app.mouse_down = true;
                                if let Some((row, col)) = app.click_to_cell(mouse.column, mouse.row) {
                                    app.paint_value = !app.grid[row][col];
                                    app.set_pixel(row, col, app.paint_value);
                                }
                            }
                            MouseEventKind::Down(MouseButton::Right) => {
                                if let Some((row, col)) = app.click_to_cell(mouse.column, mouse.row) {
                                    app.set_pixel(row, col, false);
                                }
                            }
                            MouseEventKind::Drag(MouseButton::Left) => {
                                if app.mouse_down {
                                    if let Some((row, col)) = app.click_to_cell(mouse.column, mouse.row) {
                                        app.set_pixel(row, col, app.paint_value);
                                    }
                                }
                                app.hover = app.click_to_cell(mouse.column, mouse.row);
                            }
                            MouseEventKind::Up(MouseButton::Left) => {
                                app.mouse_down = false;
                            }
                            MouseEventKind::Moved => {
                                app.hover = app.click_to_cell(mouse.column, mouse.row);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn render(app: &mut App, f: &mut Frame) {
    match app.estado {
        Estado::Menu => render_menu(app, f),
        Estado::Resultados => render_resultados(app, f),
        Estado::Desenhando => render_desenhando(app, f),
    }
}

// ─── MENU ────────────────────────────────────────────────────────────────────

fn render_menu(app: &App, f: &mut Frame) {
    let area = f.area();

    let bloco = Block::default()
        .title(" MADALINE — Reconhecimento de Letras A-Z ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = bloco.inner(area);
    f.render_widget(bloco, area);

    // Centraliza verticalmente
    let n_opcoes = MENU_OPCOES.len();
    let total_height = n_opcoes as u16 + 6; // opcoes + titulo + instrucoes + padding
    let v_offset = inner.height.saturating_sub(total_height) / 2;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(v_offset),
            Constraint::Length(2),
            Constraint::Length(n_opcoes as u16 + 2),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(inner);

    // Subtítulo
    let subtitulo = Paragraph::new("Rede neural MADALINE com 26 unidades ADALINE")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(subtitulo, chunks[1]);

    // Opções do menu
    let opcoes_bloco = Block::default()
        .title(" Menu ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let opcoes_inner = opcoes_bloco.inner(chunks[2]);
    f.render_widget(opcoes_bloco, chunks[2]);

    let mut linhas: Vec<Line> = Vec::new();
    for (i, (label, requer_treino)) in MENU_OPCOES.iter().enumerate() {
        let desabilitada = *requer_treino && !app.ja_treinou();
        let selecionada = i == app.cursor;

        let (prefixo, style) = if desabilitada {
            ("  ", Style::default().fg(Color::DarkGray))
        } else if selecionada {
            ("▶ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        } else {
            ("  ", Style::default().fg(Color::White))
        };

        let sufixo = if desabilitada { " (treinar primeiro)" } else { "" };
        linhas.push(Line::from(vec![
            Span::styled(prefixo, style),
            Span::styled(*label, style),
            Span::styled(sufixo, Style::default().fg(Color::DarkGray)),
        ]));
    }

    // Status da rede
    if app.ja_treinou() {
        let status = if app.convergiu {
            format!(
                " ✓ Rede treinada ({} épocas)",
                app.historico_treino.len()
            )
        } else {
            format!(
                " ⚠ Treino incompleto ({} épocas)",
                app.historico_treino.len()
            )
        };
        linhas.push(Line::from(Span::styled(
            status,
            Style::default().fg(if app.convergiu { Color::Green } else { Color::Yellow }),
        )));
    }

    let p = Paragraph::new(linhas).alignment(Alignment::Left);
    f.render_widget(p, opcoes_inner);

    // Instrucoes
    let instrucoes = Paragraph::new("↑↓ Navegar   Enter Selecionar   Q Sair")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(instrucoes, chunks[3]);
}

// ─── RESULTADOS ──────────────────────────────────────────────────────────────

fn render_resultados(app: &App, f: &mut Frame) {
    let area = f.area();

    let titulo = if app.convergiu {
        " Treino concluído — CONVERGIU! "
    } else {
        " Treino concluído — não convergiu "
    };
    let cor_titulo = if app.convergiu { Color::Green } else { Color::Yellow };

    let bloco = Block::default()
        .title(titulo)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(cor_titulo));

    let inner = bloco.inner(area);
    f.render_widget(bloco, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(inner);

    // Resumo
    let epocas = app.historico_treino.len();
    let erro_final = app.historico_treino.last().map(|e| e.erro).unwrap_or(0.0);

    let resumo_style = Style::default().fg(Color::White);
    let resumo = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("  Épocas executadas: ", resumo_style),
            Span::styled(
                format!("{}", epocas),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Erro na última época: ", resumo_style),
            Span::styled(
                format!("{:.6}", erro_final),
                Style::default().fg(if erro_final <= 0.01 { Color::Green } else { Color::Red })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);
    f.render_widget(resumo, chunks[1]);

    // Barra de progresso (épocas usadas / max_epocas)
    let max_epocas = 500u16;
    let pct = ((epocas as u16).min(max_epocas) * 100 / max_epocas) as u16;
    let gauge = Gauge::default()
        .block(Block::default().title(" Progresso ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(if app.convergiu { Color::Green } else { Color::Yellow }))
        .percent(pct)
        .label(format!("{}/{} épocas", epocas, max_epocas));
    f.render_widget(gauge, chunks[2]);

    // Histórico resumido (últimas épocas)
    if !app.historico_treino.is_empty() {
        let n_mostrar = (chunks[3].height as usize).saturating_sub(2).min(app.historico_treino.len());
        let inicio = app.historico_treino.len().saturating_sub(n_mostrar);
        let linhas: Vec<Line> = app.historico_treino[inicio..]
            .iter()
            .map(|e| {
                let cor = if e.erro <= 0.01 { Color::Green } else { Color::White };
                Line::from(Span::styled(
                    format!("  Época {:>4}  |  erro: {:>10.6}", e.epoca, e.erro),
                    Style::default().fg(cor),
                ))
            })
            .collect();

        let hist = Paragraph::new(linhas)
            .block(Block::default().title(" Histórico (últimas épocas) ").borders(Borders::ALL));
        f.render_widget(hist, chunks[3]);
    }

    // Instrução para continuar
    let instrucao = Paragraph::new("Enter / Esc  Voltar ao menu")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(instrucao, chunks[4]);
}

// ─── DESENHANDO ──────────────────────────────────────────────────────────────

fn render_desenhando(app: &mut App, f: &mut Frame) {
    let area = f.area();

    let bloco_externo = Block::default()
        .title(" MADALINE — Desenhe uma letra ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = bloco_externo.inner(area);
    f.render_widget(bloco_externo, area);

    // Grid ocupa 45% da tela, info o restante
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Fill(1)])
        .split(inner);

    render_grid(app, f, chunks[0]);
    render_info_panel(app, f, chunks[1]);
}

fn render_grid(app: &mut App, f: &mut Frame, area: Rect) {
    // Tamanho exato do grid: 7 cols × 2 chars + 2 bordas = 16, 9 rows × 1 linha + 2 bordas = 11
    let grid_w = 7u16 * 2 + 2;
    let grid_h = 9u16 * 1 + 2;

    // Centraliza o bloco dentro da área disponível
    let h_margin = area.width.saturating_sub(grid_w) / 2;
    let v_margin = area.height.saturating_sub(grid_h) / 2;
    let area = Rect {
        x: area.x + h_margin,
        y: area.y + v_margin,
        width: grid_w.min(area.width),
        height: grid_h.min(area.height),
    };

    let bloco = Block::default()
        .title(" Grid 7×9 ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let inner = bloco.inner(area);

    // Salva a posição do grid externo para mapear cliques do mouse
    app.grid_rect = Some(area);

    f.render_widget(bloco, area);

    let mut linhas: Vec<Line> = Vec::new();
    for (row, celulas) in app.grid.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::new();
        for (col, &ligado) in celulas.iter().enumerate() {
            let hover = app.hover == Some((row, col));
            let (texto, cor) = match (ligado, hover) {
                (true, true) => ("██", Color::Yellow),
                (true, false) => ("██", Color::White),
                (false, true) => ("▒▒", Color::DarkGray),
                (false, false) => ("░░", Color::Rgb(40, 40, 40)),
            };
            spans.push(Span::styled(texto, Style::default().fg(cor)));
        }
        linhas.push(Line::from(spans));
    }

    let grid_para = Paragraph::new(linhas);
    f.render_widget(grid_para, inner);
}

fn render_info_panel(app: &App, f: &mut Frame, area: Rect) {
    let bloco = Block::default()
        .title(" Predição ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = bloco.inner(area);
    f.render_widget(bloco, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Fill(1),
            Constraint::Length(5),
        ])
        .split(inner);

    // Resultado da predição
    let pred_linhas: Vec<Line> = match app.predicao {
        Some((idx, net)) => {
            let letra = alphabet::LETRAS[idx].0;
            vec![
                Line::from(Span::styled(
                    " Letra reconhecida:",
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!("  {}", letra),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::SLOW_BLINK),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled(" Net: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("{:+.3}", net),
                        Style::default().fg(if net > 0.0 { Color::Green } else { Color::Red }),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    format!(" Índice: {}", idx),
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        }
        None => vec![
            Line::from(Span::styled(
                " Desenhe uma letra",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                " no grid ao lado",
                Style::default().fg(Color::DarkGray),
            )),
        ],
    };
    let pred_para = Paragraph::new(pred_linhas).wrap(Wrap { trim: false });
    f.render_widget(pred_para, chunks[0]);

    // Exibe o bitmap da letra predita como referência
    if let Some((idx, _)) = app.predicao {
        let bitmap = &alphabet::LETRAS[idx].1;
        let mut ref_linhas: Vec<Line> = vec![Line::from(Span::styled(
            " Referência:",
            Style::default().fg(Color::Gray),
        ))];
        for row in bitmap.iter() {
            let spans: Vec<Span> = row
                .iter()
                .map(|&b| {
                    let (txt, cor) = if b == 1 {
                        ("█", Color::Cyan)
                    } else {
                        ("░", Color::Rgb(30, 30, 30))
                    };
                    Span::styled(txt, Style::default().fg(cor))
                })
                .collect();
            ref_linhas.push(Line::from(spans));
        }
        let ref_para = Paragraph::new(ref_linhas);
        f.render_widget(ref_para, chunks[1]);
    }

    // Instruções
    let instrucoes = Paragraph::new(vec![
        Line::from(Span::styled(
            " Controles:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "  Click E — ligar pixel",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  Click D — desligar pixel",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  L       — limpar grid",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  Q/Esc   — voltar ao menu",
            Style::default().fg(Color::DarkGray),
        )),
    ]);
    f.render_widget(instrucoes, chunks[2]);
}
