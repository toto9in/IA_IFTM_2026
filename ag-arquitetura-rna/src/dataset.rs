use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// ---------------------------------------------------------------------------
// Banco de dados ficticio (edite aqui).
// 100 padroes com 15 entradas e 13 saidas, gerados randomicamente.
// A semente e fixa para que o dataset seja estavel entre execucoes.
// ---------------------------------------------------------------------------
pub const NUM_PADROES: usize = 100;
pub const NUM_ENTRADAS: usize = 15;
pub const NUM_SAIDAS: usize = 13;

pub const ENTRADA_MIN: f64 = 3.0;
pub const ENTRADA_MAX: f64 = 1457.0;
pub const SAIDA_MIN: f64 = 58.0;
pub const SAIDA_MAX: f64 = 312.0;

const SEMENTE: u64 = 42;

#[derive(Clone)]
pub struct Dataset {
    pub entradas: Vec<Vec<f64>>,
    pub saidas: Vec<Vec<f64>>,
}

pub fn gerar() -> Dataset {
    let mut rng = StdRng::seed_from_u64(SEMENTE);
    let mut entradas = Vec::with_capacity(NUM_PADROES);
    let mut saidas = Vec::with_capacity(NUM_PADROES);

    for _ in 0..NUM_PADROES {
        let entrada: Vec<f64> = (0..NUM_ENTRADAS)
            .map(|_| rng.gen_range(ENTRADA_MIN..=ENTRADA_MAX))
            .collect();
        let saida: Vec<f64> = (0..NUM_SAIDAS)
            .map(|_| rng.gen_range(SAIDA_MIN..=SAIDA_MAX))
            .collect();
        entradas.push(entrada);
        saidas.push(saida);
    }

    Dataset { entradas, saidas }
}

pub fn normalizar(dados: &Dataset) -> Dataset {
    let entradas = normalizar_matriz(&dados.entradas, NUM_ENTRADAS);
    let saidas = normalizar_matriz(&dados.saidas, NUM_SAIDAS);
    Dataset { entradas, saidas }
}

fn normalizar_matriz(matriz: &[Vec<f64>], colunas: usize) -> Vec<Vec<f64>> {
    let mut minimos = vec![f64::INFINITY; colunas];
    let mut maximos = vec![f64::NEG_INFINITY; colunas];

    for linha in matriz {
        for (j, &valor) in linha.iter().enumerate() {
            minimos[j] = minimos[j].min(valor);
            maximos[j] = maximos[j].max(valor);
        }
    }

    matriz
        .iter()
        .map(|linha| {
            linha
                .iter()
                .enumerate()
                .map(|(j, &valor)| {
                    let intervalo = maximos[j] - minimos[j];
                    if intervalo == 0.0 {
                        0.0
                    } else {
                        // escala [min, max] -> [-1, 1]
                        2.0 * (valor - minimos[j]) / intervalo - 1.0
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn dimensoes_corretas() {
        let d = gerar();
        assert_eq!(d.entradas.len(), NUM_PADROES);
        assert_eq!(d.saidas.len(), NUM_PADROES);
        assert!(d.entradas.iter().all(|e| e.len() == NUM_ENTRADAS));
        assert!(d.saidas.iter().all(|s| s.len() == NUM_SAIDAS));
    }

    #[test]
    fn valores_nas_faixas() {
        let d = gerar();
        for e in &d.entradas {
            for &v in e {
                assert!((ENTRADA_MIN..=ENTRADA_MAX).contains(&v));
            }
        }
        for s in &d.saidas {
            for &v in s {
                assert!((SAIDA_MIN..=SAIDA_MAX).contains(&v));
            }
        }
    }

    #[test]
    fn dataset_determinista() {
        let a = gerar();
        let b = gerar();
        assert_eq!(a.entradas, b.entradas);
        assert_eq!(a.saidas, b.saidas);
    }

    #[test]
    fn normalizacao_dentro_de_menos_um_e_um() {
        let d = normalizar(&gerar());
        for linha in d.entradas.iter().chain(d.saidas.iter()) {
            for &v in linha {
                assert!((-1.0..=1.0).contains(&v), "valor fora de [-1,1]: {v}");
            }
        }
    }
}
