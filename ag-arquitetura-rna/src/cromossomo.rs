use rand::Rng;

// ---------------------------------------------------------------------------
// Faixas validas de cada posicao do cromossomo (edite aqui).
// O cromossomo e um Vec<String> de 6 posicoes. Os valores sao guardados como
// String (exigencia do enunciado); a conversao para inteiro/real so acontece
// no momento de usar o valor (funcoes de parsing abaixo).
// ---------------------------------------------------------------------------
pub const NUM_POSICOES: usize = 6;

pub const NEURONIOS_MIN: usize = 2; // pos 0: neuronios por camada oculta
pub const NEURONIOS_MAX: usize = 15;

pub const CAMADAS_MIN: usize = 2; // pos 1: numero de camadas ocultas
pub const CAMADAS_MAX: usize = 5;

pub const TAXA_MIN: f64 = 0.00001; // pos 2: taxa de aprendizagem
pub const TAXA_MAX: f64 = 0.1;

pub const EPOCAS_MIN: usize = 20; // pos 3: numero maximo de epocas
pub const EPOCAS_MAX: usize = 1000;

// pos 4: 1 = on-line, 2 = off-line
// pos 5: 1 = normaliza, 2 = nao normaliza

pub type Cromossomo = Vec<String>;

pub fn gene_aleatorio(posicao: usize, rng: &mut impl Rng) -> String {
    match posicao {
        0 => rng.gen_range(NEURONIOS_MIN..=NEURONIOS_MAX).to_string(),
        1 => rng.gen_range(CAMADAS_MIN..=CAMADAS_MAX).to_string(),
        2 => {
            let valor = TAXA_MIN + rng.gen::<f64>() * (TAXA_MAX - TAXA_MIN);
            format!("{valor:.6}")
        }
        3 => rng.gen_range(EPOCAS_MIN..=EPOCAS_MAX).to_string(),
        4 => rng.gen_range(1..=2).to_string(),
        5 => rng.gen_range(1..=2).to_string(),
        _ => unreachable!("posicao de cromossomo invalida: {posicao}"),
    }
}

pub fn cromossomo_aleatorio(rng: &mut impl Rng) -> Cromossomo {
    (0..NUM_POSICOES).map(|p| gene_aleatorio(p, rng)).collect()
}

pub fn neuronios(c: &Cromossomo) -> usize {
    c[0].parse().expect("gene 0 invalido")
}
pub fn camadas(c: &Cromossomo) -> usize {
    c[1].parse().expect("gene 1 invalido")
}
pub fn taxa(c: &Cromossomo) -> f64 {
    c[2].parse().expect("gene 2 invalido")
}
pub fn epocas(c: &Cromossomo) -> usize {
    c[3].parse().expect("gene 3 invalido")
}

pub fn online(c: &Cromossomo) -> bool {
    c[4] == "1"
}

pub fn normaliza(c: &Cromossomo) -> bool {
    c[5] == "1"
}

pub fn descrever(c: &Cromossomo) -> String {
    format!(
        "  pos 0 - neuronios por camada oculta : {}\n\
         \x20 pos 1 - numero de camadas ocultas   : {}\n\
         \x20 pos 2 - taxa de aprendizagem        : {}\n\
         \x20 pos 3 - maximo de epocas            : {}\n\
         \x20 pos 4 - modo de treino             : {} ({})\n\
         \x20 pos 5 - normalizacao               : {} ({})",
        neuronios(c),
        camadas(c),
        taxa(c),
        epocas(c),
        c[4],
        if online(c) { "on-line" } else { "off-line" },
        c[5],
        if normaliza(c) {
            "normaliza"
        } else {
            "nao normaliza"
        },
    )
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn genes_sempre_na_faixa() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            let n: usize = gene_aleatorio(0, &mut rng).parse().unwrap();
            assert!((NEURONIOS_MIN..=NEURONIOS_MAX).contains(&n));

            let cam: usize = gene_aleatorio(1, &mut rng).parse().unwrap();
            assert!((CAMADAS_MIN..=CAMADAS_MAX).contains(&cam));

            let t: f64 = gene_aleatorio(2, &mut rng).parse().unwrap();
            assert!((TAXA_MIN..=TAXA_MAX).contains(&t));

            let e: usize = gene_aleatorio(3, &mut rng).parse().unwrap();
            assert!((EPOCAS_MIN..=EPOCAS_MAX).contains(&e));

            let modo: usize = gene_aleatorio(4, &mut rng).parse().unwrap();
            assert!((1..=2).contains(&modo));

            let norm: usize = gene_aleatorio(5, &mut rng).parse().unwrap();
            assert!((1..=2).contains(&norm));
        }
    }

    #[test]
    fn cromossomo_tem_seis_posicoes() {
        let mut rng = rand::thread_rng();
        let c = cromossomo_aleatorio(&mut rng);
        assert_eq!(c.len(), NUM_POSICOES);
    }

    #[test]
    fn parsers_batem_com_genes() {
        let c: Cromossomo = vec![
            "8".into(),
            "3".into(),
            "0.010000".into(),
            "500".into(),
            "1".into(),
            "2".into(),
        ];
        assert_eq!(neuronios(&c), 8);
        assert_eq!(camadas(&c), 3);
        assert!((taxa(&c) - 0.01).abs() < 1e-9);
        assert_eq!(epocas(&c), 500);
        assert!(online(&c));
        assert!(!normaliza(&c));
    }
}
