pub struct EpocaInfo {
    pub epoca: usize,
    pub erros: usize,
}

struct Adaline {
    pesos: Vec<f64>,
    bias: f64,
}

impl Adaline {
    fn new(n: usize) -> Self {
        Self { pesos: vec![0.0; n], bias: 0.0 }
    }

    fn net(&self, x: &[f64]) -> f64 {
        let mut net = self.bias;
        for (w, &xi) in self.pesos.iter().zip(x.iter()) {
            net += w * xi;
        }
        net
    }

    /// Regra LMS (Widrow-Hoff): erro baseado no net contínuo, não na saída binária
    fn atualizar(&mut self, x: &[f64], alvo: f64, eta: f64) {
        let erro = alvo - self.net(x);
        for (w, &xi) in self.pesos.iter_mut().zip(x.iter()) {
            *w += eta * erro * xi;
        }
        self.bias += eta * erro;
    }
}

pub struct Madaline {
    adalines: Vec<Adaline>,
    eta: f64,
    pub max_epocas: usize,
}

impl Madaline {
    pub fn new(n_entradas: usize, n_classes: usize, eta: f64, max_epocas: usize) -> Self {
        Self {
            adalines: (0..n_classes).map(|_| Adaline::new(n_entradas)).collect(),
            eta,
            max_epocas,
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
        let mut historico = Vec::new();

        for epoca in 1..=self.max_epocas {
            let mut erros = 0;

            for (x, classe) in amostras {
                let (pred, _) = self.prever(x);
                if pred != *classe {
                    erros += 1;
                }
                for (j, adaline) in self.adalines.iter_mut().enumerate() {
                    let alvo = if j == *classe { 1.0 } else { -1.0 };
                    adaline.atualizar(x, alvo, self.eta);
                }
            }

            historico.push(EpocaInfo { epoca, erros });
            if erros == 0 {
                break;
            }
        }

        historico
    }
}
