//! Funcao de Rastrigin em 3 dimensoes.
//!
//! f(x, y, z) = 30 + (x^2 - 10 cos(2 pi x))
//!                 + (y^2 - 10 cos(2 pi y))
//!                 + (z^2 - 10 cos(2 pi z))
//!
//! Minimo global: f = 0 em (0, 0, 0).
//! Dominio usual de cada variavel: [-5.12, 5.12].

use std::f64::consts::PI;

/// Limite inferior do dominio de cada variavel.
pub const LIMITE_INFERIOR: f64 = -5.12;
/// Limite superior do dominio de cada variavel.
pub const LIMITE_SUPERIOR: f64 = 5.12;
/// Numero de variaveis (genes) do problema.
pub const NUM_GENES: usize = 3;

/// Avalia a funcao de Rastrigin para um vetor de genes.
///
/// Como o problema e de minimizacao, quanto menor o valor retornado,
/// melhor o individuo.
pub fn avaliar(genes: &[f64]) -> f64 {
    let a = 10.0;
    let n = genes.len() as f64;
    let soma: f64 = genes
        .iter()
        .map(|&x| x * x - a * (2.0 * PI * x).cos())
        .sum();
    a * n + soma
}
