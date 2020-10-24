#[aoc_generator(day5)]
fn generate_input(input: &str) -> Vec<i64> {
    let mut maze = Vec::<i64>::new();
    for line in input.lines() {
        let value = line.parse::<i64>().unwrap();
        maze.push(value);
    }
    return maze;
}

#[aoc(day5, part1)]
fn solve_part_1(input: &Vec<i64>) -> u64 {
    let mut maze = input.to_vec();
    let mut current_index: usize = 0;
    let mut steps_taken: u64 = 0;
    loop {
        steps_taken += 1;
        // Determine size of next jump
        let jump_size = maze[current_index];
        // Determine if next jump would exit the maze
        if (jump_size < 0 && (jump_size.abs() as usize) > current_index) ||
            (jump_size as usize + current_index) >= maze.len() {
                return steps_taken;
        }
        // Increment instruction at current index
        maze[current_index] += 1;
        // Conduct next jump if remaining within maze
        if jump_size < 0 {
            current_index -= jump_size.abs() as usize;
        } else {
            current_index += jump_size as usize;
        }
    }
    return steps_taken;
}

#[aoc(day5, part2)]
fn solve_part_2(input: &Vec<i64>) -> u64 {
    unimplemented!();
}
