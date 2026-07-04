mod ag;
mod cromossomo;
mod dataset;
mod mlp;

fn main() {
    println!("=== AG para otimizacao de arquitetura de RNA (MLP) ===\n");

    let dados = dataset::gerar();
    println!(
        "Dataset: {} padroes | {} entradas [{}..{}] | {} saidas [{}..{}]\n",
        dataset::NUM_PADROES,
        dataset::NUM_ENTRADAS,
        dataset::ENTRADA_MIN,
        dataset::ENTRADA_MAX,
        dataset::NUM_SAIDAS,
        dataset::SAIDA_MIN,
        dataset::SAIDA_MAX,
    );

    println!(
        "Rodando AG: populacao {}, ate {} geracoes...\n",
        ag::TAMANHO_POPULACAO,
        ag::MAX_GERACOES
    );

    let resultado = ag::executar(&dados);

    for registro in resultado.historico.iter().step_by(10) {
        println!(
            "geracao {:>3} | melhor MSE = {:>14.6} | MSE medio = {:>14.6}",
            registro.geracao, registro.melhor_mse, registro.mse_medio
        );
    }

    // -----------------------------------------------------------------------
    // Melhor arquitetura encontrada pelo AG.
    // -----------------------------------------------------------------------
    let melhor = &resultado.melhor;
    println!("\n================ MELHOR ARQUITETURA ================");
    println!("Cromossomo vencedor (String): {:?}", melhor.cromossomo);
    println!("{}", cromossomo::descrever(&melhor.cromossomo));
    println!("\nMSE final: {:.6}", melhor.fitness);
    println!("====================================================");
}
