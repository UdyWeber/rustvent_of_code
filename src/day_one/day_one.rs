use std::fs;

pub fn day_one_resolution() -> u64 {
    let (sum, mut elves_sums)= fs::read_to_string("src/day_one/my_input.txt").unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap_or(0))
        .fold(
            (0, Vec::new()),
            |(mut sum, mut maxes), value| {
                if value == 0 && sum != 0 {
                    maxes.push(sum);
                    (0, maxes)
                } else {
                    sum += value;
                    (sum, maxes)
                }
            },
        );

    if sum != 0 {
        elves_sums.push(sum);
    }

    elves_sums.sort();

    let mut top_three_sum = 0;

    for _ in 0..3 {
        top_three_sum += elves_sums.pop().unwrap();
    }

    top_three_sum
}
