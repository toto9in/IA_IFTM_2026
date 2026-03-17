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

    fn net(&self, x: &[f64]) -> f64 {
        let mut net = self.bias;
        for (w, &xi) in self.pesos.iter().zip(x.iter()) {
            net += w * xi;
        }
        net
    }

    fn atualizar(&mut self, x: &[f64], alvo: f64, eta: f64) {
        let net = self.net(x);
        let y = if net >= 0.0 { 1.0 } else { -1.0 };
        let delta = alvo - y;
        for (w, &xi) in self.pesos.iter_mut().zip(x.iter()) {
            *w += eta * delta * xi;
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

    /// Retorna (índice da classe, net do vencedor)
    pub fn prever(&self, x: &[f64]) -> (usize, f64) {
        let nets: Vec<f64> = self.adalines.iter().map(|a| a.net(x)).collect();
        nets.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, &net)| (i, net))
            .unwrap_or((0, 0.0))
    }

    pub fn treinar(&mut self, amostras: &[(Vec<f64>, usize)]) -> Vec<EpocaInfo> {
        let n_classes = self.adalines.len();
        let mut historico = Vec::new();

        for epoca in 1..=self.max_epocas {
            let mut erro = 0.0f64;

            for (x, classe) in amostras {
                // Forward: net de cada saída
                let nets: Vec<f64> = self.adalines.iter().map(|a| a.net(x)).collect();

                // Binarizar (limiar = 0.0)
                let y: Vec<f64> = nets
                    .iter()
                    .map(|&net| if net >= 0.0 { 1.0 } else { -1.0 })
                    .collect();

                // MSE: 0.5 * (target - y)^2 para cada saída
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
