use crate::fuzzy::TermoTrapezoidal;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TermoCor {
    Boa,
    Adequada,
    Inadequada,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TermoPh {
    InadequadoBaixo,
    AdequadoBaixo,
    Bom,
    AdequadoAlto,
    InadequadoAlto,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TermoTurbidez {
    Boa,
    Adequada,
    Inadequada,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TermoQualidade {
    Inadequada,
    Adequada,
    Boa,
}

impl TermoCor {
    pub const TODOS: [TermoCor; 3] = [TermoCor::Boa, TermoCor::Adequada, TermoCor::Inadequada];

    pub fn nome(&self) -> &'static str {
        match self {
            TermoCor::Boa => "boa",
            TermoCor::Adequada => "adequada",
            TermoCor::Inadequada => "inadequada",
        }
    }

    pub fn trapezio(&self) -> TermoTrapezoidal {
        match self {
            TermoCor::Boa => TermoTrapezoidal::new(0.0, 0.0, 4.0, 6.0),
            TermoCor::Adequada => TermoTrapezoidal::new(4.0, 6.0, 14.0, 16.0),
            TermoCor::Inadequada => TermoTrapezoidal::new(14.0, 16.0, 30.0, 30.0),
        }
    }
}

impl TermoPh {
    pub const TODOS: [TermoPh; 5] = [
        TermoPh::InadequadoBaixo,
        TermoPh::AdequadoBaixo,
        TermoPh::Bom,
        TermoPh::AdequadoAlto,
        TermoPh::InadequadoAlto,
    ];

    pub fn nome(&self) -> &'static str {
        match self {
            TermoPh::InadequadoBaixo => "inadequado baixo",
            TermoPh::AdequadoBaixo => "adequado baixo",
            TermoPh::Bom => "bom",
            TermoPh::AdequadoAlto => "adequado alto",
            TermoPh::InadequadoAlto => "inadequado alto",
        }
    }

    pub fn trapezio(&self) -> TermoTrapezoidal {
        match self {
            TermoPh::InadequadoBaixo => TermoTrapezoidal::new(0.0, 0.0, 5.5, 6.0),
            TermoPh::AdequadoBaixo => TermoTrapezoidal::new(5.5, 6.0, 6.2, 6.6),
            TermoPh::Bom => TermoTrapezoidal::new(6.2, 6.6, 8.4, 8.8),
            TermoPh::AdequadoAlto => TermoTrapezoidal::new(8.4, 8.8, 9.4, 10.0),
            TermoPh::InadequadoAlto => TermoTrapezoidal::new(9.4, 10.0, 14.0, 14.0),
        }
    }
}

impl TermoTurbidez {
    pub const TODOS: [TermoTurbidez; 3] = [
        TermoTurbidez::Boa,
        TermoTurbidez::Adequada,
        TermoTurbidez::Inadequada,
    ];

    pub fn nome(&self) -> &'static str {
        match self {
            TermoTurbidez::Boa => "boa",
            TermoTurbidez::Adequada => "adequada",
            TermoTurbidez::Inadequada => "inadequada",
        }
    }

    pub fn trapezio(&self) -> TermoTrapezoidal {
        match self {
            TermoTurbidez::Boa => TermoTrapezoidal::new(0.0, 0.0, 1.0, 2.0),
            TermoTurbidez::Adequada => TermoTrapezoidal::new(1.0, 2.0, 4.0, 6.0),
            TermoTurbidez::Inadequada => TermoTrapezoidal::new(4.0, 6.0, 10.0, 10.0),
        }
    }
}

impl TermoQualidade {
    pub const TODOS: [TermoQualidade; 3] = [
        TermoQualidade::Inadequada,
        TermoQualidade::Adequada,
        TermoQualidade::Boa,
    ];

    pub fn nome(&self) -> &'static str {
        match self {
            TermoQualidade::Inadequada => "inadequada",
            TermoQualidade::Adequada => "adequada",
            TermoQualidade::Boa => "boa",
        }
    }

    pub fn trapezio(&self) -> TermoTrapezoidal {
        match self {
            TermoQualidade::Inadequada => TermoTrapezoidal::new(0.0, 0.0, 0.3, 0.45),
            TermoQualidade::Adequada => TermoTrapezoidal::new(0.35, 0.45, 0.7, 0.8),
            TermoQualidade::Boa => TermoTrapezoidal::new(0.7, 0.8, 1.0, 1.0),
        }
    }
}
