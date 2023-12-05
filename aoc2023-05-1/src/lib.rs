use std::{iter::Peekable, sync::OnceLock};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SourceRange {
    start: i64,
    end: i64,
}

impl SourceRange {
    pub fn has(&self, source: i64) -> bool {
        self.start <= source && source <= self.end
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    range: SourceRange,
    offset: i64,
}

impl Mapping {
    pub fn get(&self, source: i64) -> Option<i64> {
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
                .parse::<i64>()
                .unwrap()
            - 1;
        let offset = captures
            .name("dest")
            .unwrap()
            .as_str()
            .parse::<i64>()
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

    pub fn get(&self, source: i64) -> i64 {
        for mapping in self.0.iter() {
            if let Some(destination) = mapping.get(source) {
                return destination;
            }
        }
        return source;
    }
}

#[derive(Debug, Default)]
struct Maps(Vec<Map>);

impl Maps {
    pub fn digest(&self, mut seed: i64) -> i64 {
        for map in self.0.iter() {
            seed = map.get(seed);
        }
        seed
    }
}

fn get_seeds(line: &str) -> Vec<i64> {
    static SEEDS: OnceLock<Regex> = OnceLock::new();
    SEEDS
        .get_or_init(|| Regex::new(r"\d+").unwrap())
        .find_iter(line)
        .filter_map(|s| s.as_str().parse().ok())
        .collect()
}

/// get a single Map from input stream
fn get_map<I>(it: &mut Peekable<I>) -> Map
where
    I: Iterator<Item = String>,
{
    static PREAMBLE: OnceLock<Regex> = OnceLock::new();
    let preamble = PREAMBLE.get_or_init(|| Regex::new(r"^[^\d]*$").unwrap());

    // throw away preamble
    while let Some(line) = it.peek() {
        if !preamble.is_match(line.as_str()) {
            break;
        }
        it.next();
    }

    let mut map = Map::default();
    while let Some(line) = it.peek() {
        if preamble.is_match(line.as_str()) {
            break;
        }
        map.push(Mapping::from(line.as_str()));
        it.next();
    }
    map
}

fn get_maps(it: impl Iterator<Item = String>) -> Maps {
    let mut maps = Vec::new();
    let mut it = it.peekable();

    while let Some(_) = it.peek() {
        maps.push(get_map(&mut it));
    }

    Maps(maps)
}

pub fn nearest_seed_location(mut it: impl Iterator<Item = String>) -> i64 {
    let seeds = get_seeds(it.next().unwrap().as_str());
    let maps = get_maps(it);
    seeds
        .iter()
        .map(|seed| maps.digest(seed.clone()))
        .min()
        .unwrap()
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
