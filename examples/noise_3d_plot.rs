use plotters::prelude::*;

use dynamic_poisson_sampling::*;

fn main() {
    let root = BitMapBackend::new("examples/images/3d-env.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Empty 3D Figure", ("sans-serif", 40))
        .build_cartesian_3d(0.0..1.0, 0.0..1.0, 0.0..1.0)
        .unwrap();
    chart.configure_axes().draw().unwrap();

    
    let mut rng = rand::thread_rng();
    let points = get_points(12, (0.5, 0.5, 0.5).into(), &mut rng, |pos|{
        if pos[0] < 0.0 || pos[0] >= 1.0 || pos[1] < 0.0 || pos[1] >= 1.0 || pos[2] < 0.0 || pos[2] >= 1.0{
            None
        }else{
            Some(0.05)
        }
    });

    chart.draw_series(PointSeries::of_element(
        points.iter().map(|arr| (arr[0], arr[1], arr[2])),
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0), s, st.filled()); // At this point, the new pixel coordinate is established
        },
    )).unwrap();
}