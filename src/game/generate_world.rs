use bevy::reflect::Reflect;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct PerlinNoiseParams {
    pub scale: f64,
    pub max: i8,
    pub min: i8,
    pub weights: Vec<u8>,
}

fn generate_perlin_noise_elevation_ranges(min: i8, weights: &Vec<u8>) -> Vec<(f64, f64, i8)> {
    let mut ranges = Vec::new();
    let weights_sum: u64 = weights.iter().map(|&x| x as u64).sum();
    let mut end: f64 = -1.;
    for i in 0..weights.len() {
        let start = end;
        end = if i < weights.len() {
            end + (*weights.get(i).unwrap() as f64 / weights_sum as f64) * 2.
        } else {
            2.
        };
        ranges.push((start, end, min + i as i8))
    }
    ranges
}

pub fn generate_elevation_map_with_perlin_noise(
    seed: Option<u32>,
    width: u8,
    height: u8,
    params: PerlinNoiseParams,
) -> Result<Vec<Vec<i8>>, String> {
    if width == 0 {
        return Err(String::from("width cannot be zero"));
    }
    if height == 0 {
        return Err(String::from("height cannot be zero"));
    }
    if params.min > params.max {
        return Err(String::from("min cannot be greater than max"));
    }
    if (params.max - params.min + 1) as usize != params.weights.len() {
        return Err(String::from(
            "Number of weights does not match the difference between max and min",
        ));
    }
    let mut map = vec![vec![0; height as usize]; width as usize];

    let perlin = Perlin::new(if let Some(seed) = seed {
        seed
    } else {
        rand::thread_rng().gen::<u32>()
    });

    let ranges = generate_perlin_noise_elevation_ranges(params.min, &params.weights);
    println!("ranges:{:#?}", ranges);
    for x in 0..width {
        for y in 0..height {
            let nx = x as f64 / params.scale;
            let ny = y as f64 / params.scale;
            let value = perlin.get([nx, ny]);
            for range in &ranges {
                if range.0 <= value && value < range.1 {
                    map.get_mut(x as usize)
                        .unwrap()
                        .get_mut(y as usize)
                        .unwrap()
                        .set(Box::new(range.2))
                        .unwrap();
                    break;
                }
            }
        }
    }

    Ok(map)
}
