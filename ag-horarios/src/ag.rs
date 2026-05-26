use rand::Rng;
use crate::cromossomo::Cromossomo;

/// Controla toda a lógica do Algoritmo Genético:
/// população, seleção, cruzamento, mutação e evolução por gerações.
pub struct Ag {
    pub populacao: Vec<Cromossomo>, // conjunto de soluções candidatas
    tamanho: usize,                 // quantos indivíduos na população
    taxa_cruzamento: f64,           // probabilidade de dois pais cruzarem (ex: 0.85 = 85%)
    taxa_mutacao: f64,              // probabilidade de mutação por célula (ex: 0.05 = 5%)
}

impl Ag {
    /// Cria o AG com uma população inicial de indivíduos aleatórios.
    /// Todos os cromossomos são gerados com horários sorteados
    /// e já têm seu fitness calculado.
    pub fn novo<R: Rng>(
        tamanho: usize,
        taxa_cruzamento: f64,
        taxa_mutacao: f64,
        rng: &mut R,
    ) -> Self {
        let populacao = (0..tamanho).map(|_| Cromossomo::novo_aleatorio(rng)).collect();
        Ag { populacao, tamanho, taxa_cruzamento, taxa_mutacao }
    }

    /// Retorna referência ao melhor indivíduo da população atual.
    /// "Melhor" = maior valor de fitness.
    pub fn melhor(&self) -> &Cromossomo {
        self.populacao
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap()
    }

    /// Seleciona um indivíduo usando torneio com k=3.
    ///
    /// Sorteia 3 índices aleatórios da população e retorna o com maior fitness.
    /// O torneio equilibra pressão seletiva: bons indivíduos têm mais chance
    /// de vencer, mas não é garantido — mantendo diversidade.
    fn selecionar<R: Rng>(&self, rng: &mut R) -> &Cromossomo {
        let mut melhor_idx = rng.gen_range(0..self.tamanho);
        for _ in 1..3 {
            let idx = rng.gen_range(0..self.tamanho);
            if self.populacao[idx].fitness > self.populacao[melhor_idx].fitness {
                melhor_idx = idx;
            }
        }
        &self.populacao[melhor_idx]
    }

    /// Evolui a população por uma geração.
    ///
    /// Passos:
    ///   1. Elitismo: o melhor indivíduo passa direto para a próxima geração,
    ///      garantindo que a melhor solução nunca seja perdida.
    ///   2. Seleção: dois pais são escolhidos por torneio.
    ///   3. Cruzamento: com probabilidade `taxa_cruzamento`, os pais geram dois filhos
    ///      por cruzamento de ponto de linha. Caso contrário, os pais passam sem alteração.
    ///   4. Mutação: cada filho sofre mutação aleatória célula a célula.
    ///   5. A nova população substitui completamente a antiga.
    pub fn evoluir<R: Rng>(&mut self, rng: &mut R) {
        let mut nova = Vec::with_capacity(self.tamanho);

        // Elitismo: copia o melhor da geração atual sem modificação
        nova.push(self.melhor().clone());

        // Preenche o restante da nova população com filhos gerados
        while nova.len() < self.tamanho {
            let pai1 = self.selecionar(rng).clone();
            let pai2 = self.selecionar(rng).clone();

            // Decide se aplica cruzamento ou passa os pais diretamente
            let (mut filho1, mut filho2) = if rng.r#gen::<f64>() < self.taxa_cruzamento {
                pai1.cruzar(&pai2, rng)
            } else {
                (pai1, pai2)
            };

            // Aplica mutação em ambos os filhos
            filho1.mutar(self.taxa_mutacao, rng);
            filho2.mutar(self.taxa_mutacao, rng);

            nova.push(filho1);
            if nova.len() < self.tamanho {
                nova.push(filho2);
            }
        }

        // Substitui a população antiga pela nova geração
        self.populacao = nova;
    }
}
