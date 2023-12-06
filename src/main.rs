

use std::{collections::HashMap, f64::consts::PI};

use imageproc::{window::display_image, drawing::draw_filled_circle_mut};
use noise::{permutationtable::PermutationTable, utils::{PlaneMapBuilder, NoiseMapBuilder, NoiseMap}, core::perlin::perlin_2d};

use image::{ImageBuffer, Luma, Rgb, buffer::ConvertBuffer};

use rand::Rng;

pub struct Param {
    //how many random points are generated and tested for each new point
    pub k: u32,
    //from where we dont plot point
    pub threshold_min: f64,
    //from where we cant go up than max_distance
    pub threshold_max: f64,
    //when noise = 0, how many pixel
    pub min_distance: f64,
    //when noise = 1, how many pixel
    pub max_distance: f64,
}

pub fn noise_map_to_image_buffer(nm: &NoiseMap) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    // collect the values from f64 into u8 in a separate vec
    let (width, height) = nm.size();
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

    for v in nm.iter(){
        pixels.push(((v * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8);
    }

    ImageBuffer::from_raw(width as u32, height as u32, pixels).unwrap()
}

pub fn distance(pos0: (f64, f64), pos1: (f64, f64)) -> f64{
    ((pos0.0-pos1.0)*(pos0.0-pos1.0)+(pos0.1-pos1.1)*(pos0.1-pos1.1)).sqrt()
}

fn main() {

    let mut rng = rand::thread_rng();

    let size_noise: (u32, u32) = (500,500);
    // let mid_noise = (size_noise.0 as f64 /2., size_noise.1 as f64 /2.);
    // let diag_noise = ((size_noise.0*size_noise.0 + size_noise.1*size_noise.1)as f64).sqrt();
    let freq_noise = 3.;
    

    //set_size represents the precision
    //set_*_bounds represents the frequency

    let hasher = PermutationTable::new(rng.gen());
    let nm = PlaneMapBuilder::new_fn(|point, hasher| perlin_2d(point.into(), hasher), &hasher)
        .set_size(size_noise.0 as usize, size_noise.1 as usize)
        .set_x_bounds(0., freq_noise)
        .set_y_bounds(0., freq_noise)
        .build();
    let noise_height = noise_map_to_image_buffer(&nm);
    display_image("noise_height", &noise_height, size_noise.0, size_noise.1);


    let param = Param{
        k: 25,
        threshold_min: 0.3,
        threshold_max: 1.,
        min_distance: 5.,
        max_distance: 24.,
    };

    


    //background grid
    let tile_size = param.min_distance/f64::sqrt(2.);
    let mut grid: HashMap<(u32, u32), (f64, f64)> = HashMap::new();

    //first random point
    let r_pos: (f64, f64) = (rng.gen_range(0. .. size_noise.0 as f64), rng.gen_range(0. .. size_noise.0 as f64));

    let ind_grid = ((r_pos.0/tile_size) as u32, (r_pos.1/tile_size) as u32);
    grid.insert(ind_grid, r_pos);

    //active list
    let mut active_list: Vec<(f64, f64)> = Vec::new();
    active_list.push(r_pos);

    loop {
        let r_ind = rng.gen_range(0..active_list.len());
        let current_pos = *active_list.get(r_ind).unwrap();
        
        let alpha = noise_height.get_pixel(current_pos.0 as u32, current_pos.1 as u32).0[0] as f64/255.;
        let min_rad_current_pos = param.max_distance - (alpha-param.threshold_min)/(param.threshold_max-param.threshold_min) * (param.max_distance-param.min_distance);        
        
        // let mut to_remove = true;
        for _ in 0..param.k {
            let r_angle = rng.gen_range(0. .. 2.*PI);
            let r_distance = rng.gen_range(min_rad_current_pos .. 2.*min_rad_current_pos);
            let r_pos = (current_pos.0 + f64::cos(r_angle)*r_distance, current_pos.1 + f64::sin(r_angle)*r_distance);
            
            if r_pos.0 < 0. || r_pos.0 >= size_noise.0 as f64 || r_pos.1 < 0. || r_pos.1 >= size_noise.1 as f64 {
                continue;
            }

            let mut alpha = noise_height.get_pixel(r_pos.0 as u32, r_pos.1 as u32).0[0] as f64/255.;
            if alpha < param.threshold_min {
                continue;
            }
            if alpha > param.threshold_max {
                alpha = param.threshold_max;
            }

            let min_rad = param.max_distance - (alpha-param.threshold_min)/(param.threshold_max-param.threshold_min) * (param.max_distance-param.min_distance);

            let ind_grid = ((r_pos.0/tile_size) as u32, (r_pos.1/tile_size) as u32);

            let mut check_indices: Vec<(u32, u32)> = Vec::new();
            let nb_tile_check = (f64::ceil(min_rad/param.min_distance)*param.min_distance/tile_size).ceil() as u32;
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

            if min_distance.is_none() || min_distance.is_some_and(|d| d > min_rad) {
                let already = grid.insert(ind_grid, r_pos);
                if already.is_some() {
                    println!("had already");
                }
                active_list.push(r_pos);
                // to_remove = false;
            }
        }
        // if to_remove {
        if true {
            active_list.remove(r_ind);
        }

        if active_list.is_empty() {
            break;
        }
    }


    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = noise_height.convert();
    for (_, p) in grid.iter(){
        draw_filled_circle_mut(&mut image, (p.0 as i32, p.1 as i32), 2, Rgb([0,255,0]));
    }
    display_image("image", &image, size_noise.0, size_noise.1);
}