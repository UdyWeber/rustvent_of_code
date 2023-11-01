use std::fs;

pub fn day_three_resolution() -> usize {
    let file_read = fs::read_to_string("src/day_three/my_input.txt").unwrap();
    let lines_vec: Vec<&str> = file_read.lines().collect();

    let mut sum = 0;

    for i in 0..lines_vec.len() / 3 {
        let group = &lines_vec[i * 3..(i + 1) * 3];

        let common: char = group[0]
            .chars()
            .fold(None, |optional_char, char| {
                if group[1].chars().any(|char_two| char == char_two)
                    && group[2].chars().any(|char_tree| char == char_tree) {
                    return Some(char);
                }

                optional_char
            },
            )
            .unwrap();

        let chars_to_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let idx = chars_to_map
            .chars()
            .position(|c| c == common)
            .unwrap();

        sum += idx + 1;
    }

    sum
}

