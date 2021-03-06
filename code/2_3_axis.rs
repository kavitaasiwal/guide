use plotters::prelude::*;

fn main() {
    let root_drawing_area = BitMapBackend::new("images/2.3.png", (600, 400)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        // enables Y axis, the size is 40 px
        .set_label_area_size(LabelAreaPosition::Left, 40)
        // enable X axis, the size is 40 px
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(0..100, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
}
