# variable_poisson_disk_sampling
A not so fast variable density poisson disk sampling in Rust

# Result
```
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


let param = Param{
    k: 25,
    threshold_min: 0.3,
    threshold_max: 1.,
    min_distance: 5.,
    max_distance: 24.,
};
```
![Alt text](img/ss.png?raw=true "Result")