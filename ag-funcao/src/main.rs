use plotters::prelude::*;
use rand::Rng;

const POP_SIZE: usize = 100;
const NUM_GERACOES: usize = 50;
const BITS: u32 = 10;
const MAX_CROM: u16 = (1 << BITS) - 1; // 1023
const X_MAX: f64 = 512.0;
const PROB_CRUZAMENTO: f64 = 0.8;
const PROB_MUTACAO: f64 = 0.01;
const NUM_MELHORES: usize = 20;

fn decodificar(crom: u16) -> f64 {
    crom as f64 * X_MAX / MAX_CROM as f64
}

fn f(x: f64) -> f64 {
    -(x * x.abs().sqrt().sin()).abs()
}

fn fitness(fx: f64) -> f64 {
    -fx
}

fn gerar_populacao(rng: &mut impl Rng) -> Vec<u16> {
    (0..POP_SIZE).map(|_| rng.gen_range(0..=MAX_CROM)).collect()
}

fn gerar_imagem(pop_dec: &[f64]) -> Vec<f64> {
    pop_dec.iter().map(|&x| f(x)).collect()
}

fn gerar_probabilidades(imagem: &[f64]) -> Vec<f64> {
    let fits: Vec<f64> = imagem.iter().map(|&fx| fitness(fx)).collect();
    let soma: f64 = fits.iter().sum();
    fits.iter().map(|&fi| fi / soma).collect()
}

fn separar_vinte_melhores(probab_rolet: &[f64], pop_bin: &[u16], rng: &mut impl Rng) -> Vec<u16> {
    let mut disponiveis: Vec<usize> = (0..pop_bin.len()).collect();
    let mut selecionados = Vec::with_capacity(NUM_MELHORES);

    for _ in 0..NUM_MELHORES {
        if disponiveis.is_empty() {
            break;
        }

        let soma: f64 = disponiveis.iter().map(|&i| probab_rolet[i]).sum();
        let r: f64 = rng.gen_range(0.0..1.0_f64) * soma;

        let mut acumulado = 0.0;
        let mut escolhido_idx = 0;
        for (pos, &i) in disponiveis.iter().enumerate() {
            acumulado += probab_rolet[i];
            if acumulado >= r {
                escolhido_idx = pos;
                break;
            }
        }

        let idx = disponiveis.remove(escolhido_idx);
        selecionados.push(pop_bin[idx]);
    }

    selecionados
}

fn sortear_casais(s_melhores: &[u16], rng: &mut impl Rng) -> Vec<(u16, u16)> {
    let mut embaralhados = s_melhores.to_vec();
    for i in (1..embaralhados.len()).rev() {
        let j = rng.gen_range(0..=i);
        embaralhados.swap(i, j);
    }
    embaralhados
        .chunks(2)
        .filter(|c| c.len() == 2)
        .map(|c| (c[0], c[1]))
        .collect()
}

fn gerar_ponto_de_corte(rng: &mut impl Rng) -> u32 {
    rng.gen_range(1..BITS)
}

fn cruzamento(ponto: u32, casais: &[(u16, u16)], rng: &mut impl Rng) -> Vec<u16> {
    let mask = (1u16 << ponto) - 1;
    let mut filhos = Vec::new();

    for &(pai1, pai2) in casais {
        if rng.gen_range(0.0..1.0_f64) < PROB_CRUZAMENTO {
            filhos.push((pai1 & !mask) | (pai2 & mask));
            filhos.push((pai2 & !mask) | (pai1 & mask));
        } else {
            filhos.push(pai1);
            filhos.push(pai2);
        }
    }

    filhos
}

fn efetuar_mutacao(filhos: &[u16], rng: &mut impl Rng) -> Vec<u16> {
    filhos
        .iter()
        .map(|&crom| {
            if rng.gen_range(0.0..1.0_f64) < PROB_MUTACAO {
                let bit = rng.gen_range(0..BITS);
                crom ^ (1u16 << bit)
            } else {
                crom
            }
        })
        .collect()
}

fn plotar_convergencia(historico: &[(usize, f64)]) {
    let root = BitMapBackend::new("convergencia.png", (900, 500)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let y_min = historico.iter().map(|&(_, fx)| fx).fold(f64::INFINITY, f64::min);
    let y_max = historico.iter().map(|&(_, fx)| fx).fold(f64::NEG_INFINITY, f64::max);
    let margem_y = (y_max - y_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Convergência do AG — melhor f(x) por geração", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            1usize..NUM_GERACOES,
            (y_min - margem_y)..(y_max + margem_y),
        )
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
        .draw_series(historico.iter().map(|&(g, fx)| {
            Circle::new((g, fx), 3, BLUE.filled())
        }))
        .unwrap();

    chart.configure_series_labels().border_style(&BLACK).draw().unwrap();
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

    let y_min = pontos.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min);
    let y_max = pontos.iter().map(|&(_, y)| y).fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("f(x) = -|x·sin(√|x|)| com mínimo encontrado pelo AG", ("sans-serif", 18))
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

    chart.configure_series_labels().border_style(&BLACK).draw().unwrap();
    println!("Gráfico salvo: funcao.png");
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut pop = gerar_populacao(&mut rng);

    println!("Algoritmo Genético — Minimização de f(x) = -|x·sin(√|x|)|");
    println!(
        "Intervalo: [0, {}] | Bits: {} | Pop: {} | Gerações: {} | P_cruz: {} | P_mut: {}",
        X_MAX, BITS, POP_SIZE, NUM_GERACOES, PROB_CRUZAMENTO, PROB_MUTACAO
    );
    println!("{:-<65}", "");

    let mut melhor_x = 0.0f64;
    let mut melhor_fx = 0.0f64;
    let mut primeira = true;
    let mut historico: Vec<(usize, f64)> = Vec::new();

    for geracao in 0..NUM_GERACOES {
        let pop_dec: Vec<f64> = pop.iter().map(|&c| decodificar(c)).collect();
        let imagem = gerar_imagem(&pop_dec);
        let probab_rolet = gerar_probabilidades(&imagem);
        let s_melhores = separar_vinte_melhores(&probab_rolet, &pop, &mut rng);
        let casais_formados = sortear_casais(&s_melhores, &mut rng);
        let ponto_corte = gerar_ponto_de_corte(&mut rng);
        let s_filhos = cruzamento(ponto_corte, &casais_formados, &mut rng);
        let s_filhos = efetuar_mutacao(&s_filhos, &mut rng);

        pop = s_melhores;
        pop.extend(s_filhos);
        while pop.len() < POP_SIZE {
            pop.push(rng.gen_range(0..=MAX_CROM));
        }

        let (gen_x, gen_fx) = pop
            .iter()
            .map(|&c| {
                let x = decodificar(c);
                (x, f(x))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        if primeira || gen_fx < melhor_fx {
            melhor_fx = gen_fx;
            melhor_x = gen_x;
            primeira = false;
        }

        historico.push((geracao + 1, melhor_fx));

        println!(
            "Gen {:>3} | x: {:>10.4} | f(x): {:>12.6} | melhor global: {:>12.6}",
            geracao + 1,
            gen_x,
            gen_fx,
            melhor_fx
        );
    }

    println!("{:-<65}", "");
    println!("Mínimo encontrado: x* = {:.6}, f(x*) = {:.6}", melhor_x, melhor_fx);

    plotar_convergencia(&historico);
    plotar_funcao(melhor_x, melhor_fx);
}
