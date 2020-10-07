
use std::f64::consts::*;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, LineStyle};

fn get_harmonic<T : Into<f64>>(tap : f64, harmonic : T) -> f64
{
    (tap*PI*harmonic.into()).sin()
}

fn main()
{
    let scale_length : f64 = 25.5;
    let bobbin_radius = 3.0/16.0/2.0;
    //let bobbin_radius = 3.0/32.0/2.0;
    //let tap_a : f64 = 5.0;
    let tap_a : f64 = 1.5;
    let extra_taps = [0.75];
    //let extra_taps = [];
    
    let tuning_hz = 73.4162;
    let max_hz = 10000.0;
    let mut reached_harmonic = 1.0;
    let mut reached_freq = tuning_hz;
    while reached_freq < max_hz
    {
        reached_freq = (reached_harmonic+1.0) * tuning_hz;
        if reached_freq < max_hz
        {
            reached_harmonic += 1.0;
        }
    }
    
    let sample_harmonic = |h : f64| -> f64
    {
        let mut taps = vec![
          tap_a - bobbin_radius,
          tap_a + bobbin_radius,
        ];
        for tap_offset in extra_taps.iter()
        {
            taps.push(tap_a - bobbin_radius + tap_offset);
            taps.push(tap_a + bobbin_radius + tap_offset);
        }
        taps.iter().map(|tap| get_harmonic(tap/scale_length, h)).sum::<f64>().abs() / taps.len() as f64
    };
    
    let sample_freq = |hz : f64| -> f64
    {
        let harmonic = hz/tuning_hz;
        sample_harmonic(harmonic)
    };
    
    let s1: Plot = Plot::from_function(sample_freq, tuning_hz, reached_freq).line_style(LineStyle::new());

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new().add(s1).y_range(0.0, 1.0);

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("plot.svg").unwrap();
}
