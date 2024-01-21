use bevy::reflect::Reflect;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub fn generate_elevation_map_with_perlin_noise(
    seed: Option<u32>,
    width: u8,
    height: u8,
    scale: f64,
    min: i8,
    max: i8,
    weights: &Vec<u8>,
) -> Result<Vec<Vec<i8>>, String> {
    if width == 0 {
        return Err(String::from("width cannot be zero"));
    }
    if height == 0 {
        return Err(String::from("height cannot be zero"));
    }
    if min > max {
        return Err(String::from("min cannot be greater than max"));
    }
    if (max - min + 1) as usize != weights.len() {
        return Err(String::from(
            "Number of weights does not match the difference between max and min",
        ));
    }
    let mut map = vec![vec![0; height as usize]; width as usize];

    let seed = if let Some(seed) = seed {
        seed
    } else {
        rand::thread_rng().gen::<u32>()
    };

    let noise = generate_perlin_noise(seed, width as usize, height as usize, scale);

    let ranges = generate_perlin_noise_elevation_ranges(min, &weights);
    for x in 0..width {
        for y in 0..height {
            let value = noise[x as usize][y as usize];
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

fn generate_perlin_noise(seed: u32, width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(seed);

    let mut noise_map = vec![vec![0.0; height]; width];

    for x in 0..width {
        for y in 0..height {
            let nx = x as f64 / scale;
            let ny = y as f64 / scale;
            noise_map[x][y] = perlin.get([nx, ny]);
        }
    }

    noise_map
}

fn generate_perlin_noise_elevation_ranges(min: i8, weights: &Vec<u8>) -> Vec<(f64, f64, i8)> {
    let mut ranges = Vec::new();
    let weights_sum: u64 = weights.iter().map(|&x| x as u64).sum();
    let mut end: f64 = -1.;
    for i in 0..weights.len() {
        let start = end;
        end = if i < weights.len() - 1 {
            end + (*weights.get(i).unwrap() as f64 / weights_sum as f64) * 2.
        } else {
            f64::MAX
        };
        ranges.push((start, end, min + i as i8))
    }
    ranges
}
