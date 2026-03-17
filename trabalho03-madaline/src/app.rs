use ratatui::layout::Rect;

use crate::alphabet::{LETRAS, gerar_amostras, to_bipolar};
use crate::madaline::{EpocaInfo, Madaline};

pub enum Estado {
    Menu,
    Resultados,
    Desenhando,
}

pub struct App {
    pub estado: Estado,
    pub cursor: usize,

    // Rede neural
    pub madaline: Option<Madaline>,
    pub historico_treino: Vec<EpocaInfo>,
    pub convergiu: bool,

    // Grid de desenho (9 linhas × 7 colunas)
    pub grid: [[bool; 7]; 9],
    pub predicao: Option<(usize, f64)>,

    // Posição do grid no terminal (para mapear cliques do mouse)
    pub grid_rect: Option<Rect>,
    // Célula sob o cursor do mouse
    pub hover: Option<(usize, usize)>,
    // Se o botão esquerdo está pressionado (para drag)
    pub mouse_down: bool,
    // Valor que está sendo pintado no drag (true = ligar, false = desligar)
    pub paint_value: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            estado: Estado::Menu,
            cursor: 0,
            madaline: None,
            historico_treino: Vec::new(),
            convergiu: false,
            grid: [[false; 7]; 9],
            predicao: None,
            grid_rect: None,
            hover: None,
            mouse_down: false,
            paint_value: true,
        }
    }

    pub fn ja_treinou(&self) -> bool {
        self.madaline.is_some()
    }

    pub fn treinar(&mut self) {
        let amostras = gerar_amostras(4);
        let mut m = Madaline::new(63, 26, 0.01, 500, 0.01);
        let errotolerado = m.errotolerado;
        let historico = m.treinar(&amostras);
        self.convergiu = historico.last().map(|e| e.erro <= errotolerado).unwrap_or(false);
        self.historico_treino = historico;
        self.madaline = Some(m);
        self.estado = Estado::Resultados;
    }

    pub fn limpar_grid(&mut self) {
        self.grid = [[false; 7]; 9];
        self.predicao = None;
    }

    pub fn atualizar_predicao(&mut self) {
        if let Some(m) = &self.madaline {
            let entradas: Vec<f64> = self.grid.iter()
                .flatten()
                .map(|&b| if b { 1.0 } else { -1.0 })
                .collect();
            self.predicao = Some(m.prever(&entradas));
        }
    }

    #[allow(dead_code)]
    pub fn toggle_pixel(&mut self, row: usize, col: usize) {
        self.grid[row][col] = !self.grid[row][col];
        self.atualizar_predicao();
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, value: bool) {
        if self.grid[row][col] != value {
            self.grid[row][col] = value;
            self.atualizar_predicao();
        }
    }

    #[allow(dead_code)]
    pub fn letra_predita(&self) -> Option<&str> {
        self.predicao.map(|(idx, _)| LETRAS[idx].0)
    }

    /// Carrega o bitmap de uma letra no grid (para demonstração)
    #[allow(dead_code)]
    pub fn carregar_letra(&mut self, idx: usize) {
        let bitmap = to_bipolar(&LETRAS[idx].1);
        for (i, &v) in bitmap.iter().enumerate() {
            let row = i / 7;
            let col = i % 7;
            self.grid[row][col] = v > 0.0;
        }
        self.atualizar_predicao();
    }

    pub fn click_to_cell(&self, col: u16, row: u16) -> Option<(usize, usize)> {
        let rect = self.grid_rect?;
        let inner_x = rect.x + 1;
        let inner_y = rect.y + 1;

        if col < inner_x || row < inner_y {
            return None;
        }

        let rel_col = col - inner_x;
        let rel_row = row - inner_y;

        // Cada célula ocupa 2 caracteres de largura e 1 de altura
        let cell_col = (rel_col / 2) as usize;
        let cell_row = rel_row as usize;

        if cell_col < 7 && cell_row < 9 {
            Some((cell_row, cell_col))
        } else {
            None
        }
    }
}
