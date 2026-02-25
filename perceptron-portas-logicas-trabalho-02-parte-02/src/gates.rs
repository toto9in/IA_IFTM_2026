#[derive(Clone, Copy, PartialEq)]
pub enum PortaLogica {
    AND,
    NAND,
    OR,
    NOR,
    XOR,
}

impl PortaLogica {
    pub fn nome(&self) -> &'static str {
        match self {
            PortaLogica::AND => "AND",
            PortaLogica::NAND => "NAND",
            PortaLogica::OR => "OR",
            PortaLogica::NOR => "NOR",
            PortaLogica::XOR => "XOR",
        }
    }
}

/// Retorna as amostras de treino para a porta lógica especificada.
/// Entradas: combinações binárias {0.0, 1.0} para (x1, x2).
/// Saídas bipolares: {-1.0, +1.0}.
pub fn amostras_para(porta: PortaLogica) -> Vec<(Vec<f64>, f64)> {
    let entradas = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let saidas: Vec<f64> = match porta {
        // AND: só +1 quando ambas as entradas são 1
        PortaLogica::AND  => vec![-1.0, -1.0, -1.0,  1.0],
        // NAND: negação do AND
        PortaLogica::NAND => vec![ 1.0,  1.0,  1.0, -1.0],
        // OR: +1 quando ao menos uma entrada é 1
        PortaLogica::OR   => vec![-1.0,  1.0,  1.0,  1.0],
        // NOR: negação do OR
        PortaLogica::NOR  => vec![ 1.0, -1.0, -1.0, -1.0],
        // XOR: +1 quando as entradas diferem (não linearmente separável)
        PortaLogica::XOR  => vec![-1.0,  1.0,  1.0, -1.0],
    };

    entradas.into_iter().zip(saidas.into_iter()).collect()
}
