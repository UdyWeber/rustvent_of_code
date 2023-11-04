use std::fs;

#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}


impl Section {
    pub fn new(from_string: &str) -> Self {
        let (start, end) = from_string.split_once("-").unwrap();

        Self {
            start: start.parse::<u32>().unwrap(),
            end: end.parse::<u32>().unwrap(),
        }
    }

    pub fn contains_or_is_contained(&self, s: &Section) -> bool {
        let (contains_range, is_contained_range)
            = ((self.start..=self.end), (s.start..=s.end));

        contains_range.contains(&s.start)
            || contains_range.contains(&s.end)
            || is_contained_range.contains(&self.start)
            || is_contained_range.contains(&self.end)
    }
}

pub fn day_four_resolution() {
    let file_read = fs::read_to_string("src/day_four/my_input.txt").unwrap();

    let overlapped_sections = file_read
        .lines()
        .map(|line| {
            let (first_section, second_section) = line.split_once(",").unwrap();
            (Section::new(first_section), Section::new(second_section))
        })
        .fold(0, |mut sum, (first_section, second_section)| {
            if first_section.contains_or_is_contained(&second_section) {
                sum += 1;
            }

            return sum;
        });

    println!("{overlapped_sections}")
}