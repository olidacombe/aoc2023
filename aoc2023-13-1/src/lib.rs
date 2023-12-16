use std::mem;

#[derive(Default)]
struct Pattern {
    pixels: Vec<Vec<char>>,
    pixels_t: Vec<Vec<char>>,
}

impl From<Vec<Vec<char>>> for Pattern {
    fn from(value: Vec<Vec<char>>) -> Self {
        let rows = value.len();
        let cols = value.first().unwrap().len();
        let mut transpose = Vec::new();
        for col in 0..cols {
            let mut new_col = Vec::new();
            for row in 0..rows {
                new_col.push(value[row][col]);
            }
            transpose.push(new_col);
        }
        Pattern {
            pixels: value,
            pixels_t: transpose,
        }
    }
}

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Default for Reflection {
    fn default() -> Self {
        Reflection::Horizontal(0)
    }
}

impl Into<usize> for Reflection {
    fn into(self) -> usize {
        match self {
            Reflection::Horizontal(h) => 100 * h,
            Reflection::Vertical(v) => v,
        }
    }
}

fn find_reflection(input: &Vec<Vec<char>>) -> Option<usize> {
    for i in 1..input.len() {
        if (0..i).all(|j| {
            if i + j >= input.len() {
                return true;
            }
            input[i - j - 1] == input[i + j]
        }) {
            return Some(i);
        }
    }
    None
}

impl Pattern {
    fn find_reflection(&self) -> Reflection {
        find_reflection(&self.pixels)
            .map(|i| Reflection::Horizontal(i))
            .unwrap_or_else(|| {
                find_reflection(&self.pixels_t)
                    .map(|i| Reflection::Vertical(i))
                    .unwrap()
            })
    }

    fn reflection_score(&self) -> usize {
        self.find_reflection().into()
    }
}

fn parse_input(it: impl Iterator<Item = String>) -> Vec<Pattern> {
    let mut patterns = Vec::<Pattern>::new();
    let mut pixels = Vec::new();
    for line in it {
        if line == "" {
            let pixels = mem::take(&mut pixels);
            patterns.push(pixels.into());
            continue;
        }
        pixels.push(line.chars().collect());
    }
    patterns.push(pixels.into());
    patterns
}

pub fn reflection_summary(it: impl Iterator<Item = String>) -> usize {
    let patterns = parse_input(it);
    patterns
        .iter()
        .map(|pattern| pattern.reflection_score())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};
        assert_eq!(reflection_summary(example.lines().map(String::from)), 405);
    }
}
