

use imageproc::{window::display_image, drawing::draw_filled_circle_mut};
use noise::{permutationtable::PermutationTable, utils::{PlaneMapBuilder, NoiseMapBuilder, NoiseMap}, core::perlin::perlin_2d};

use image::{ImageBuffer, Luma, Rgb, buffer::ConvertBuffer};

use rand::Rng;

use dynamic_poisson_sampling::*;

pub fn noise_map_to_image_buffer(nm: &NoiseMap) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    // collect the values from f64 into u8 in a separate vec
    let (width, height) = nm.size();
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

    for v in nm.iter(){
        pixels.push(((v * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8);
    }

    ImageBuffer::from_raw(width as u32, height as u32, pixels).unwrap()
}

fn main() {

    let mut rng = rand::thread_rng();

    let size_noise: (u32, u32) = (500,500);
    let freq_noise = 3.;
    

    let hasher = PermutationTable::new(rng.gen());
    let nm = PlaneMapBuilder::new_fn(|point, hasher| perlin_2d(point.into(), hasher), &hasher)
        .set_size(size_noise.0 as usize, size_noise.1 as usize)
        .set_x_bounds(0., freq_noise)
        .set_y_bounds(0., freq_noise)
        .build();
    let noise_height: ImageBuffer<Luma<u8>, Vec<u8>> = noise_map_to_image_buffer(&nm);
    // display_image("noise_height", &noise_height, size_noise.0, size_noise.1);


    let bounds = (0., 0., size_noise.0 as f64, size_noise.1 as f64);
    let points = get_points(12, (size_noise.0 as f64/2f64, size_noise.1 as f64/2f64).into(), &mut rng, |pos|{
        if pos[0] < bounds.0 || pos[0] >= bounds.0+bounds.2 || pos[1] < bounds.1 || pos[1] >= bounds.1+bounds.3 {
            None
        }else{
            Some(3. + noise_height.get_pixel(pos[0] as u32, pos[1] as u32).0[0] as f64/255. * 10.)
        }
    });


    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = noise_height.convert();
    for p in points.iter(){
        draw_filled_circle_mut(&mut image, (p[0] as i32, p[1] as i32), 2, Rgb([0,255,0]));
    }
    display_image("image", &image, size_noise.0, size_noise.1);
}