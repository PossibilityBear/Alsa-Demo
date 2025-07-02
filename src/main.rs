mod waves;
mod notes;
mod playback;
mod ui_controllers;
use playback::Playback;
use crate::ui_controllers::app::App;
use crate::ui_controllers::wave_widget::WaveWidget;
use ratatui;

const WAVE_CHAR_HEIGHT: u32 = 7;


fn main() {
    /* let mut playback = Playback::new();
    // let my_wave: SinWave = SinWave::default();
    // playback.write_wave(&my_wave);
    // // Plot it to terminal
    // Chart::new(200, 10, 0.0, 0.5)
    //     .x_axis_style(LineStyle::None)
    //     .y_axis_style(LineStyle::None)
    //     .lineplot(&Shape::Continuous(
    //         Box::new(|x| {my_wave.wave_func(x, 1)})))
    //     .display();
    // let my_wave: SinWave = SinWave::default();
    // let mut chart = Chart::new(200, 10, 0.0, 0.5);
    // let shape = &Shape::Continuous(
    //         Box::new(|x| {my_wave.wave_func(x, 1)}));
    // let chart = chart
    //     .x_axis_style(LineStyle::None)
    //     .y_axis_style(LineStyle::None)
    //     .lineplot(shape);
    // chart.axis();
    // chart.figures();
    // println!("{}", chart);
    // println!("My chart: {:?}", chart.clone().frame());
    // playback.play(); */
    



    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result;


         

}
