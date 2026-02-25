pub struct ConfigPerceptron {
    pub num_entradas: usize,
    pub taxa_aprendizagem: f64,
    pub max_epocas: usize,
    pub bias_inicial: f64,
}

pub struct IteracaoTreino {
    pub epoca: usize,
    pub pesos: Vec<f64>,
    pub bias: f64,
    pub erros: usize,
}

pub struct Perceptron {
    pub pesos: Vec<f64>,
    pub bias: f64,
    pub config: ConfigPerceptron,
}

impl Perceptron {
    pub fn new(config: ConfigPerceptron) -> Self {
        let pesos = vec![0.0; config.num_entradas];
        Self {
            pesos,
            bias: config.bias_inicial,
            config,
        }
    }

    /// Função degrau bipolar
    fn ativar(net: f64) -> f64 {
        if net >= 0.0 { 1.0 } else { -1.0 }
    }

    pub fn prever(&self, x: &Vec<f64>) -> f64 {
        let mut net = self.bias;
        for i in 0..self.pesos.len() {
            net += self.pesos[i] * x[i];
        }
        Self::ativar(net)
    }

    /// Treina o perceptron com a regra de Rosenblatt.
    /// Retorna o histórico de iterações (uma por época).
    pub fn treinar(&mut self, amostras: &Vec<(Vec<f64>, f64)>) -> Vec<IteracaoTreino> {
        let mut historico = Vec::new();

        for epoca in 1..=self.config.max_epocas {
            let mut erros = 0;

            for (x, alvo) in amostras {
                let saida = self.prever(x);

                if saida != *alvo {
                    erros += 1;
                    let delta = self.config.taxa_aprendizagem * (alvo - saida);

                    for i in 0..self.pesos.len() {
                        self.pesos[i] += delta * x[i];
                    }
                    self.bias += delta;
                }
            }

            historico.push(IteracaoTreino {
                epoca,
                pesos: self.pesos.clone(),
                bias: self.bias,
                erros,
            });

            if erros == 0 {
                break;
            }
        }

        historico
    }
}
