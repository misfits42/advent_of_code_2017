/// Calculates the knot hash of the given input using the algorithm developed in in AoC 2017 Day 10,
/// Part 2.
pub fn calculate_knot_hash(input: &str) -> Vec<u8> {
    // Read input in as ASCII characters and convert into array of lengths for processing
    let mut lengths = input.chars().map(|x| x as u8).collect::<Vec<u8>>();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    // Initialise list, current position and skip size
    let mut list = (0..=255).collect::<Vec<u8>>();
    let mut current_pos: usize = 0;
    let mut skip_size: usize = 0;
    // Perform 64 iterations of sparse hash calculation
    for _ in 0..64 {
        list = calculate_sparse_hash(&list, &lengths, &mut current_pos, &mut skip_size);
    }
    // Calculate dense hash
    let dense_hash = calculate_dense_hash(&list);
    return dense_hash;
}   

/// Performs a single round of the sparse hash (first stage of knot hash) algorithm introduced in
/// AoC 2017 Day 10, Part 1.
pub fn calculate_sparse_hash(list: &Vec<u8>, lengths: &Vec<u8>, current_pos: &mut usize, skip_size: &mut usize) -> Vec<u8>{
    let mut sparse_hash = list.clone();
    for length in lengths {
        let length = *length as usize;
        // Calculate list of indices included in the reverse
        let mut rev_indices = Vec::<usize>::new();
        for i in 0..length {
            let index = (*current_pos + i) % sparse_hash.len();
            &rev_indices.push(index);
        }
        // Extract values from the list then reverse
        let mut extract_list = Vec::<u8>::new();
        for i in &rev_indices {
            extract_list.push(sparse_hash[*i]);
        }
        extract_list.reverse();
        // Replace values in list with reversed values
        for i in 0..extract_list.len() {
            let list_index = rev_indices[i];
            sparse_hash[list_index] = extract_list[i]
        }
        // Move the current positon
        *current_pos = (*current_pos + length + *skip_size) % sparse_hash.len();
        // Increase skip size by 1
        *skip_size += 1;
    }
    return sparse_hash;
}

/// Calculates the dense hash of the output from the sparse hash calculation phase of the knot hash
/// algorithm. Introduced in AoC 2017 Day 10, Part 2.
fn calculate_dense_hash(list: &Vec<u8>) -> Vec<u8> {
    let mut dense_hash: Vec<u8> = vec![0; 16];
    for group in 0..16 {
        let start = group * 16;
        // Calculate the result of XOR'ing the 16 values in the current group
        let mut xor_res: u8 = 0;
        for i in 0..16 {
            xor_res ^= list[start + i];
        }
        dense_hash[group] = xor_res;
    }
    return dense_hash;
}
