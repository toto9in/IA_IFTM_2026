use plotters::prelude::*;

use crate::ag::RegistroGeracao;

pub fn salvar_convergencia(
    historico: &[RegistroGeracao],
    caminho: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let area = BitMapBackend::new(caminho, (900, 600)).into_drawing_area();
    area.fill(&WHITE)?;

    let max_geracao = historico.len().max(1);

    let max_fitness = historico
        .iter()
        .map(|r| r.fitness_medio)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let mut grafico = ChartBuilder::on(&area)
        .caption(
            "Convergencia do AG - Funcao de Rastrigin",
            ("sans-serif", 28),
        )
        .margin(20)
        .x_label_area_size(45)
        .y_label_area_size(60)
        .build_cartesian_2d(0..max_geracao, 0.0..max_fitness)?;

    grafico
        .configure_mesh()
        .x_desc("Geracao")
        .y_desc("Fitness (Rastrigin)")
        .draw()?;

    grafico
        .draw_series(LineSeries::new(
            historico.iter().map(|r| (r.geracao, r.melhor_fitness)),
            &RED,
        ))?
        .label("Melhor fitness")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    grafico
        .draw_series(LineSeries::new(
            historico.iter().map(|r| (r.geracao, r.fitness_medio)),
            &BLUE,
        ))?
        .label("Fitness medio")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    grafico
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    area.present()?;
    Ok(())
}
