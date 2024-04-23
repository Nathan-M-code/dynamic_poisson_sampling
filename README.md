# dynamic_poisson_sampling
A slow poisson disk sampling with dynamic distance written in Rust.

# Example
```
let bounds = (0., 0., size_noise.0 as f64, size_noise.1 as f64);
let points = get_points(12, (size_noise.0 as f64/2f64, size_noise.1 as f64/2f64).into(), &mut rng, |pos|{
    if pos[0] < bounds.0 || pos[0] >= bounds.0+bounds.2 || pos[1] < bounds.1 || pos[1] >= bounds.1+bounds.3 {
        None
    }else{
        Some(3. + noise_height.get_pixel(pos[0] as u32, pos[1] as u32).0[0] as f64/255. * 10.)
    }
});
```
# Result
![Alt text](img/result.png?raw=true "Result")