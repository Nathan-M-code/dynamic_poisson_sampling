
use std::f64::consts::PI;

pub struct Param {
    //boundaries of points (left, top, width, height)
    pub bounds: (f64, f64, f64, f64),
    //how many random points are generated and tested for each new point
    pub k: u32,
}

pub fn distance(pos0: (f64, f64), pos1: (f64, f64)) -> f64{
    ((pos0.0-pos1.0)*(pos0.0-pos1.0)+(pos0.1-pos1.1)*(pos0.1-pos1.1)).sqrt()
}
///
/// The density function 'density_func' takes the position of the tested point as a tuple argument
/// and returns an optional radius. If it is None, the point is discarded.
/// 
pub fn get_points<T>(param: &Param, rng: &mut impl rand::Rng, density_func:T) -> Vec<(f64, f64)>
where
    T: Fn((f64, f64)) -> Option<f64>
{
    struct Point{
        pos: (f64, f64),
        min_rad: f64,
    }

    //background grid
    let mut points: Vec<Point> = Vec::new();
    
    //active list
    let mut active_list: Vec<usize> = Vec::new();





    //first random point
    loop{
        let r_pos: (f64, f64) = (rng.gen_range(param.bounds.0 .. param.bounds.0+param.bounds.2), rng.gen_range(param.bounds.1 .. param.bounds.1+param.bounds.3));
        
        let r_rad = density_func(r_pos);
        if r_rad.is_none() { continue; }
        let r_rad = r_rad.unwrap();

        points.push(Point{
            pos: r_pos,
            min_rad: r_rad,
        });
    
        active_list.push(0);
        break;
    }




    loop {
        let r_ind = rng.gen_range(0..active_list.len());
        let curr_ind = *active_list.get(r_ind).unwrap();
        let (curr_pos, curr_rad) = (points.get(curr_ind).unwrap().pos, points.get(curr_ind).unwrap().min_rad);
        
        // let mut to_remove = true;
        'rloop: for _ in 0..param.k {
            let r_angle = rng.gen_range(0. .. 2.*PI);
            let r_distance = rng.gen_range(curr_rad .. 3.*curr_rad);
            let r_pos = (curr_pos.0 + f64::cos(r_angle)*r_distance, curr_pos.1 + f64::sin(r_angle)*r_distance);
    
            if r_pos.0 < param.bounds.0 || r_pos.0 >= param.bounds.0+param.bounds.2 || r_pos.1 < param.bounds.1 || r_pos.1 >= param.bounds.1+param.bounds.3 {
                continue 'rloop;
            }

            let r_rad = density_func(r_pos);
            if r_rad.is_none() { continue 'rloop; }
            let r_rad = r_rad.unwrap();
            // println!("{:?}",rad);



            for point in points.iter() {
                let distance = distance(point.pos, r_pos);
                if distance < point.min_rad+r_rad {
                    continue 'rloop;
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

    return points.iter().map(|p| p.pos).collect();
}