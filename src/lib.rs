
use std::{collections::HashMap, f64::consts::PI};

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
pub fn get_points<T>(param: &Param, rng: impl rand::Rng, density_func:T) -> Vec<(f64, f64)>
where
    T: Fn((f64, f64)) -> Option<f64>
{
    struct Point{
        pos: (f64, f64),
        min_rad: f64,
    }

    let grid_size = param.bounds.2-param.bounds.0;

    //background grid
    let mut grid: HashMap<(u32, u32), Vec<Point>> = HashMap::new();
    
    //active list
    let mut active_list: Vec<&Point> = Vec::new();





    //first random point
    loop{
        let r_pos: (f64, f64) = (rng.gen_range(param.bounds.0 .. param.bounds.0+param.bounds.2), rng.gen_range(param.bounds.1 .. param.bounds.1+param.bounds.3));
        
        let r_rad = density_func(r_pos);
        if r_rad.is_none() { continue; }
        let r_rad = r_rad.unwrap();

        let ind_grid = ((r_pos.0/grid_size) as u32, (r_pos.1/grid_size) as u32);
        grid.entry(ind_grid).or_insert(Vec::new()).push(Point{
            pos: r_pos,
            min_rad: r_rad,
        });
    
        active_list.push(&grid.get(&ind_grid).unwrap()[0]);
        break;
    }




    loop {
        let r_ind = rng.gen_range(0..active_list.len());
        let curr_point = *active_list.get(r_ind).unwrap();
        
        let rad = density_func(curr_point.pos);
        if rad.is_none() { continue; }
        let rad_current_pos = rad.unwrap();
        
        // let mut to_remove = true;
        for _ in 0..param.k {
            let r_angle = rng.gen_range(0. .. 2.*PI);
            let r_distance = rng.gen_range(rad_current_pos .. 2.*rad_current_pos);
            let r_pos = (current_pos.0 + f64::cos(r_angle)*r_distance, current_pos.1 + f64::sin(r_angle)*r_distance);
    
            if r_pos.0 < param.bounds_min.0 || r_pos.0 >= param.bounds_max.0 || r_pos.1 < param.bounds_min.1 || r_pos.1 >= param.bounds_max.1 {
                continue;
            }

            let r_rad = density_func(r_pos);
            if r_rad.is_none() { continue; }
            let r_rad = r_rad.unwrap();
            // println!("{:?}",rad);



            let ind_grid = ((r_pos.0/grid_size) as u32, (r_pos.1/grid_size) as u32);

            let mut check_indices: Vec<(u32, u32)> = Vec::new();
            let nb_tile_check = (r_rad/grid_size).ceil() as u32;
            for x_check in (ind_grid.0 as i64)-(nb_tile_check as i64) .. (ind_grid.0+nb_tile_check) as i64 {
                for y_check in (ind_grid.1 as i64)-(nb_tile_check as i64) .. (ind_grid.1+nb_tile_check) as i64 {
                    if x_check >= 0 && y_check >= 0 {
                        check_indices.push((x_check as u32, y_check as u32));
                    }
                }
            }
            
            let mut min_distance = None;
            for indices in check_indices.iter() {
                let p_check = grid.get(indices);
                if p_check.is_some() {
                    let p = p_check.unwrap();
                    let distance = distance(*p, r_pos);
                    if min_distance.is_none() || distance < min_distance.unwrap() {
                        min_distance = Some(distance);
                    }
                }
            }

            if min_distance.is_none() || min_distance.is_some_and(|d| d > r_rad) {
                let already: Option<(f64, f64)> = grid.insert(ind_grid, r_pos);
                if already.is_some() {
                    println!("had already");
                }
                active_list.push(r_pos);
            }
        }
        
        active_list.remove(r_ind);

        if active_list.is_empty() {
            break;
        }
    }

    return grid.iter().map(|p| *p.1).collect();
}