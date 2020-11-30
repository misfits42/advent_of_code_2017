use super::utils::map::Particle3D;

use enum_iterator::IntoEnumIterator;
use regex::Regex;
use std::collections::HashMap;

/// Categorises the rate at which the Manhattan distance of particles from (0,0,0) is changing
/// compared to previous tick.
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, IntoEnumIterator)]
enum DistanceGroup {
    IncreasingHigher,
    IncreasingSame,
    IncreasingLower,
    Same,
    DecreasingLower,
    DecreasingSame,
    DecreasingHigher
}

impl DistanceGroup {
    /// Determines the applicable DistanceGroup from the provided distance deltas.
    pub fn determine_distance_group(deltas: (i64, i64)) -> DistanceGroup {
        if deltas.1 > 0 { // Particle getting further away from (0, 0, 0)
            if deltas.1 > deltas.0 {
                return DistanceGroup::IncreasingHigher;
            } else if deltas.1 == deltas.0 {
                return DistanceGroup::IncreasingSame;
            } else {
                return DistanceGroup::IncreasingLower;
            }
        } else if deltas.1 == 0 { // Particle is same distance from (0, 0, 0)
            return DistanceGroup::Same;
        } else { // deltas.1 < 0 --- Particle getting closer to (0, 0, 0)
            if deltas.1 > deltas.0 {
                return DistanceGroup::DecreasingLower;
            } else if deltas.1 == deltas.0 {
                return DistanceGroup::DecreasingSame;
            } else {
                return DistanceGroup::DecreasingHigher;
            }
        }
    }
}

#[aoc_generator(day20)]
fn generate_input(input: &str) -> HashMap<u64, Particle3D> {
    let mut particles: HashMap<u64, Particle3D> = HashMap::new();
    let mut particle_num: u64 = 0;
    // Create regex to extract particle details from each line
    let particle_regex = Regex::new(
        r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>",
    ).unwrap();
    // Process each line with particle details
    for line in input.lines() {
        // Remove all leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Conduct regex match and capture extraction
        let captures = particle_regex.captures(line).unwrap();
        let mut vals: Vec<i64> = vec![];
        for i in 1..=9 {
            vals.push(captures[i].parse::<i64>().unwrap());
        }
        let pos = (vals[0], vals[1], vals[2]);
        let vel = (vals[3], vals[4], vals[5]);
        let acc = (vals[6], vals[7], vals[8]);
        let particle = Particle3D::new(pos, vel, acc);
        particles.insert(particle_num, particle);
        particle_num += 1;
    }
    return particles;
}

#[aoc(day20, part1)]
fn solve_part_1(particles: &HashMap<u64, Particle3D>) -> u64 {
    let mut particles = particles.clone();
    let mut total_ticks = 0;
    // Record the distances of particles from origin after each tick
    let mut manhattan_dists: HashMap<u64, Vec<i64>> = HashMap::new();
    for num in particles.keys() {
        manhattan_dists.insert(*num, vec![]);
    }
    loop {
        // Conduct tick
        for (num, particle) in particles.iter_mut() {
            particle.update_pos_and_vel();
            let manhattan_dist = particle.get_manhattan_distance((0, 0, 0));
            manhattan_dists.get_mut(&num).unwrap().push(manhattan_dist);
        }
        // Determine distance group for the particle based on change in distance from (0,0,0)
        if total_ticks >= 4 {
            let mut distance_groups = generate_distance_group_records();
            for (num, dists) in manhattan_dists.iter() {
                // Calculate deltas and distance group
                let dist_deltas = calculate_distance_deltas(dists);
                let distance_group = DistanceGroup::determine_distance_group(dist_deltas);
                distance_groups.get_mut(&distance_group).unwrap().push(*num);
            }
            // Check if all particles have a increasing rate of movement away from origin
            if check_particles_for_only_increasing_distance(particles.len(), &distance_groups) {
                if distance_groups.get(&DistanceGroup::IncreasingSame).unwrap().len() == 1 {
                    return distance_groups.get(&DistanceGroup::IncreasingSame).unwrap()[0];
                }
            }
        }
        total_ticks += 1;
    }
}

#[aoc(day20, part2)]
fn solve_part_2(particles: &HashMap<u64, Particle3D>) -> u64 {
    let mut particles = particles.clone();
    let mut total_ticks = 0;
    // Record the distances of particles from origin after each tick
    let mut manhattan_dists: HashMap<u64, Vec<i64>> = HashMap::new();
    for num in particles.keys() {
        manhattan_dists.insert(*num, vec![]);
    }
    loop {
        // Record position of each particle after tick
        let mut particle_positions: HashMap<(i64, i64, i64), Vec<u64>> = HashMap::new();
        // Conduct tick
        for (num, particle) in particles.iter_mut() {
            particle.update_pos_and_vel();
            // Record resulting position of particle
            let pos = particle.get_position_3d();
            if particle_positions.contains_key(&pos) {
                particle_positions.get_mut(&pos).unwrap().push(*num);
            } else {
                particle_positions.insert(pos, vec![*num]);
            }
            // Record resulting Manhattan distance of particle from origin
            let manhattan_dist = particle.get_manhattan_distance((0, 0, 0));
            manhattan_dists.get_mut(&num).unwrap().push(manhattan_dist);
        }
        // Remove particles that have collided from particle list and manhattan distance records
        for (_pos, nums) in particle_positions {
            if nums.len() > 1 {
                for num in nums {
                    particles.remove(&num);
                    manhattan_dists.remove(&num);
                }
            }
        }
        // Determine distance group for the particle based on change in distance from (0,0,0)
        if total_ticks >= 4 {
            let mut distance_groups = generate_distance_group_records();
            for (num, dists) in manhattan_dists.iter() {
                // Calculate deltas and distance group
                let dist_deltas = calculate_distance_deltas(dists);
                let distance_group = DistanceGroup::determine_distance_group(dist_deltas);
                distance_groups.get_mut(&distance_group).unwrap().push(*num);
            }
            // Assume no more collisions if remaining particles are all moving away from (0,0,0)
            if check_particles_for_only_increasing_distance(particles.len(), &distance_groups) {
                return particles.len() as u64;
            }
        }
        total_ticks += 1;
    }
}

/// Checks if the particle distance groups are only increasing at the same or higher rate.
fn check_particles_for_only_increasing_distance(total_particles: usize,
        distance_groups: &HashMap<DistanceGroup, Vec<u64>>) -> bool
{
    let mut count = 0;
    count += distance_groups.get(&DistanceGroup::IncreasingHigher).unwrap().len();
    count += distance_groups.get(&DistanceGroup::IncreasingSame).unwrap().len();
    return count == total_particles;
}

/// Calculates the most recent two changes in distance.
fn calculate_distance_deltas(dists: &Vec<i64>) -> (i64, i64) {
    let len = dists.len();
    let delta1 = dists[len - 1] - dists[len - 2];
    let delta0 = dists[len - 2] - dists[len - 3];
    return (delta0, delta1);
}

/// Generates a HashMap with a count initialised to 0 for each variant of DistanceGroup.
fn generate_distance_group_records() -> HashMap<DistanceGroup, Vec<u64>> {
    let mut records: HashMap<DistanceGroup, Vec<u64>> = HashMap::new();
    for distance_group in DistanceGroup::into_enum_iter() {
        records.insert(distance_group, vec![]);
    }
    return records;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d20_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day20.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(376, result);
    }

    #[test]
    fn test_d20_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day20.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(574, result);
    }
}
