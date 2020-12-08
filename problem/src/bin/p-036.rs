use lang_lib::mecab::convert_sentences;
use std::collections::HashMap;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut hash_map: HashMap<&str, u32> = HashMap::new();
    let text = include_str!("../../assets/neko.txt.mecab");
    let sentences = convert_sentences(&text);
    sentences.iter().for_each(|sentence| {
        sentence.morphemes.iter().for_each(|morpheme| {
            let key = &morpheme.surface;
            *hash_map.entry(key).or_default() += 1;
        })
    });
    let mut vec: Vec<_> = hash_map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));


    let data: Vec<_> = vec.iter().take(10).map(|(&_label, &value)| value).collect();
    let labels: Vec<_> = vec.iter().take(10).map(|(&label, &_value)| label).collect();
    let max_data = data.first().unwrap();
    let font = "sans-serif";

    println!("data: {:?}", data);
    println!("labels : {:?}", labels);

    let root =
        BitMapBackend::new("./output/036.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("単語の頻度", (font, 50).into_font())
        .build_cartesian_2d((0u32..9).into_segmented(), 0u32..*max_data)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("頻度")
        .x_desc("単語")
        .axis_desc_style((font, 15).into_font())
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().enumerate().map(|(index, x)| (index as u32, *x))),
    )?;

    Ok(())
}
