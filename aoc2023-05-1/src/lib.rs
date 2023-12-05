use std::sync::OnceLock;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SourceRange {
    start: i32,
    end: i32,
}

impl SourceRange {
    pub fn has(&self, source: i32) -> bool {
        self.start <= source && source <= self.end
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    range: SourceRange,
    offset: i32,
}

impl Mapping {
    pub fn get(&self, source: i32) -> Option<i32> {
        if self.range.has(source) {
            return Some(source + self.offset);
        }
        None
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        static SPEC: OnceLock<Regex> = OnceLock::new();
        let captures = SPEC
            .get_or_init(|| Regex::new(r"^(?<dest>\d+) (?<source>\d+) (?<length>\d+)$").unwrap())
            .captures(value)
            .unwrap();
        let start = captures.name("source").unwrap().as_str().parse().unwrap();
        let end = start
            + captures
                .name("length")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap()
            - 1;
        let offset = captures
            .name("dest")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap()
            - start;

        Self {
            range: SourceRange { start, end },
            offset,
        }
    }
}

#[derive(Debug, Default)]
/// ordered list of mappings
struct Map(Vec<Mapping>);

impl Map {
    pub fn push(&mut self, mapping: Mapping) {
        self.0.push(mapping);
        self.0.sort();
    }

    pub fn get(&self, source: i32) -> i32 {
        for mapping in self.0.iter() {
            if let Some(destination) = mapping.get(source) {
                return destination;
            }
        }
        return source;
    }
}

fn get_seeds(line: &str) -> Vec<i32> {
    static SEEDS: OnceLock<Regex> = OnceLock::new();
    SEEDS
        .get_or_init(|| Regex::new(r"\d+").unwrap())
        .find_iter(line)
        .filter_map(|s| s.as_str().parse().ok())
        .collect()
}

/// get a single Map from input stream
fn get_map(it: &mut impl Iterator<Item = String>) -> Map {
    static PREAMBLE: OnceLock<Regex> = OnceLock::new();
    let preamble = PREAMBLE.get_or_init(|| Regex::new(r"^[^\d]*$").unwrap());

    // throw away preamble
    while let Some(line) = it.next() {
        if !preamble.is_match(line.as_str()) {
            break;
        }
    }

    let mut map = Map::default();
    while let Some(line) = it.next() {
        if preamble.is_match(line.as_str()) {
            break;
        }
        map.push(Mapping::from(line.as_str()));
    }
    map
}

fn get_maps(mut it: impl Iterator<Item = String>) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut it = it.peekable();

    while let Some(_) = it.peek() {
        maps.push(get_map(&mut it));
    }

    maps
}

pub fn nearest_seed_location(mut it: impl Iterator<Item = String>) -> i32 {
    let seeds = get_seeds(it.next().unwrap().as_str());
    let maps = get_maps(it);

    dbg!(maps);

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc!{"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        assert_eq!(nearest_seed_location(example.lines().map(String::from)), 35)
    }
}
