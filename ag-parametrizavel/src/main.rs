use plotters::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};

const X_MAX: f64 = 512.0;

enum MetodoSelecao {
    Roleta,
    Torneio(usize),
}

enum MetodoCruzamento {
    UmPonto,
    DoisPontos,
}

struct Config {
    bits: u32,
    pop_size: usize,
    perc_cruzamento: f64,
    prob_mutacao: f64,
    max_geracoes: usize,
    selecao: MetodoSelecao,
    cruzamento: MetodoCruzamento,
}

fn f(x: f64) -> f64 {
    -(x * x.abs().sqrt().sin()).abs()
}

fn fitness(fx: f64) -> f64 {
    -fx
}

fn decodificar(crom: u32, bits: u32) -> f64 {
    let max_crom = (1u32 << bits) - 1;
    crom as f64 * X_MAX / max_crom as f64
}

fn ler_linha(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn ler_u32(prompt: &str, min: u32, max: u32) -> u32 {
    loop {
        match ler_linha(prompt).parse::<u32>() {
            Ok(v) if v >= min && v <= max => return v,
            _ => println!("  Inválido. Digite um inteiro entre {} e {}.", min, max),
        }
    }
}

fn ler_usize(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        match ler_linha(prompt).parse::<usize>() {
            Ok(v) if v >= min && v <= max => return v,
            _ => println!("  Inválido. Digite um inteiro entre {} e {}.", min, max),
        }
    }
}

fn ler_f64(prompt: &str, min: f64, max: f64) -> f64 {
    loop {
        match ler_linha(prompt).parse::<f64>() {
            Ok(v) if v >= min && v <= max => return v,
            _ => println!("  Inválido. Digite um número de {:.2} a {:.2}.", min, max),
        }
    }
}

fn ler_config() -> Config {
    println!("\n=== Configuração do Algoritmo Genético ===\n");

    let bits = ler_u32("Tamanho do cromossomo (bits) [2-31]: ", 2, 31);
    let pop_size = ler_usize("Tamanho da população [2-10000]: ", 2, 10000);
    let perc_cruzamento = ler_f64(
        "Porcentagem da população para cruzamento (ex: 0.8 = 80%) [0.01-1.0]: ",
        0.01,
        1.0,
    );
    let prob_mutacao = ler_f64(
        "Probabilidade de mutação (ex: 0.01 = 1%) [0.0-1.0]: ",
        0.0,
        1.0,
    );
    let max_geracoes = ler_usize("Quantidade máxima de gerações [1-10000]: ", 1, 10000);

    println!("\nMétodo de seleção:");
    println!("  1 - Roleta");
    println!("  2 - Torneio");
    let selecao = match ler_u32("Escolha [1-2]: ", 1, 2) {
        2 => {
            let tamanho = ler_usize(
                &format!("Tamanho do torneio [2-{}]: ", pop_size),
                2,
                pop_size,
            );
            MetodoSelecao::Torneio(tamanho)
        }
        _ => MetodoSelecao::Roleta,
    };

    println!("\nMétodo de cruzamento:");
    println!("  1 - Um ponto");
    println!("  2 - Dois pontos");
    let cruzamento = match ler_u32("Escolha [1-2]: ", 1, 2) {
        2 => MetodoCruzamento::DoisPontos,
        _ => MetodoCruzamento::UmPonto,
    };

    Config {
        bits,
        pop_size,
        perc_cruzamento,
        prob_mutacao,
        max_geracoes,
        selecao,
        cruzamento,
    }
}

fn selecionar_roleta(imagem: &[f64], n: usize, rng: &mut impl Rng) -> Vec<usize> {
    let fits: Vec<f64> = imagem.iter().map(|&fx| fitness(fx)).collect();
    let mut disponiveis: Vec<usize> = (0..imagem.len()).collect();
    let mut selecionados = Vec::with_capacity(n);

    for _ in 0..n {
        if disponiveis.is_empty() {
            break;
        }
        let soma: f64 = disponiveis.iter().map(|&i| fits[i]).sum();
        let escolhido_pos = if soma <= f64::EPSILON {
            rng.gen_range(0..disponiveis.len())
        } else {
            let r = rng.gen_range(0.0..soma);
            let mut acumulado = 0.0;
            let mut pos_final = 0;
            for (pos, &i) in disponiveis.iter().enumerate() {
                acumulado += fits[i];
                if acumulado >= r {
                    pos_final = pos;
                    break;
                }
            }
            pos_final
        };
        let idx = disponiveis.remove(escolhido_pos);
        selecionados.push(idx);
    }

    selecionados
}

fn selecionar_torneio(imagem: &[f64], n: usize, tamanho: usize, rng: &mut impl Rng) -> Vec<usize> {
    let fits: Vec<f64> = imagem.iter().map(|&fx| fitness(fx)).collect();
    let mut disponiveis: Vec<usize> = (0..imagem.len()).collect();
    let mut selecionados = Vec::with_capacity(n);

    for _ in 0..n {
        if disponiveis.is_empty() {
            break;
        }
        let k = tamanho.min(disponiveis.len());
        let candidatos: Vec<usize> = disponiveis.choose_multiple(rng, k).cloned().collect();
        let vencedor = *candidatos
            .iter()
            .max_by(|&&a, &&b| fits[a].partial_cmp(&fits[b]).unwrap())
            .unwrap();
        let pos = disponiveis.iter().position(|&x| x == vencedor).unwrap();
        disponiveis.remove(pos);
        selecionados.push(vencedor);
    }

    selecionados
}

fn cruzar_um_ponto(pai1: u32, pai2: u32, bits: u32, rng: &mut impl Rng) -> (u32, u32) {
    let ponto = rng.gen_range(1..bits);
    let mask = (1u32 << ponto) - 1;
    ((pai1 & !mask) | (pai2 & mask), (pai2 & !mask) | (pai1 & mask))
}

fn cruzar_dois_pontos(pai1: u32, pai2: u32, bits: u32, rng: &mut impl Rng) -> (u32, u32) {
    if bits < 3 {
        return cruzar_um_ponto(pai1, pai2, bits, rng);
    }
    let p1 = rng.gen_range(1..bits - 1);
    let p2 = rng.gen_range(p1 + 1..bits);
    let mask = ((1u32 << p2) - 1) ^ ((1u32 << p1) - 1);
    ((pai1 & !mask) | (pai2 & mask), (pai2 & !mask) | (pai1 & mask))
}

fn mutar(crom: u32, bits: u32, prob: f64, rng: &mut impl Rng) -> u32 {
    if rng.r#gen::<f64>() < prob {
        let bit = rng.gen_range(0..bits);
        crom ^ (1u32 << bit)
    } else {
        crom
    }
}

fn plotar_convergencia(historico: &[(usize, f64)], max_geracoes: usize) {
    let root = BitMapBackend::new("convergencia.png", (900, 500)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let y_min = historico
        .iter()
        .map(|&(_, fx)| fx)
        .fold(f64::INFINITY, f64::min);
    let y_max = historico
        .iter()
        .map(|&(_, fx)| fx)
        .fold(f64::NEG_INFINITY, f64::max);
    let margem = (y_max - y_min).abs() * 0.1 + 1.0;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Convergência do AG — melhor f(x) por geração",
            ("sans-serif", 20),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(1usize..max_geracoes + 1, (y_min - margem)..(y_max + margem))
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Geração")
        .y_desc("f(x)")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            historico.iter().map(|&(g, fx)| (g, fx)),
            &BLUE,
        ))
        .unwrap()
        .label("melhor f(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(
            historico
                .iter()
                .map(|&(g, fx)| Circle::new((g, fx), 3, BLUE.filled())),
        )
        .unwrap();

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()
        .unwrap();

    println!("Gráfico salvo: convergencia.png");
}

fn plotar_funcao(melhor_x: f64, melhor_fx: f64) {
    let root = BitMapBackend::new("funcao.png", (900, 500)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let pontos: Vec<(f64, f64)> = (0..=1023)
        .map(|i| {
            let x = i as f64 * X_MAX / 1023.0;
            (x, f(x))
        })
        .collect();

    let y_min = pontos
        .iter()
        .map(|&(_, y)| y)
        .fold(f64::INFINITY, f64::min);
    let y_max = pontos
        .iter()
        .map(|&(_, y)| y)
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "f(x) = -|x·sin(√|x|)| com mínimo encontrado pelo AG",
            ("sans-serif", 18),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..X_MAX, (y_min - 10.0)..(y_max + 10.0))
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("x")
        .y_desc("f(x)")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(pontos, &BLUE))
        .unwrap()
        .label("f(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(std::iter::once(Circle::new(
            (melhor_x, melhor_fx),
            8,
            RED.filled(),
        )))
        .unwrap()
        .label(format!("x*={:.2}, f(x*)={:.2}", melhor_x, melhor_fx))
        .legend(|(x, y)| Circle::new((x + 10, y), 5, RED.filled()));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()
        .unwrap();

    println!("Gráfico salvo: funcao.png");
}

fn main() {
    let config = ler_config();

    let max_crom = (1u32 << config.bits) - 1;

    // n_pais: even, at least 2, at most pop_size (rounded down to even)
    let n_pais = {
        let n = (config.perc_cruzamento * config.pop_size as f64) as usize;
        (n / 2 * 2).max(2).min(config.pop_size / 2 * 2)
    };

    let nome_selecao = match &config.selecao {
        MetodoSelecao::Roleta => "Roleta".to_string(),
        MetodoSelecao::Torneio(t) => format!("Torneio(k={})", t),
    };
    let nome_cruzamento = match &config.cruzamento {
        MetodoCruzamento::UmPonto => "1 ponto",
        MetodoCruzamento::DoisPontos => "2 pontos",
    };

    println!("\nAlgoritmo Genético — Minimização de f(x) = -|x·sin(√|x|)|");
    println!(
        "Bits: {} | Pop: {} | Pais/ger: {} | P_mut: {:.4} | Gerações: {} | Seleção: {} | Cruzamento: {}",
        config.bits,
        config.pop_size,
        n_pais,
        config.prob_mutacao,
        config.max_geracoes,
        nome_selecao,
        nome_cruzamento,
    );
    println!("{:-<80}", "");

    let mut rng = rand::thread_rng();
    let mut pop: Vec<u32> = (0..config.pop_size)
        .map(|_| rng.gen_range(0..=max_crom))
        .collect();

    let mut melhor_x = 0.0f64;
    let mut melhor_fx = f64::INFINITY;
    let mut historico: Vec<(usize, f64)> = Vec::new();

    for geracao in 0..config.max_geracoes {
        let imagem: Vec<f64> = pop
            .iter()
            .map(|&c| f(decodificar(c, config.bits)))
            .collect();

        let indices_pais = match &config.selecao {
            MetodoSelecao::Roleta => selecionar_roleta(&imagem, n_pais, &mut rng),
            MetodoSelecao::Torneio(t) => selecionar_torneio(&imagem, n_pais, *t, &mut rng),
        };

        let mut filhos: Vec<u32> = Vec::with_capacity(indices_pais.len());
        for chunk in indices_pais.chunks(2) {
            if chunk.len() < 2 {
                break;
            }
            let pai1 = pop[chunk[0]];
            let pai2 = pop[chunk[1]];
            let (f1, f2) = match &config.cruzamento {
                MetodoCruzamento::UmPonto => cruzar_um_ponto(pai1, pai2, config.bits, &mut rng),
                MetodoCruzamento::DoisPontos => {
                    cruzar_dois_pontos(pai1, pai2, config.bits, &mut rng)
                }
            };
            filhos.push(mutar(f1, config.bits, config.prob_mutacao, &mut rng));
            filhos.push(mutar(f2, config.bits, config.prob_mutacao, &mut rng));
        }

        // Remove os pais (ordem decrescente para não deslocar índices)
        let mut sorted_indices = indices_pais;
        sorted_indices.sort_unstable_by(|a, b| b.cmp(a));
        for idx in &sorted_indices {
            pop.remove(*idx);
        }
        // Insere os filhos
        pop.extend(filhos);

        let (gen_x, gen_fx) = pop
            .iter()
            .map(|&c| {
                let x = decodificar(c, config.bits);
                (x, f(x))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        if gen_fx < melhor_fx {
            melhor_fx = gen_fx;
            melhor_x = gen_x;
        }

        historico.push((geracao + 1, melhor_fx));

        println!(
            "Gen {:>4} | x: {:>10.4} | f(x): {:>12.6} | melhor: {:>12.6}",
            geracao + 1,
            gen_x,
            gen_fx,
            melhor_fx
        );
    }

    println!("{:-<80}", "");
    println!(
        "Mínimo encontrado: x* = {:.6}, f(x*) = {:.6}",
        melhor_x, melhor_fx
    );

    plotar_convergencia(&historico, config.max_geracoes);
    plotar_funcao(melhor_x, melhor_fx);
}
