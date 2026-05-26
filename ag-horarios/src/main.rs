mod ag;
mod cromossomo;

use ag::Ag;
use cromossomo::{DIAS, HORARIOS_SLOTS, NOMES_DIAS, NOMES_PROFESSORES, NOMES_TURMAS, SLOTS_POR_DIA, TURMAS};
use rand::SeedableRng;
use rand::rngs::SmallRng;

// Parâmetros do Algoritmo Genético
const TAMANHO_POPULACAO: usize = 100; // número de horários candidatos por geração
const GERACOES: usize = 10000; // quantas gerações o AG vai evoluir
const TAXA_CRUZAMENTO: f64 = 0.85; // 85% de chance de dois pais cruzarem
const TAXA_MUTACAO: f64 = 0.1; // 5% de chance de mutação por célula

fn main() {
    // Gerador de números aleatórios inicializado com entropia do sistema
    let mut rng = SmallRng::from_entropy();

    println!("=== AG - Horário Escolar ===");
    println!(
        "Populacao: {} | Gerações: {} | Cruzamento: {:.0}% | Mutação: {:.0}%",
        TAMANHO_POPULACAO,
        GERACOES,
        TAXA_CRUZAMENTO * 100.0,
        TAXA_MUTACAO * 100.0,
    );
    println!();

    // Cria o AG com a população inicial aleatória
    let mut ag = Ag::novo(TAMANHO_POPULACAO, TAXA_CRUZAMENTO, TAXA_MUTACAO, &mut rng);

    // Loop principal: evolui a população geração por geração
    for geracao in 0..=GERACOES {
        if geracao > 0 {
            ag.evoluir(&mut rng);
        }

        // Imprime o progresso a cada 200 gerações e na última
        if geracao % 200 == 0 || geracao == GERACOES {
            let melhor = ag.melhor();
            println!("Geração {:5}: fitness = {:.1}", geracao, melhor.fitness);
        }
    }

    println!();
    imprimir_horario(ag.melhor());
    println!();
    println!("Fitness final: {:.1}", ag.melhor().fitness);
}

/// Imprime o melhor horário encontrado como uma tabela formatada no terminal.
///
/// Linhas = dias × slots. Colunas = turmas. Células = nome do professor.
fn imprimir_horario(c: &cromossomo::Cromossomo) {
    let col_dia = 10usize;
    let col_slot = 6usize;
    let col_prof = 14usize;
    let largura = col_dia + col_slot + 1 + TURMAS * (col_prof + 3);

    println!("=== MELHOR HORÁRIO ENCONTRADO ===");
    println!();

    // Cabeçalho com os nomes das turmas
    print!("{:<col_dia$} {:>col_slot$} |", "", "Horário");
    for turma in NOMES_TURMAS {
        print!(" {:<col_prof$} |", turma);
    }
    println!();
    println!("{}", "─".repeat(largura));

    // Linhas da tabela: cada combinação de dia e slot
    for dia in 0..DIAS {
        for slot in 0..SLOTS_POR_DIA {
            // Calcula o índice da linha na matriz (dia * 5 + slot)
            let linha = dia * SLOTS_POR_DIA + slot;

            // Só imprime o nome do dia na primeira linha do bloco
            let nome_dia = if slot == 0 { NOMES_DIAS[dia] } else { "" };

            // Exibe o horário real do slot (ex: 07:00, 08:00...)
            print!("{:<col_dia$} {:>col_slot$} |", nome_dia, HORARIOS_SLOTS[slot]);

            // Imprime o professor de cada turma nesse slot
            for turma in 0..TURMAS {
                let prof = c.matriz[linha][turma] as usize;
                print!(" {:<col_prof$} |", NOMES_PROFESSORES[prof]);
            }
            println!();
        }
        // Separador entre dias
        println!("{}", "─".repeat(largura));
    }
}
