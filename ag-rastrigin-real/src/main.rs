mod ag;
mod grafico;
mod rastrigin;

use std::io::{self, Write};

use ag::MetodoSelecao;

const ARQUIVO_GRAFICO: &str = "convergencia.png";

fn main() {
    let metodo = ler_metodo_selecao();

    println!("\nExecutando AG (selecao: {})...\n", nome_metodo(metodo));
    let resultado = ag::executar(metodo);

    println!(
        "{:>8} | {:>14} | {:>14}",
        "Geracao", "Melhor fitness", "Fitness medio"
    );
    println!("{}", "-".repeat(42));
    for registro in &resultado.historico {
        println!(
            "{:>8} | {:>14.6} | {:>14.6}",
            registro.geracao, registro.melhor_fitness, registro.fitness_medio
        );
    }

    let melhor = &resultado.melhor;
    println!("\n===== Melhor solucao encontrada =====");
    println!(
        "x = {:.6}, y = {:.6}, z = {:.6}",
        melhor.genes[0], melhor.genes[1], melhor.genes[2]
    );
    println!(
        "f(x, y, z) = {:.6}  (minimo global e 0 em (0, 0, 0))",
        melhor.fitness
    );

    match grafico::salvar_convergencia(&resultado.historico, ARQUIVO_GRAFICO) {
        Ok(()) => println!("\nGrafico salvo em '{}'.", ARQUIVO_GRAFICO),
        Err(e) => eprintln!("\nFalha ao salvar grafico: {}", e),
    }
}

fn ler_metodo_selecao() -> MetodoSelecao {
    loop {
        println!("Metodo de selecao de pais:");
        println!("  1 - Torneio");
        println!("  2 - Roleta");
        print!("Escolha (1 ou 2): ");
        io::stdout().flush().ok();

        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_err() {
            continue;
        }

        match entrada.trim() {
            "1" => return MetodoSelecao::Torneio,
            "2" => return MetodoSelecao::Roleta,
            _ => println!("Opcao invalida. Tente novamente.\n"),
        }
    }
}

fn nome_metodo(metodo: MetodoSelecao) -> &'static str {
    match metodo {
        MetodoSelecao::Torneio => "torneio",
        MetodoSelecao::Roleta => "roleta",
    }
}
