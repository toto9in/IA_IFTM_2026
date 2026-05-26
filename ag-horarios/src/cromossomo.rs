use rand::Rng;

pub const DIAS: usize = 3;
pub const SLOTS_POR_DIA: usize = 5;
pub const TURMAS: usize = 3;
pub const LINHAS: usize = DIAS * SLOTS_POR_DIA;
pub const NUM_PROFESSORES: usize = 5;

pub const NOMES_DIAS: [&str; DIAS] = ["Sábado", "Domingo", "Segunda"];
pub const NOMES_TURMAS: [&str; TURMAS] = ["Turma 01", "Turma 02", "Turma 03"];

// Horário de início de cada slot (índice 0 = slot 1)
pub const HORARIOS_SLOTS: [&str; SLOTS_POR_DIA] = ["07:00", "08:00", "09:00", "10:00", "11:00"];
pub const NOMES_PROFESSORES: [&str; NUM_PROFESSORES + 1] = [
    "Vazio",
    "Matemática",
    "Português",
    "História",
    "Ciências",
    "Inglês",
];

// Matriz LINHAS × TURMAS. Valor = número do professor (0 = vazio).
#[derive(Clone)]
pub struct Cromossomo {
    pub matriz: [[u8; TURMAS]; LINHAS],
    pub fitness: f64,
}

impl Cromossomo {
    pub fn novo_aleatorio<R: Rng>(rng: &mut R) -> Self {
        let mut matriz = [[0u8; TURMAS]; LINHAS];
        for linha in &mut matriz {
            for celula in linha.iter_mut() {
                *celula = rng.gen_range(1..=NUM_PROFESSORES as u8);
            }
        }
        let mut c = Cromossomo {
            matriz,
            fitness: 0.0,
        };
        c.fitness = c.calcular_fitness();
        c
    }

    pub fn calcular_fitness(&self) -> f64 {
        let mut fitness = 0.0;

        // Penalidade: choque de horário (professor no mesmo slot em turmas diferentes)
        for linha in 0..LINHAS {
            for p in 1..=NUM_PROFESSORES as u8 {
                let count = self.matriz[linha].iter().filter(|&&v| v == p).count();
                if count > 1 {
                    fitness -= 20.0 * (count - 1) as f64;
                }
            }
        }

        // Bônus: mesma matéria aparece 1 ou 2 vezes por dia por turma (ideal)
        // Penalidade: mais de 2 vezes por dia por turma (excesso)
        for turma in 0..TURMAS {
            for dia in 0..DIAS {
                let inicio = dia * SLOTS_POR_DIA;
                let fim = inicio + SLOTS_POR_DIA;

                for p in 1..=NUM_PROFESSORES as u8 {
                    let count = (inicio..fim)
                        .filter(|&slot| self.matriz[slot][turma] == p)
                        .count();

                    match count {
                        1 | 2 => fitness += 5.0 * count as f64,
                        c if c > 2 => fitness -= 5.0 * (c - 2) as f64,
                        _ => {}
                    }
                }
            }
        }

        // Bônus por célula preenchida (incentiva schedule completo)
        for linha in &self.matriz {
            for &celula in linha {
                if celula > 0 {
                    fitness += 1.0;
                }
            }
        }

        fitness
    }

    // Cruzamento por ponto de corte de linha
    pub fn cruzar<R: Rng>(&self, outro: &Cromossomo, rng: &mut R) -> (Cromossomo, Cromossomo) {
        let ponto = rng.gen_range(1..LINHAS);

        let mut filho1 = self.clone();
        let mut filho2 = outro.clone();

        for i in ponto..LINHAS {
            filho1.matriz[i] = outro.matriz[i];
            filho2.matriz[i] = self.matriz[i];
        }

        filho1.fitness = filho1.calcular_fitness();
        filho2.fitness = filho2.calcular_fitness();

        (filho1, filho2)
    }

    pub fn mutar<R: Rng>(&mut self, taxa: f64, rng: &mut R) {
        for linha in &mut self.matriz {
            for celula in linha.iter_mut() {
                if rng.r#gen::<f64>() < taxa {
                    *celula = rng.gen_range(1..=NUM_PROFESSORES as u8);
                }
            }
        }
        self.fitness = self.calcular_fitness();
    }
}
