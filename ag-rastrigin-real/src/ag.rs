use rand::Rng;

use crate::rastrigin::{self, LIMITE_INFERIOR, LIMITE_SUPERIOR, NUM_GENES};

// ---------------------------------------------------------------------------
// Parametros do AG (edite aqui).
// ---------------------------------------------------------------------------
pub const TAMANHO_POPULACAO: usize = 100;
pub const MAX_GERACOES: usize = 200;
pub const TAXA_CRUZAMENTO: f64 = 0.9;
pub const TAXA_MUTACAO: f64 = 0.05;
pub const TAMANHO_TORNEIO: usize = 3;

#[derive(Clone, Copy, PartialEq)]
pub enum MetodoSelecao {
    Torneio,
    Roleta,
}

#[derive(Clone)]
pub struct Individuo {
    pub genes: Vec<f64>,
    pub fitness: f64,
}

impl Individuo {
    fn aleatorio(rng: &mut impl Rng) -> Self {
        let genes: Vec<f64> = (0..NUM_GENES).map(|_| gene_aleatorio(rng)).collect();
        let fitness = rastrigin::avaliar(&genes);
        Individuo { genes, fitness }
    }

    fn reavaliar(&mut self) {
        self.fitness = rastrigin::avaliar(&self.genes);
    }
}

fn gene_aleatorio(rng: &mut impl Rng) -> f64 {
    let c: f64 = rng.gen();
    LIMITE_INFERIOR + c * (LIMITE_SUPERIOR - LIMITE_INFERIOR)
}

pub struct RegistroGeracao {
    pub geracao: usize,
    pub melhor_fitness: f64,
    pub fitness_medio: f64,
}

pub struct ResultadoAG {
    pub melhor: Individuo,
    pub historico: Vec<RegistroGeracao>,
}

pub fn executar(metodo: MetodoSelecao) -> ResultadoAG {
    let mut rng = rand::thread_rng();

    let mut populacao: Vec<Individuo> = (0..TAMANHO_POPULACAO)
        .map(|_| Individuo::aleatorio(&mut rng))
        .collect();

    let mut historico = Vec::with_capacity(MAX_GERACOES);
    let mut melhor_global = melhor_da_populacao(&populacao).clone();

    for geracao in 0..MAX_GERACOES {
        let elite = melhor_da_populacao(&populacao).clone();
        let mut nova_populacao: Vec<Individuo> = Vec::with_capacity(TAMANHO_POPULACAO);
        nova_populacao.push(elite);

        while nova_populacao.len() < TAMANHO_POPULACAO {
            let pai1 = selecionar(&populacao, metodo, &mut rng);
            let pai2 = selecionar(&populacao, metodo, &mut rng);

            let (mut filho1, mut filho2) = if rng.gen::<f64>() < TAXA_CRUZAMENTO {
                cruzar_radcliff(pai1, pai2, &mut rng)
            } else {
                (pai1.clone(), pai2.clone())
            };

            mutar(&mut filho1, &mut rng);
            mutar(&mut filho2, &mut rng);

            filho1.reavaliar();
            filho2.reavaliar();

            nova_populacao.push(filho1);
            if nova_populacao.len() < TAMANHO_POPULACAO {
                nova_populacao.push(filho2);
            }
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
            melhor_fitness: melhor_global.fitness,
            fitness_medio: media,
        });
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

fn selecionar<'a>(
    populacao: &'a [Individuo],
    metodo: MetodoSelecao,
    rng: &mut impl Rng,
) -> &'a Individuo {
    match metodo {
        MetodoSelecao::Torneio => selecao_torneio(populacao, rng),
        MetodoSelecao::Roleta => selecao_roleta(populacao, rng),
    }
}

fn selecao_torneio<'a>(populacao: &'a [Individuo], rng: &mut impl Rng) -> &'a Individuo {
    let mut melhor = &populacao[rng.gen_range(0..populacao.len())];
    for _ in 1..TAMANHO_TORNEIO {
        let candidato = &populacao[rng.gen_range(0..populacao.len())];
        if candidato.fitness < melhor.fitness {
            melhor = candidato;
        }
    }
    melhor
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

/// cruzamento de Radcliff (combinacao linear), gene a gene:
///   filho1 = beta * pai1 + (1 - beta) * pai2
///   filho2 = (1 - beta) * pai1 + beta * pai2
/// com beta aleatorio em [0, 1] sorteado para cada gene.
fn cruzar_radcliff(
    pai1: &Individuo,
    pai2: &Individuo,
    rng: &mut impl Rng,
) -> (Individuo, Individuo) {
    let mut genes1 = Vec::with_capacity(NUM_GENES);
    let mut genes2 = Vec::with_capacity(NUM_GENES);

    for i in 0..NUM_GENES {
        let beta: f64 = rng.gen();
        let g1 = beta * pai1.genes[i] + (1.0 - beta) * pai2.genes[i];
        let g2 = (1.0 - beta) * pai1.genes[i] + beta * pai2.genes[i];
        genes1.push(g1);
        genes2.push(g2);
    }

    let filho1 = Individuo {
        genes: genes1,
        fitness: f64::INFINITY,
    };
    let filho2 = Individuo {
        genes: genes2,
        fitness: f64::INFINITY,
    };
    (filho1, filho2)
}

fn mutar(individuo: &mut Individuo, rng: &mut impl Rng) {
    if rng.gen::<f64>() < TAXA_MUTACAO {
        let indice = rng.gen_range(0..NUM_GENES);
        individuo.genes[indice] = gene_aleatorio(rng);
    }
}
