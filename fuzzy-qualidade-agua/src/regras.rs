use crate::variaveis::{TermoCor, TermoPh, TermoQualidade, TermoTurbidez};

#[derive(Clone, Copy)]
pub struct Regra {
    pub cor: TermoCor,
    pub ph: TermoPh,
    pub turbidez: TermoTurbidez,
    pub saida: TermoQualidade,
}

use TermoQualidade::Adequada as ADE;
use TermoQualidade::Boa as BOA;
use TermoQualidade::Inadequada as INA;

// ordem das linhas: [InadequadoBaixo, AdequadoBaixo, Bom, AdequadoAlto, InadequadoAlto]
// ordem das colunas: [Boa, Adequada, Inadequada]  (turbidez)

/// regras quando a aparência da água é boa.
const TABELA_BOA: [[TermoQualidade; 3]; 5] = [
    [INA, INA, INA],
    [ADE, ADE, INA],
    [BOA, BOA, INA],
    [ADE, ADE, INA],
    [INA, INA, INA],
];

/// regras quando a aparência da água é adequada.
const TABELA_ADEQUADA: [[TermoQualidade; 3]; 5] = [
    [INA, INA, INA],
    [ADE, ADE, INA],
    [ADE, ADE, INA],
    [ADE, ADE, INA],
    [INA, INA, INA],
];

/// regras quando a aparência da água é inadequada.
const TABELA_INADEQUADA: [[TermoQualidade; 3]; 5] = [
    [INA, INA, INA],
    [INA, INA, INA],
    [ADE, ADE, INA],
    [INA, INA, INA],
    [INA, INA, INA],
];

pub fn base_de_regras() -> Vec<Regra> {
    let tabelas = [
        (TermoCor::Boa, &TABELA_BOA),
        (TermoCor::Adequada, &TABELA_ADEQUADA),
        (TermoCor::Inadequada, &TABELA_INADEQUADA),
    ];

    let mut regras = Vec::with_capacity(45);
    for (cor, tabela) in tabelas {
        for (i, ph) in TermoPh::TODOS.iter().enumerate() {
            for (j, turbidez) in TermoTurbidez::TODOS.iter().enumerate() {
                regras.push(Regra {
                    cor,
                    ph: *ph,
                    turbidez: *turbidez,
                    saida: tabela[i][j],
                });
            }
        }
    }
    regras
}
