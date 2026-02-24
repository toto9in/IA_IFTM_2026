struct ConfigPerceptron {
    num_entradas: usize,
    taxa_aprendizagem: f64,
    max_epocas: usize,
    bias_inicial: f64,
}

fn flatten(mat: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut v = Vec::new();
    for linha in mat {
        for &pixel in linha {
            v.push(pixel);
        }
    }
    v
}

fn imprimir_letra(label: &str, mat: &Vec<Vec<f64>>) {
    println!("Letra {}:", label);
    for linha in mat {
        for &pixel in linha {
            if pixel == 1.0 {
                print!("█ ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

struct Perceptron {
    pesos: Vec<f64>,
    bias: f64,
    config: ConfigPerceptron,
}

impl Perceptron {
    fn new(config: ConfigPerceptron) -> Self {
        let pesos = vec![0.0; config.num_entradas];
        Self {
            pesos,
            bias: config.bias_inicial,
            config,
        }
    }

    /// funcao degrau bipolar
    fn ativar(net: f64) -> f64 {
        if net >= 0.0 { 1.0 } else { -1.0 }
    }

    fn prever(&self, x: &Vec<f64>) -> f64 {
        let mut net = self.bias;
        for i in 0..self.pesos.len() {
            net += self.pesos[i] * x[i];
        }
        Self::ativar(net)
    }

    fn treinar(&mut self, amostras: &Vec<(Vec<f64>, f64)>) -> usize {
        for epoca in 1..=self.config.max_epocas {
            let mut erros = 0;

            for (amostra_idx, (x, alvo)) in amostras.iter().enumerate() {
                let saida = self.prever(x);

                if saida != *alvo {
                    erros += 1;
                    let delta = self.config.taxa_aprendizagem * (alvo - saida);

                    for i in 0..self.pesos.len() {
                        self.pesos[i] += delta * x[i];
                    }
                    self.bias += delta;
                }

                println!(
                    "    Amostra {:>2} | bias: {:+.4} | pesos (7x7):",
                    amostra_idx, self.bias
                );
                for linha in self.pesos.chunks(7) {
                    let linha_str: Vec<String> =
                        linha.iter().map(|p| format!("{:+.4}", p)).collect();
                    println!("      {}", linha_str.join("  "));
                }
                println!();
            }

            println!("  Época {:>4} | erros: {}", epoca, erros);

            if erros == 0 {
                return epoca;
            }
        }
        self.config.max_epocas
    }
}

fn main() {
    let letra_a: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        vec![0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ];

    let letra_b: Vec<Vec<f64>> = vec![
        vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    ];

    imprimir_letra("A", &letra_a);
    imprimir_letra("B", &letra_b);

    let amostras: Vec<(Vec<f64>, f64)> = vec![
        (flatten(&letra_a), -1.0), // A → -1
        (flatten(&letra_b), 1.0),  // B → +1
    ];

    let config = ConfigPerceptron {
        num_entradas: 7 * 7,
        taxa_aprendizagem: 0.01,
        max_epocas: 100,
        bias_inicial: 0.3256,
    };

    let mut p = Perceptron::new(config);

    println!("=== Treinamento ===");
    let epocas = p.treinar(&amostras);
    println!("\nConvergiu em {} época(s).\n", epocas);

    println!("=== Resultados ===");
    for (x, alvo) in &amostras {
        let saida = p.prever(x);
        let letra = if *alvo < 0.0 { "A" } else { "B" };
        let status = if saida == *alvo { "✓" } else { "✗" };
        println!(
            "  Letra {} | alvo: {:+.0} | saída: {:+.0} | {}",
            letra, alvo, saida, status
        );
    }
}
