use lang_lib::mecab::convert_sentences;
use std::collections::HashMap;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    let mut map: HashMap<u32, u32> = HashMap::new();

    let text = include_str!("../../assets/neko.txt.mecab");
    let sentences = convert_sentences(&text);
    sentences.iter().for_each(|sentence| {
        sentence.morphemes.iter().for_each(|morpheme| {
            let key = &morpheme.surface;
            *word_count.entry(key).or_default() += 1;
        })
    });

    word_count.iter().for_each(|(_word, &count)| {
        let key = count;
        *map.entry(key).or_default() += 1;
    });


    let mut vec: Vec<_> = map.iter().map(|(&l, &v)| (l, v)).collect();
    vec.sort();


    let max_data = vec.first().unwrap().1;
    let font = "sans-serif";

    println!("data: {:?}", vec);

    let root =
        BitMapBackend::new("./output/038.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("単語の頻度", (font, 50).into_font())
        .build_cartesian_2d((0u32..max_data).into_segmented(), 0u32..max_data)?;

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
            .data(vec),
    )?;

    Ok(())
}
