use plotly::{HeatMap, Plot, ImageFormat, Histogram};
use plotly::layout::{Axis, Layout};
use plotly::common::Title;
use plotly::common::{ColorScale, ColorScalePalette}; 

pub fn plot_histogram(data: &[f64], path: &str, main_title: &str, x_title: &str, x_range: Vec<f64>) {

    let mut plot = Plot::new();

    let layout = Layout::new()
        .title(Title::new(main_title))
        .x_axis(
            Axis::new()
            .title(Title::new(x_title))
            .range(x_range));

    plot.set_layout(layout);

    let trace = Histogram::new(data.to_vec()).name("h");
    plot.add_trace(trace);

    plot.write_image(path, ImageFormat::SVG, 600, 600, 1.0);
}

pub fn plot_heatmap(z: Vec<Vec<f64>>, title: &str, path: &str, f_name: &str, x_range: Vec<f64>, y_range: Vec<f64>) {

    // here define the palette color we want to use
    let custom_color_scale = ColorScale::Palette(ColorScalePalette::Viridis);

    // create the heatmap
    let trace = HeatMap::new_z(z)
        .x(x_range)
        .y(y_range)
        .color_scale(custom_color_scale);

    // create clot
    let mut plot = Plot::new();
    plot.add_trace(trace);

    // add a layout
    let layout = Layout::new()
        .title(Title::new(title))
        .x_axis(Axis::new()
                .title(Title::new("X parameter")))
        .y_axis(Axis::new()
                .title(Title::new("Y parameter")));

    // apply layout
    plot.set_layout(layout);

    let full_path = path.to_string() + f_name;

    // write plot
    plot.write_image(full_path, ImageFormat::SVG, 600, 600, 1.0);


}
