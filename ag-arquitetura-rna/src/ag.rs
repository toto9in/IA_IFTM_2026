use rand::Rng;

use crate::cromossomo::{self, Cromossomo, NUM_POSICOES};
use crate::dataset::{self, Dataset, NUM_ENTRADAS, NUM_SAIDAS};
use crate::mlp::Mlp;

pub const TAMANHO_POPULACAO: usize = 40;
pub const MAX_GERACOES: usize = 100;
pub const NUM_GENITORES: usize = 20;
pub const TAXA_MUTACAO: f64 = 0.05;

#[derive(Clone)]
pub struct Individuo {
    pub cromossomo: Cromossomo,
    pub fitness: f64,
}

pub struct RegistroGeracao {
    pub geracao: usize,
    pub melhor_mse: f64,
    pub mse_medio: f64,
}

pub struct ResultadoAG {
    pub melhor: Individuo,
    pub historico: Vec<RegistroGeracao>,
}

pub fn avaliar(cromo: &Cromossomo, dados: &Dataset, rng: &mut impl Rng) -> f64 {
    let dados_treino = if cromossomo::normaliza(cromo) {
        dataset::normalizar(dados)
    } else {
        dados.clone()
    };

    let mut rede = Mlp::nova(
        NUM_ENTRADAS,
        cromossomo::neuronios(cromo),
        cromossomo::camadas(cromo),
        NUM_SAIDAS,
        rng,
    );
    rede.treinar(
        &dados_treino,
        cromossomo::taxa(cromo),
        cromossomo::epocas(cromo),
        cromossomo::online(cromo),
    );

    rede.mse(&dados_treino)
}

/// avalia varios cromossomos em paralelo (um treino de RNA por cromossomo,
/// independentes entre si). Usa apenas a biblioteca padrao (std::thread::scope);
/// cada thread tem seu proprio RNG para inicializar os pesos. A ordem de saida
/// corresponde a ordem de entrada.
fn avaliar_paralelo(cromos: Vec<Cromossomo>, dados: &Dataset) -> Vec<Individuo> {
    if cromos.is_empty() {
        return Vec::new();
    }
    let num_threads = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
        .min(cromos.len());
    let tamanho_bloco = cromos.len().div_ceil(num_threads);
    let blocos: Vec<Vec<Cromossomo>> = cromos
        .chunks(tamanho_bloco)
        .map(|bloco| bloco.to_vec())
        .collect();

    std::thread::scope(|escopo| {
        let handles: Vec<_> = blocos
            .into_iter()
            .map(|bloco| {
                escopo.spawn(move || {
                    let mut rng = rand::thread_rng();
                    bloco
                        .into_iter()
                        .map(|cromo| {
                            let fitness = avaliar(&cromo, dados, &mut rng);
                            Individuo {
                                cromossomo: cromo,
                                fitness,
                            }
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    })
}

pub fn executar(dados: &Dataset) -> ResultadoAG {
    let mut rng = rand::thread_rng();

    let cromos_iniciais: Vec<Cromossomo> = (0..TAMANHO_POPULACAO)
        .map(|_| cromossomo::cromossomo_aleatorio(&mut rng))
        .collect();
    let mut populacao = avaliar_paralelo(cromos_iniciais, dados);
    eprintln!("  populacao inicial avaliada");

    let mut historico = Vec::with_capacity(MAX_GERACOES);
    let mut melhor_global = melhor_da_populacao(&populacao).clone();

    for geracao in 0..MAX_GERACOES {
        let genitores: Vec<Cromossomo> = (0..NUM_GENITORES)
            .map(|_| selecao_roleta(&populacao, &mut rng).cromossomo.clone())
            .collect();

        let mut cromos_filhos: Vec<Cromossomo> = Vec::new();
        let mut i = 0;
        while i + 1 < genitores.len() {
            let (mut f1, mut f2) = cruzar(&genitores[i], &genitores[i + 1], &mut rng);
            mutar(&mut f1, &mut rng);
            mutar(&mut f2, &mut rng);
            cromos_filhos.push(f1);
            cromos_filhos.push(f2);
            i += 2;
        }

        let filhos = avaliar_paralelo(cromos_filhos, dados);

        let elite = melhor_da_populacao(&populacao).clone();
        let mut nova_populacao = Vec::with_capacity(TAMANHO_POPULACAO);
        nova_populacao.push(elite);
        nova_populacao.extend(filhos);

        if nova_populacao.len() < TAMANHO_POPULACAO {
            let mut ordenados = populacao.clone();
            ordenados.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
            let mut k = 0;
            while nova_populacao.len() < TAMANHO_POPULACAO {
                nova_populacao.push(ordenados[k % ordenados.len()].clone());
                k += 1;
            }
        } else {
            nova_populacao.truncate(TAMANHO_POPULACAO);
        }

        populacao = nova_populacao;

        let melhor_atual = melhor_da_populacao(&populacao);
        if melhor_atual.fitness < melhor_global.fitness {
            melhor_global = melhor_atual.clone();
        }
        let media: f64 =
            populacao.iter().map(|ind| ind.fitness).sum::<f64>() / populacao.len() as f64;
        historico.push(RegistroGeracao {
            geracao,
            melhor_mse: melhor_global.fitness,
            mse_medio: media,
        });
        eprintln!(
            "  geracao {:>3}/{} | melhor MSE = {:>14.6}",
            geracao + 1,
            MAX_GERACOES,
            melhor_global.fitness
        );
    }

    ResultadoAG {
        melhor: melhor_global,
        historico,
    }
}

fn melhor_da_populacao(populacao: &[Individuo]) -> &Individuo {
    populacao
        .iter()
        .min_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap()
}

fn selecao_roleta<'a>(populacao: &'a [Individuo], rng: &mut impl Rng) -> &'a Individuo {
    let aptidoes: Vec<f64> = populacao
        .iter()
        .map(|ind| 1.0 / (1.0 + ind.fitness))
        .collect();
    let total: f64 = aptidoes.iter().sum();

    let alvo = rng.gen::<f64>() * total;
    let mut acumulado = 0.0;
    for (i, &aptidao) in aptidoes.iter().enumerate() {
        acumulado += aptidao;
        if acumulado >= alvo {
            return &populacao[i];
        }
    }
    populacao.last().unwrap()
}

fn cruzar(pai1: &Cromossomo, pai2: &Cromossomo, rng: &mut impl Rng) -> (Cromossomo, Cromossomo) {
    let corte = rng.gen_range(1..NUM_POSICOES);
    let mut filho1 = Vec::with_capacity(NUM_POSICOES);
    let mut filho2 = Vec::with_capacity(NUM_POSICOES);
    for pos in 0..NUM_POSICOES {
        if pos < corte {
            filho1.push(pai1[pos].clone());
            filho2.push(pai2[pos].clone());
        } else {
            filho1.push(pai2[pos].clone());
            filho2.push(pai1[pos].clone());
        }
    }
    (filho1, filho2)
}

fn mutar(cromo: &mut Cromossomo, rng: &mut impl Rng) {
    if rng.gen::<f64>() < TAXA_MUTACAO {
        let posicao = rng.gen_range(0..NUM_POSICOES);
        cromo[posicao] = cromossomo::gene_aleatorio(posicao, rng);
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn cromo(vals: [&str; 6]) -> Cromossomo {
        vals.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn cruzamento_preserva_tamanho_e_mistura() {
        let mut rng = rand::thread_rng();
        let p1 = cromo(["2", "2", "0.010000", "20", "1", "1"]);
        let p2 = cromo(["15", "5", "0.090000", "999", "2", "2"]);
        for _ in 0..100 {
            let (f1, f2) = cruzar(&p1, &p2, &mut rng);
            assert_eq!(f1.len(), NUM_POSICOES);
            assert_eq!(f2.len(), NUM_POSICOES);
            // cada gene do filho vem de um dos pais na mesma posicao
            for pos in 0..NUM_POSICOES {
                assert!(f1[pos] == p1[pos] || f1[pos] == p2[pos]);
                assert!(f2[pos] == p1[pos] || f2[pos] == p2[pos]);
            }
        }
    }

    #[test]
    fn mutacao_altera_no_maximo_uma_posicao_e_mantem_validade() {
        let mut rng = rand::thread_rng();
        for _ in 0..2000 {
            let original = cromossomo::cromossomo_aleatorio(&mut rng);
            let mut mutado = original.clone();
            mutar(&mut mutado, &mut rng);
            let diferencas = (0..NUM_POSICOES)
                .filter(|&p| original[p] != mutado[p])
                .count();
            assert!(diferencas <= 1);
            // continua parseavel/valido apos mutacao
            let _ = cromossomo::neuronios(&mutado);
            let _ = cromossomo::taxa(&mutado);
        }
    }

    #[test]
    fn roleta_favorece_menor_mse() {
        let mut rng = rand::thread_rng();
        let populacao = vec![
            Individuo {
                cromossomo: cromo(["2", "2", "0.010000", "20", "1", "1"]),
                fitness: 0.01, // melhor
            },
            Individuo {
                cromossomo: cromo(["3", "3", "0.010000", "20", "1", "1"]),
                fitness: 100.0, // pior
            },
        ];
        let mut melhor = 0;
        for _ in 0..2000 {
            if selecao_roleta(&populacao, &mut rng).fitness < 1.0 {
                melhor += 1;
            }
        }

        assert!(
            melhor > 1500,
            "roleta nao favoreceu o melhor: {melhor}/2000"
        );
    }
}
