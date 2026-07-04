use rand::Rng;

use crate::dataset::Dataset;

// ---------------------------------------------------------------------------
// Rede neural MLP feita a mao.
// - Camadas ocultas usam tangente hiperbolica (tanh).
// - Camada de saida e linear, para conseguir representar tanto dados
//   normalizados em [-1,1] quanto dados brutos (ex: 58..312).
// - Pesos iniciais aleatorios.
// ---------------------------------------------------------------------------

const PESO_INICIAL: f64 = 0.5;

/// Uma camada densa: pesos[neuronio][entrada] + bias[neuronio].
struct Camada {
    pesos: Vec<Vec<f64>>,
    bias: Vec<f64>,
}

impl Camada {
    fn aleatoria(entradas: usize, neuronios: usize, rng: &mut impl Rng) -> Self {
        let pesos = (0..neuronios)
            .map(|_| {
                (0..entradas)
                    .map(|_| rng.gen_range(-PESO_INICIAL..=PESO_INICIAL))
                    .collect()
            })
            .collect();
        let bias = (0..neuronios)
            .map(|_| rng.gen_range(-PESO_INICIAL..=PESO_INICIAL))
            .collect();
        Camada { pesos, bias }
    }
}

pub struct Mlp {
    camadas: Vec<Camada>,
}

fn tanh_derivada(saida_tanh: f64) -> f64 {
    // derivada de tanderivada de tanh em funcao da propria saida: 1 - tanh^2
    1.0 - saida_tanh * saida_tanh
}

impl Mlp {
    pub fn nova(
        num_entradas: usize,
        neuronios: usize,
        camadas_ocultas: usize,
        num_saidas: usize,
        rng: &mut impl Rng,
    ) -> Self {
        let mut camadas = Vec::with_capacity(camadas_ocultas + 1);
        let mut entrada_atual = num_entradas;

        for _ in 0..camadas_ocultas {
            camadas.push(Camada::aleatoria(entrada_atual, neuronios, rng));
            entrada_atual = neuronios;
        }

        camadas.push(Camada::aleatoria(entrada_atual, num_saidas, rng));

        Mlp { camadas }
    }

    fn ultima(&self) -> usize {
        self.camadas.len() - 1
    }

    fn propagar(&self, entrada: &[f64]) -> Vec<Vec<f64>> {
        let mut ativacoes = Vec::with_capacity(self.camadas.len() + 1);
        ativacoes.push(entrada.to_vec());

        for (indice, camada) in self.camadas.iter().enumerate() {
            let e_saida = indice == self.ultima();
            let anterior = ativacoes.last().unwrap();
            let saida: Vec<f64> = camada
                .pesos
                .iter()
                .zip(&camada.bias)
                .map(|(pesos_neuronio, &b)| {
                    let z: f64 = pesos_neuronio
                        .iter()
                        .zip(anterior)
                        .map(|(w, x)| w * x)
                        .sum::<f64>()
                        + b;
                    if e_saida {
                        z // saida linear
                    } else {
                        z.tanh() // oculta
                    }
                })
                .collect();
            ativacoes.push(saida);
        }
        ativacoes
    }

    pub fn prever(&self, entrada: &[f64]) -> Vec<f64> {
        self.propagar(entrada).pop().unwrap()
    }

    pub fn mse(&self, dados: &Dataset) -> f64 {
        let mut soma = 0.0;
        let mut n = 0usize;
        for (entrada, alvo) in dados.entradas.iter().zip(&dados.saidas) {
            let saida = self.prever(entrada);
            for (s, a) in saida.iter().zip(alvo) {
                let erro = s - a;
                soma += erro * erro;
                n += 1;
            }
        }
        soma / n as f64
    }

    /// Treina a rede por `epocas`. `online = true` atualiza os pesos apos cada
    /// padrao; `false` (off-line) acumula o gradiente de todos os padroes e
    /// atualiza uma vez por epoca.
    pub fn treinar(&mut self, dados: &Dataset, taxa: f64, epocas: usize, online: bool) {
        for _ in 0..epocas {
            if online {
                for (entrada, alvo) in dados.entradas.iter().zip(&dados.saidas) {
                    let grad = self.gradiente(entrada, alvo);
                    self.aplicar(&grad, taxa);
                }
            } else {
                let mut acumulado = self.gradiente_zerado();
                for (entrada, alvo) in dados.entradas.iter().zip(&dados.saidas) {
                    let grad = self.gradiente(entrada, alvo);
                    somar_gradiente(&mut acumulado, &grad);
                }
                let fator = taxa / dados.entradas.len() as f64;
                self.aplicar(&acumulado, fator);
            }
        }
    }

    fn gradiente(&self, entrada: &[f64], alvo: &[f64]) -> Gradiente {
        let ativacoes = self.propagar(entrada);
        let mut deltas: Vec<Vec<f64>> = vec![Vec::new(); self.camadas.len()];

        let idx_saida = self.ultima();
        let saida = &ativacoes[idx_saida + 1];
        deltas[idx_saida] = saida.iter().zip(alvo).map(|(s, a)| s - a).collect();

        for camada in (0..idx_saida).rev() {
            let saida_camada = &ativacoes[camada + 1];
            let camada_seguinte = &self.camadas[camada + 1];
            let delta_seguinte = &deltas[camada + 1];

            let delta: Vec<f64> = (0..saida_camada.len())
                .map(|j| {
                    let soma: f64 = camada_seguinte
                        .pesos
                        .iter()
                        .zip(delta_seguinte)
                        .map(|(pesos_neuronio, &d)| pesos_neuronio[j] * d)
                        .sum();
                    soma * tanh_derivada(saida_camada[j])
                })
                .collect();
            deltas[camada] = delta;
        }

        let mut grad_pesos = Vec::with_capacity(self.camadas.len());
        let mut grad_bias = Vec::with_capacity(self.camadas.len());
        for (indice, camada) in self.camadas.iter().enumerate() {
            let anterior = &ativacoes[indice];
            let delta = &deltas[indice];
            let gp: Vec<Vec<f64>> = delta
                .iter()
                .map(|&d| anterior.iter().map(|&x| d * x).collect())
                .collect();
            let gb: Vec<f64> = delta.clone();
            let _ = camada; // dimensoes ja vem dos deltas/ativacoes
            grad_pesos.push(gp);
            grad_bias.push(gb);
        }

        Gradiente {
            pesos: grad_pesos,
            bias: grad_bias,
        }
    }

    fn gradiente_zerado(&self) -> Gradiente {
        let pesos = self
            .camadas
            .iter()
            .map(|c| c.pesos.iter().map(|p| vec![0.0; p.len()]).collect())
            .collect();
        let bias = self
            .camadas
            .iter()
            .map(|c| vec![0.0; c.bias.len()])
            .collect();
        Gradiente { pesos, bias }
    }

    /// Atualiza pesos e bias: w -= fator * grad.
    fn aplicar(&mut self, grad: &Gradiente, fator: f64) {
        for (camada, (gp, gb)) in self
            .camadas
            .iter_mut()
            .zip(grad.pesos.iter().zip(&grad.bias))
        {
            for (pesos_neuronio, grad_neuronio) in camada.pesos.iter_mut().zip(gp) {
                for (w, g) in pesos_neuronio.iter_mut().zip(grad_neuronio) {
                    *w -= fator * g;
                }
            }
            for (b, g) in camada.bias.iter_mut().zip(gb) {
                *b -= fator * g;
            }
        }
    }
}

struct Gradiente {
    pesos: Vec<Vec<Vec<f64>>>,
    bias: Vec<Vec<f64>>,
}

fn somar_gradiente(acumulado: &mut Gradiente, novo: &Gradiente) {
    for (ap, np) in acumulado.pesos.iter_mut().zip(&novo.pesos) {
        for (an, nn) in ap.iter_mut().zip(np) {
            for (a, n) in an.iter_mut().zip(nn) {
                *a += n;
            }
        }
    }
    for (ab, nb) in acumulado.bias.iter_mut().zip(&novo.bias) {
        for (a, n) in ab.iter_mut().zip(nb) {
            *a += n;
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn dataset_linear() -> Dataset {
        // saida = copia da entrada (relacao aprendivel, ao contrario do enunciado)
        let entradas: Vec<Vec<f64>> = (0..20)
            .map(|i| vec![(i as f64) / 20.0, ((i % 5) as f64) / 5.0])
            .collect();
        let saidas = entradas.clone();
        Dataset { entradas, saidas }
    }

    #[test]
    fn tanh_derivada_correta() {
        // em 0, tanh(0)=0 -> derivada = 1
        assert!((tanh_derivada(0.0f64.tanh()) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn mse_zero_quando_saida_igual_alvo() {
        let mut rng = rand::thread_rng();
        let rede = Mlp::nova(2, 3, 1, 2, &mut rng);
        let dados = Dataset {
            entradas: vec![vec![0.1, 0.2]],
            saidas: vec![rede.prever(&[0.1, 0.2])],
        };
        assert!(rede.mse(&dados) < 1e-12);
    }

    #[test]
    fn dimensao_da_saida() {
        let mut rng = rand::thread_rng();
        let rede = Mlp::nova(15, 8, 3, 13, &mut rng);
        assert_eq!(rede.prever(&vec![0.0; 15]).len(), 13);
    }

    #[test]
    fn treino_reduz_mse_em_problema_linear() {
        let mut rng = rand::thread_rng();
        let dados = dataset_linear();
        let mut rede = Mlp::nova(2, 6, 2, 2, &mut rng);
        let antes = rede.mse(&dados);
        rede.treinar(&dados, 0.05, 500, true);
        let depois = rede.mse(&dados);
        assert!(
            depois < antes,
            "mse nao caiu: antes={antes} depois={depois}"
        );
    }
}
