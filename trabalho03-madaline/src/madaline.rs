pub struct EpocaInfo {
    pub epoca: usize,
    pub erro: f64,
}

use rand::Rng;

struct Adaline {
    pesos: Vec<f64>,
    bias: f64,
}

impl Adaline {
    fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let pesos = (0..n).map(|_| rng.gen_range(-0.1..0.1)).collect();
        let bias = rng.gen_range(-0.1..0.1);
        Self { pesos, bias }
    }

    fn degrau_bipolar(soma: f64) -> f64 {
        if soma >= 0.0 { 1.0 } else { -1.0 }
    }

    fn soma(&self, x: &[f64]) -> f64 {
        let mut soma = self.bias;
        for i in 0..self.pesos.len() {
            soma += self.pesos[i] * x[i];
        }
        soma
    }

    fn atualizar(&mut self, x: &[f64], alvo: f64, eta: f64) {
        let soma = self.soma(x);
        let y = Adaline::degrau_bipolar(soma);
        let delta = alvo - y;
        for i in 0..self.pesos.len() {
            self.pesos[i] += eta * delta * x[i];
        }
        self.bias += eta * delta;
    }
}

pub struct Madaline {
    adalines: Vec<Adaline>,
    eta: f64,
    pub max_epocas: usize,
    pub errotolerado: f64,
}

impl Madaline {
    pub fn new(
        n_entradas: usize,
        n_classes: usize,
        eta: f64,
        max_epocas: usize,
        errotolerado: f64,
    ) -> Self {
        Self {
            adalines: (0..n_classes).map(|_| Adaline::new(n_entradas)).collect(),
            eta,
            max_epocas,
            errotolerado,
        }
    }

    /// Retorna (índice da classe, soma do vencedor)
    pub fn prever(&self, x: &[f64]) -> (usize, f64) {
        let somas: Vec<f64> = self.adalines.iter().map(|a| a.soma(x)).collect();
        somas
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, &soma)| (i, soma))
            .unwrap_or((0, 0.0))
    }

    pub fn treinar(&mut self, amostras: &[(Vec<f64>, usize)]) -> Vec<EpocaInfo> {
        let n_classes = self.adalines.len();
        let mut historico = Vec::new();

        for epoca in 1..=self.max_epocas {
            let mut erro = 0.0f64;

            for (x, classe) in amostras {
                // Forward: soma de cada saída
                let somas: Vec<f64> = self.adalines.iter().map(|a| a.soma(x)).collect();

                // Binarizar (limiar = 0.0)
                let y: Vec<f64> = somas
                    .iter()
                    .map(|&soma| Adaline::degrau_bipolar(soma))
                    .collect();

                // EQM: 0.5 * (alvo - y)^2 para cada saída
                for j in 0..n_classes {
                    let alvo = if j == *classe { 1.0 } else { -1.0 };
                    erro += 0.5 * (alvo - y[j]).powi(2);
                }

                // Atualizar pesos com (target - y_binário)
                for (j, adaline) in self.adalines.iter_mut().enumerate() {
                    let alvo = if j == *classe { 1.0 } else { -1.0 };
                    adaline.atualizar(x, alvo, self.eta);
                }
            }

            historico.push(EpocaInfo { epoca, erro });
            if erro <= self.errotolerado {
                break;
            }
        }

        historico
    }
}
