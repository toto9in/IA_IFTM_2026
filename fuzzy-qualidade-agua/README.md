# Fuzzy — Qualidade da Água (SABESP)

Sistema de inferência fuzzy **Mamdani** (mín-máx + defuzzificação por
centroide) para análise de potabilidade da água, em Rust com TUI (ratatui).

## Variáveis

**Entradas** (funções de pertinência trapezoidais):

- **Cor aparente** (UH): boa (≤5), adequada (5–15), inadequada (>15)
- **pH**: inadequado baixo, adequado baixo, bom (6,5–8,5), adequado alto, inadequado alto
- **Turbidez** (UT): boa (≤1), adequada (1–5), inadequada (>5)

**Saída**: qualidade da água em [0, 1] — inadequada, adequada, boa.

## Base de regras

45 regras (3 cor × 5 pH × 3 turbidez), das Tabelas 2.6/2.7/2.8. Cada tabela
corresponde a um termo da aparência (cor). Ver `src/regras.rs`.

## Executar

```bash
cargo run
```

TUI: preencha cor, pH e turbidez → `Enter` calcula. A tela de resultado mostra
os graus de pertinência de cada entrada, as regras ativadas com suas forças e a
qualidade defuzzificada com a classificação linguística.

## Exemplo do enunciado

Cor 15 UH, pH 7, turbidez 0 UT → **0,575** → **adequada**.

```bash
cargo test
```

> Os coeficientes dos trapézios foram aproximados a partir das figuras do
> material (2.10 a 2.13), respeitando os limites textuais da SABESP.
