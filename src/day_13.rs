use std::collections::HashMap;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum ScanDirection {
    Up,
    Down
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct FirewallLayer {
    range: u64,
    scan_pos: u64,
    scan_dir: ScanDirection
}

impl FirewallLayer {
    /// Creates a new FirewallLayer with specified depth and scanner starting at position 0.
    pub fn new(range: u64) -> Self {
        Self {
            range: range,
            scan_pos: 0,
            scan_dir: ScanDirection::Down
        }
    }

    /// Moves the location of the scanner by one position, changing direction as needed when top or
    /// bottom of layer is reached.
    pub fn move_scanner(&mut self) {
        // Check if scanner direction needs to be changed
        if self.scan_pos == 0 && self.scan_dir == ScanDirection::Up {
            self.scan_dir = ScanDirection::Down;
        } else if self.scan_pos == self.range - 1 && self.scan_dir == ScanDirection::Down {
            self.scan_dir = ScanDirection::Up;
        }
        // Move the scanner
        match self.scan_dir {
            ScanDirection::Down => self.scan_pos += 1,
            ScanDirection::Up => self.scan_pos -= 1
        }
    }

    /// Checks if the specified position is the same as the current position of the scanner.
    pub fn check_collision(&self, check_pos: u64) -> bool {
        return self.scan_pos == check_pos;
    }

    /// Gets the range of the FirewallLayer - i.e. the total number of position throuch which the
    /// scanner may move in the layer.
    pub fn get_range(&self) -> u64 {
        return self.range;
    }
}

#[aoc_generator(day13)]
fn generate_input(input: &str) -> HashMap<u64, FirewallLayer> {
    // Trim whitespace from start and end of input
    let input = input.trim();
    // Initialise record of all firewall layers read from input
    let mut firewall_layers = HashMap::<u64, FirewallLayer>::new();
    // Create regex to match firewall layer depth and range
    let layer_regex = Regex::new(r"(\d+): (\d+)").unwrap();
    // Treat each line of input as another firewall layer
    for layer_raw in input.lines() {
        let layer_raw = layer_raw.trim();
        for capture in layer_regex.captures_iter(layer_raw) {
            // Extract fields from line and create new layer
            let depth = capture[1].parse::<u64>().unwrap();
            let range = capture[2].parse::<u64>().unwrap();
            let firewall_layer = FirewallLayer::new(range);
            firewall_layers.insert(depth, firewall_layer);
        }
    }
    return firewall_layers;
}

#[aoc(day13, part1)]
fn solve_part_1(input: &HashMap<u64, FirewallLayer>) -> u64 {
    let mut firewall_layers = input.clone();
    let mut total_severity = 0;
    let packet_pos = 0;
    // Initialise starting depth and determine depth at which packet has exited the firewall
    let mut current_depth = 0;
    let mut exit_depth = input.keys().max().unwrap() + 1;
    while current_depth < exit_depth {
        // Check for collision at current depth
        if firewall_layers.contains_key(&current_depth) {
            let layer = firewall_layers.get(&current_depth).unwrap();
            if layer.check_collision(packet_pos) {
                total_severity += current_depth * layer.get_range();
            }
        }
        // Move scanner in each firewall layer with non-zero range
        for (_depth, layer) in firewall_layers.iter_mut() {
            layer.move_scanner();
        }
        // Move packet to next depth
        current_depth += 1;
    }
    return total_severity;
}

#[aoc(day13, part2)]
fn solve_part_2(input: &HashMap<u64, FirewallLayer>) -> u64 {
    unimplemented!();
}
