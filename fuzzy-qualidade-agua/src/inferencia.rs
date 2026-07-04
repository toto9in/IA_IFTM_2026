use crate::regras::base_de_regras;
use crate::variaveis::{TermoCor, TermoPh, TermoQualidade, TermoTurbidez};

pub struct GrauEntrada {
    pub termo: String,
    pub grau: f64,
}

pub struct RegraAtivada {
    pub cor: String,
    pub ph: String,
    pub turbidez: String,
    pub saida: String,
    pub forca: f64,
}

pub struct Resultado {
    pub graus_cor: Vec<GrauEntrada>,
    pub graus_ph: Vec<GrauEntrada>,
    pub graus_turbidez: Vec<GrauEntrada>,
    pub regras: Vec<RegraAtivada>,
    pub qualidade: f64,
    pub classificacao: String,
}

/// fuzzifica as três entradas, avalia as regras e defuzzifica pelo centroide.
pub fn inferir(cor: f64, ph: f64, turbidez: f64) -> Resultado {
    // fuzzificação: grau de pertinência em cada termo de cada entrada ---
    let graus_cor = fuzzificar_cor(cor);
    let graus_ph = fuzzificar_ph(ph);
    let graus_turbidez = fuzzificar_turbidez(turbidez);

    // avaliação das regras
    let mut regras_ativadas = Vec::new();
    // força de ativação de cada termo de saída (agregação por máximo)
    let mut forca_saida = [0.0_f64; 3];

    for regra in base_de_regras() {
        let forca = regra
            .cor
            .trapezio()
            .grau(cor)
            .min(regra.ph.trapezio().grau(ph))
            .min(regra.turbidez.trapezio().grau(turbidez));

        if forca > 0.0 {
            let idx = indice_qualidade(regra.saida);
            forca_saida[idx] = forca_saida[idx].max(forca);

            regras_ativadas.push(RegraAtivada {
                cor: regra.cor.nome().to_string(),
                ph: regra.ph.nome().to_string(),
                turbidez: regra.turbidez.nome().to_string(),
                saida: regra.saida.nome().to_string(),
                forca,
            });
        }
    }

    // regras mais fortes primeiro, para leitura
    regras_ativadas.sort_by(|a, b| b.forca.partial_cmp(&a.forca).unwrap());

    // defuzzificação por centroide sobre a saída agregada ---
    let qualidade = defuzzificar_centroide(&forca_saida);
    let classificacao = classificar(qualidade);

    Resultado {
        graus_cor,
        graus_ph,
        graus_turbidez,
        regras: regras_ativadas,
        qualidade,
        classificacao,
    }
}

fn fuzzificar_cor(cor: f64) -> Vec<GrauEntrada> {
    TermoCor::TODOS
        .iter()
        .map(|t| GrauEntrada {
            termo: t.nome().to_string(),
            grau: t.trapezio().grau(cor),
        })
        .collect()
}

fn fuzzificar_ph(ph: f64) -> Vec<GrauEntrada> {
    TermoPh::TODOS
        .iter()
        .map(|t| GrauEntrada {
            termo: t.nome().to_string(),
            grau: t.trapezio().grau(ph),
        })
        .collect()
}

fn fuzzificar_turbidez(turbidez: f64) -> Vec<GrauEntrada> {
    TermoTurbidez::TODOS
        .iter()
        .map(|t| GrauEntrada {
            termo: t.nome().to_string(),
            grau: t.trapezio().grau(turbidez),
        })
        .collect()
}

fn indice_qualidade(termo: TermoQualidade) -> usize {
    match termo {
        TermoQualidade::Inadequada => 0,
        TermoQualidade::Adequada => 1,
        TermoQualidade::Boa => 2,
    }
}

/// defuzzificação por centroide: amostra o domínio [0, 1] e integra
/// numericamente o conjunto
fn defuzzificar_centroide(forca_saida: &[f64; 3]) -> f64 {
    let passos = 1000;
    let mut soma_momento = 0.0;
    let mut soma_area = 0.0;

    for k in 0..=passos {
        let q = k as f64 / passos as f64;

        // pertinência agregada = máximo dos termos de saída recortados na força
        let mut mu = 0.0_f64;
        for (i, termo) in TermoQualidade::TODOS.iter().enumerate() {
            let recorte = termo.trapezio().grau(q).min(forca_saida[i]);
            mu = mu.max(recorte);
        }

        soma_momento += q * mu;
        soma_area += mu;
    }

    if soma_area == 0.0 {
        // nenhuma regra disparou: retorna o centro do domínio
        0.5
    } else {
        soma_momento / soma_area
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn exemplo_do_enunciado() {
        // cor 15 UH, pH 7, turbidez 0 UT -> ~0.6, qualidade adequada
        let r = inferir(15.0, 7.0, 0.0);
        assert_eq!(r.classificacao, "adequada");
        assert!(
            (r.qualidade - 0.6).abs() < 0.05,
            "qualidade = {}",
            r.qualidade
        );
    }

    #[test]
    fn agua_otima() {
        // cor ~0, pH 7, turbidez 0 -> boa
        let r = inferir(0.0, 7.0, 0.0);
        assert_eq!(r.classificacao, "boa");
    }

    #[test]
    fn agua_ruim() {
        // cor alta, pH extremo, turbidez alta -> inadequada
        let r = inferir(25.0, 3.0, 9.0);
        assert_eq!(r.classificacao, "inadequada");
    }
}

/// classifica o valor defuzzificado no termo de saída de maior pertinência.
fn classificar(qualidade: f64) -> String {
    let mut melhor = TermoQualidade::TODOS[0];
    let mut melhor_grau = -1.0_f64;
    for termo in TermoQualidade::TODOS {
        let g = termo.trapezio().grau(qualidade);
        if g > melhor_grau {
            melhor_grau = g;
            melhor = termo;
        }
    }
    melhor.nome().to_string()
}
