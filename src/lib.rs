
use rand;


fn distance<const N: usize>(lhs: &[f64; N], rhs: &[f64; N]) -> f64{
    lhs.iter()
    .zip(rhs.iter())
    .map(|(&l, &r)| {
        let diff = l - r;
        diff * diff
    })
    .sum::<f64>().sqrt()
}

///
/// The density function 'density_func' takes the position of the tested point as a tuple argument
/// and returns an optional radius. If it is None, the point is discarded.
/// 
pub fn get_points<const N: usize, T>(k: u32, first_pos: [f64; N], rng: &mut impl rand::Rng, density_func:T)
-> Vec<[f64; N]>
where
    T: Fn(&[f64; N]) -> Option<f64>
{
    struct Point<const N: usize>{
        pos: [f64; N],
        min_rad: f64,
    }

    let mut points: Vec<Point<N>> = Vec::new();
    
    //active list
    let mut active_list: Vec<usize> = Vec::new();





    //first point
    let r_rad = density_func(&first_pos);
    if r_rad.is_none() { return vec![]; }
    let r_rad = r_rad.unwrap();

    points.push(Point{
        pos: first_pos,
        min_rad: r_rad,
    });
    active_list.push(0);



    loop {
        let r_ind = rng.gen_range(0..active_list.len());
        let curr_ind = active_list[r_ind];
        let (curr_pos, curr_rad) = (points[curr_ind].pos, points[curr_ind].min_rad);
        
        // let mut to_remove = true;
        'k_l: for _ in 0..k {
            let r_distance = rng.gen_range(curr_rad .. 3.*curr_rad);


            let mut deltas: [f64; N] = [0.0; N];

            // Generate random direction
            for delta in &mut deltas {
                *delta = rng.gen_range(-1.0..=1.0);
            }
            
            // Add deltas to current position
            let norm = (deltas.iter().map(|&x| x.powi(2)).sum::<f64>()).sqrt();
            let mut r_pos: [f64; N] = [0.0; N];
            for i in 0..N{
                r_pos[i] = curr_pos[i]+(deltas[i]/norm)*r_distance;
            }


            let r_rad = density_func(&r_pos);
            if r_rad.is_none() { continue 'k_l; }
            let r_rad = r_rad.unwrap();

            for point in points.iter() {
                let distance = distance(&point.pos, &r_pos);
                if distance < point.min_rad+r_rad {
                    continue 'k_l;
                }
            }

            points.push(Point{
                pos: r_pos,
                min_rad: r_rad,
            });
        
            active_list.push(points.len()-1);
        }
        
        active_list.remove(r_ind);

        if active_list.is_empty() {
            break;
        }
    }

    points.iter().map(|p| p.pos).collect()
}