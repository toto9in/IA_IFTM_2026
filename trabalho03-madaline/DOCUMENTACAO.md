# Documentação — trabalho03-madaline

Implementação de uma rede neural **MADALINE** em Rust com interface gráfica no terminal (TUI) para reconhecimento das 26 letras maiúsculas do alfabeto desenhadas pelo usuário em um grid 7×9.

---

## Sumário

1. [O que é MADALINE](#1-o-que-é-madaline)
2. [Arquitetura do projeto](#2-arquitetura-do-projeto)
3. [alphabet.rs — Dados de treinamento](#3-alphabetrs--dados-de-treinamento)
4. [madaline.rs — A rede neural](#4-madaliners--a-rede-neural)
5. [app.rs — Estado da aplicação](#5-apprs--estado-da-aplicação)
6. [main.rs — Interface e loop de eventos](#6-mainrs--interface-e-loop-de-eventos)
7. [Fluxo de uso](#7-fluxo-de-uso)
8. [Algoritmo MADALINE passo a passo](#8-algoritmo-madaline-passo-a-passo)

---

## 1. O que é MADALINE

**MADALINE** (Multiple ADAptive LInear NEuron) é uma rede neural composta por múltiplas unidades **ADALINE** (ADAptive LInear NEuron) em paralelo, propostas por Bernard Widrow e seus alunos na década de 1960.

Cada unidade ADALINE é um neurônio linear com aprendizado supervisionado pela **regra delta** (Widrow-Hoff). A saída contínua (net) é binarizada por um limiar para produzir {-1, +1}. No MADALINE, uma ADALINE é dedicada a cada classe — neste projeto, uma por letra (26 no total).

A predição final é feita pelo critério **winner-takes-all**: a ADALINE com maior valor de net vence e determina a letra reconhecida.

---

## 2. Arquitetura do projeto

```
main.rs          ← ponto de entrada, loop de eventos, renderização TUI
    │
    └── app.rs   ← estado global da aplicação (grid, predição, histórico)
            │
            └── madaline.rs  ← rede neural (Madaline + Adaline)
                    │
                    └── alphabet.rs  ← bitmaps das letras e geração de amostras
```

**Dependências (Cargo.toml):**
- `ratatui 0.29` — framework para TUI (terminal user interface)
- `crossterm 0.28` — eventos de teclado/mouse e controle do terminal
- `rand 0.8` — geração de números aleatórios para inicialização dos pesos

---

## 3. alphabet.rs — Dados de treinamento

Este arquivo define os padrões visuais das letras e gera as amostras usadas no treinamento.

### `LETRAS`

```rust
pub const LETRAS: [(&str, [[u8; 7]; 9]); 26]
```

Array com 26 entradas, uma por letra (A–Z). Cada entrada é uma tupla:
- `&str` — nome da letra (`"A"`, `"B"`, ...)
- `[[u8; 7]; 9]` — bitmap 7 colunas × 9 linhas, onde `1` = pixel ligado e `0` = pixel desligado

Exemplo — letra **A**:
```
0 0 1 1 1 0 0
0 1 0 0 0 1 0
1 0 0 0 0 0 1
1 0 0 0 0 0 1
1 1 1 1 1 1 1   ← barra horizontal do A
1 0 0 0 0 0 1
1 0 0 0 0 0 1
1 0 0 0 0 0 1
0 0 0 0 0 0 0
```

### `to_bipolar(bitmap)`

```rust
pub fn to_bipolar(bitmap: &[[u8; 7]; 9]) -> Vec<f64>
```

Converte o bitmap de `u8` (0/1) para representação **bipolar** (−1.0 / +1.0), que é o formato esperado pela rede:

| Valor original | Valor bipolar |
|:--------------:|:-------------:|
| `0`            | `-1.0`        |
| `1`            | `+1.0`        |

O vetor resultante tem 63 elementos (7 × 9 achatados em uma dimensão).

### `gerar_amostras(n_variantes)`

```rust
pub fn gerar_amostras(n_variantes: usize) -> Vec<(Vec<f64>, usize)>
```

Gera o conjunto de treinamento. Para cada uma das 26 letras:
1. Cria a amostra **base** (bitmap puro)
2. Cria `n_variantes` **variantes com ruído**, invertendo exatamente 2 pixels por variante

As posições dos pixels invertidos são calculadas deterministicamente a partir do índice da letra e da variante, garantindo reprodutibilidade sem precisar de semente aleatória.

Com `n_variantes = 4` (valor usado no treinamento), são geradas **130 amostras** (26 × 5).

Retorna: `Vec<(Vec<f64>, usize)>` — lista de pares (vetor de entrada, índice da classe).

---

## 4. madaline.rs — A rede neural

Contém a implementação completa da rede: a unidade básica `Adaline` e a rede `Madaline`.

### `EpocaInfo`

```rust
pub struct EpocaInfo {
    pub epoca: usize,  // número da época
    pub erro: f64,     // erro MSE acumulado nessa época
}
```

Registra o estado de cada época do treinamento para exibição no histórico da TUI.

---

### `Adaline` (privada)

Neurônio linear adaptativo — a unidade básica da rede.

```rust
struct Adaline {
    pesos: Vec<f64>,  // um peso por entrada (63 pesos)
    bias:  f64,       // termo de polarização
}
```

#### `Adaline::new(n)`

Inicializa os pesos e o bias com valores aleatórios no intervalo **[-0.1, 0.1)** usando `rand::thread_rng()`, seguindo o mesmo padrão do código de referência do professor.

#### `Adaline::net(x)`

Calcula a **saída linear** (net) do neurônio:

```
net = bias + Σ (peso_i × entrada_i)
```

#### `Adaline::atualizar(x, alvo, eta)`

Aplica a **regra delta** para atualizar os pesos. O alvo é bipolar (+1.0 ou -1.0).

```
y     = +1.0  se net ≥ 0.0
y     = -1.0  se net < 0.0

delta = alvo - y

peso_i += eta × delta × entrada_i
bias   += eta × delta
```

A diferença em relação ao Widrow-Hoff clássico é que o erro é calculado sobre a **saída binarizada** `y`, não sobre o valor contínuo `net`. Isso segue diretamente o código do professor.

---

### `Madaline`

A rede completa: 26 unidades ADALINE, uma por letra.

```rust
pub struct Madaline {
    adalines:     Vec<Adaline>,  // 26 unidades
    eta:          f64,           // taxa de aprendizagem
    pub max_epocas:   usize,     // limite de épocas
    pub errotolerado: f64,       // limiar de convergência
}
```

#### `Madaline::new(n_entradas, n_classes, eta, max_epocas, errotolerado)`

Cria a rede com `n_classes` unidades ADALINE, cada uma com `n_entradas` pesos.

Valores usados no projeto:
- `n_entradas = 63` (grid 7×9 achatado)
- `n_classes = 26` (letras A–Z)
- `eta = 0.01`
- `max_epocas = 500`
- `errotolerado = 0.01`

#### `Madaline::prever(x) -> (usize, f64)`

Realiza a predição para uma entrada `x`:
1. Calcula o `net` de todas as 26 ADALINEs
2. Retorna a ADALINE com **maior net** (winner-takes-all)

Retorna: `(índice_da_letra, valor_net_do_vencedor)`

#### `Madaline::treinar(amostras) -> Vec<EpocaInfo>`

Loop de treinamento supervisionado:

```
para cada época (até max_epocas):
    erro = 0.0
    para cada amostra (x, classe):
        calcula nets de todas as 26 ADALINEs
        binariza cada net → y ∈ {-1.0, +1.0}
        acumula erro MSE:  erro += 0.5 × (alvo_j - y_j)²  para cada j
        atualiza todas as ADALINEs:
            alvo = +1.0 para a ADALINE da classe correta
            alvo = -1.0 para todas as outras
    registra EpocaInfo { epoca, erro }
    se erro ≤ errotolerado → convergiu, para
```

O treinamento é **online** (atualiza pesos a cada amostra, não ao final da época).

---

## 5. app.rs — Estado da aplicação

Centraliza o estado global e a lógica de negócio da aplicação.

### `Estado`

```rust
pub enum Estado {
    Menu,        // tela inicial com as opções
    Resultados,  // exibe resultado do treinamento
    Desenhando,  // grid interativo para desenhar letras
}
```

### `App`

```rust
pub struct App {
    pub estado:          Estado,
    pub cursor:          usize,              // item selecionado no menu
    pub madaline:        Option<Madaline>,   // rede treinada (None antes do treino)
    pub historico_treino: Vec<EpocaInfo>,    // histórico de épocas
    pub convergiu:       bool,               // true se erro ≤ errotolerado
    pub grid:            [[bool; 7]; 9],     // pixels desenhados (false = apagado)
    pub predicao:        Option<(usize, f64)>, // (índice_letra, net) ou None
    pub grid_rect:       Option<Rect>,       // área do grid no terminal (para mouse)
    pub hover:           Option<(usize, usize)>, // célula sob o cursor
    pub mouse_down:      bool,               // botão esquerdo pressionado
    pub paint_value:     bool,               // true = pintando, false = apagando
}
```

### Métodos principais

| Método | O que faz |
|--------|-----------|
| `new()` | Inicializa App com estado Menu, grid vazio, sem rede |
| `ja_treinou()` | Retorna `true` se `madaline.is_some()` |
| `treinar()` | Gera amostras, cria e treina a rede, muda para estado Resultados |
| `limpar_grid()` | Zera o grid e a predição |
| `atualizar_predicao()` | Converte grid → bipolar e chama `madaline.prever()` |
| `set_pixel(row, col, value)` | Define um pixel e atualiza predição se houve mudança |
| `click_to_cell(col, row)` | Mapeia posição do terminal → célula do grid |

### `click_to_cell`

Cada célula do grid ocupa **2 caracteres de largura** e **1 linha de altura** no terminal. A função desconta a borda (+1) e a posição do bloco (`grid_rect`) antes de converter:

```rust
let cell_col = (rel_col / 2) as usize;  // 2 chars por coluna
let cell_row = rel_row as usize;
```

Retorna `None` se as coordenadas estiverem fora dos limites 7×9.

---

## 6. main.rs — Interface e loop de eventos

### `main()`

Configura o terminal para o modo TUI:
1. Ativa `raw mode` (sem echo, sem buffering de linha)
2. Ativa `alternate screen` (preserva o conteúdo anterior do terminal)
3. Habilita eventos de mouse
4. Chama `run()` e restaura o terminal ao sair

### `run(terminal, app)`

Loop principal com polling de **50ms**. Processa eventos conforme o estado atual:

| Estado | Teclado | Mouse |
|--------|---------|-------|
| `Menu` | ↑↓ navega, Enter seleciona, q/Esc sai | — |
| `Resultados` | Qualquer tecla volta ao Menu | — |
| `Desenhando` | L limpa grid, q/Esc volta ao Menu | Clique/arraste pinta, botão direito apaga |

No estado `Desenhando`, o mouse é tratado assim:
- **Botão esquerdo pressionado:** define `paint_value` como o inverso do pixel atual e inicia arraste
- **Movimento com botão pressionado:** aplica `paint_value` em cada célula percorrida
- **Botão direito:** sempre apaga o pixel

### Funções de renderização

#### `render_menu(app, f)`

Exibe o menu principal com 3 opções:
1. Treinar rede neural
2. Desenhar letra (desabilitado enquanto não treinou)
3. Sair

Mostra o status do treinamento: `✓ Rede treinada` (verde) ou mensagem de pendente (amarelo).

#### `render_resultados(app, f)`

Exibe os resultados após o treinamento:
- Título verde (convergiu) ou amarelo (não convergiu)
- Número de épocas utilizadas
- Erro final com 6 casas decimais
- Gauge de progresso (épocas usadas / 500)
- Histórico das últimas ~20 épocas com erro de cada uma

#### `render_desenhando(app, f)`

Layout horizontal: **45% grid | 55% painel de informações**

**Grid (esquerda):**
- Centralizado na área disponível
- Cada célula renderizada como 2 caracteres:
  - `██` amarelo — pixel ligado
  - `▒▒` cinza escuro — hover sobre pixel apagado
  - `░░` cinza bem escuro — pixel apagado
- Borda em laranja/amarelo negrito

**Painel de informações (direita):**
- Letra predita em destaque (verde se net > 0, vermelho se < 0)
- Valor numérico do net
- Índice da classe
- Bitmap de referência da letra reconhecida (para comparação visual)
- Controles disponíveis

---

## 7. Fluxo de uso

```
┌─────────────────────────────────────────────────────┐
│                      MENU                           │
│  > Treinar rede neural                              │
│    Desenhar letra        (bloqueado)                │
│    Sair                                             │
└────────────────┬────────────────────────────────────┘
                 │ Enter em "Treinar"
                 ▼
┌─────────────────────────────────────────────────────┐
│               TREINAMENTO (em background)           │
│  130 amostras × até 500 épocas                      │
│  Para quando erro MSE ≤ 0.01                        │
└────────────────┬────────────────────────────────────┘
                 │ Automático ao concluir
                 ▼
┌─────────────────────────────────────────────────────┐
│                   RESULTADOS                        │
│  Épocas: 312 | Erro final: 0.008421                 │
│  [████████████████████░░░░░░] 62%                   │
│  Época   1  |  erro:  52.000000                     │
│  ...                                                │
└────────────────┬────────────────────────────────────┘
                 │ Enter / Esc
                 ▼
┌─────────────────────────────────────────────────────┐
│                      MENU                           │
│    Treinar rede neural   (✓ Rede treinada)          │
│  > Desenhar letra        (desbloqueado)             │
│    Sair                                             │
└────────────────┬────────────────────────────────────┘
                 │ Enter em "Desenhar letra"
                 ▼
┌──────────────┬──────────────────────────────────────┐
│  Grid 7×9    │  Predição                            │
│              │                                      │
│  ░░░░░░░░░░░ │  Letra: A                            │
│  ░░██████░░░ │  Net:   3.241                        │
│  ░░██░░██░░░ │                                      │
│  ░░████████░ │  Referência:                         │
│  ░░██░░██░░░ │  . . # # # . .                       │
│  ░░░░░░░░░░░ │  . # . . . # .                       │
│              │  ...                                 │
│              │                                      │
│              │  Controles:                          │
│              │  Clique — pintar/apagar              │
│              │  L      — limpar grid                │
│              │  Q/Esc  — voltar ao menu             │
└──────────────┴──────────────────────────────────────┘
```

---

## 8. Algoritmo MADALINE passo a passo

### Inicialização

Cada uma das 26 unidades ADALINE recebe pesos e bias inicializados aleatoriamente em **[-0.1, 0.1)**. Isso evita que a rede comece num estado simétrico que dificultaria a aprendizagem.

### Ciclo de treinamento (por época)

Para cada época, percorre todas as 130 amostras de treinamento:

**Passo 1 — Entrada**

O bitmap da letra é convertido para bipolar: cada pixel `0` vira `-1.0` e cada pixel `1` vira `+1.0`, formando um vetor de 63 valores.

**Passo 2 — Propagação (forward pass)**

Cada uma das 26 ADALINEs calcula sua saída linear:

```
net_j = bias_j + Σ (peso_j_i × entrada_i)    para j = 0..25
```

**Passo 3 — Binarização**

Aplica o limiar 0.0 sobre cada `net_j`:

```
y_j = +1.0  se net_j ≥ 0.0
y_j = -1.0  se net_j < 0.0
```

**Passo 4 — Cálculo do erro**

Para cada ADALINE `j`, o alvo é:
- `+1.0` se `j` é a classe da letra atual
- `-1.0` para todas as outras

O erro MSE acumulado na época:

```
erro += 0.5 × (alvo_j - y_j)²    para todo j
```

Observe que `(alvo_j - y_j)` só pode ser `0`, `2` ou `-2`, então cada par errado contribui com `0.5 × 4 = 2.0` ao erro.

**Passo 5 — Atualização dos pesos**

Para cada ADALINE `j`:

```
delta_j = alvo_j - y_j

peso_j_i += eta × delta_j × entrada_i    para cada peso i
bias_j   += eta × delta_j
```

Se a ADALINE acertou (`y_j == alvo_j`), `delta_j = 0` e os pesos não mudam. Se errou, os pesos são ajustados para empurrar o `net` na direção correta.

### Critério de parada

O treinamento para quando uma das condições é satisfeita:
- `erro ≤ 0.01` (convergiu — erro tolerável atingido)
- `época == 500` (limite máximo atingido sem convergência)

### Predição

Para uma entrada desconhecida, calcula o `net` de todas as 26 ADALINEs e escolhe a de maior valor:

```
classe_predita = argmax_j (net_j)
```

A letra correspondente ao índice vencedor é exibida na TUI.
