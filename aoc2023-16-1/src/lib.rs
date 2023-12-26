enum Direction {
    East,
    North,
    West,
    South,
}

struct Ray {
    row: usize,
    col: usize,
    direction: Direction,
    dead: bool,
}

impl Ray {
    pub fn alive(&self) -> bool {
        !self.dead
    }

    pub fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            row,
            col,
            direction,
            dead: false,
        }
    }

    pub fn kill(&mut self) {
        self.dead = true;
    }

    pub fn redirect(&mut self, device: char) -> Option<Self> {
        match self.direction {
            Direction::East => match device {
                '/' => {
                    self.direction = Direction::North;
                    None
                }
                '\\' => {
                    self.direction = Direction::South;
                    None
                }
                '|' => {
                    self.direction = Direction::North;
                    Some(self.split(Direction::South))
                }
                _ => None,
            },
            Direction::North => match device {
                '/' => {
                    self.direction = Direction::East;
                    None
                }
                '\\' => {
                    self.direction = Direction::West;
                    None
                }
                '-' => {
                    self.direction = Direction::East;
                    Some(self.split(Direction::West))
                }
                _ => None,
            },
            Direction::West => match device {
                '/' => {
                    self.direction = Direction::South;
                    None
                }
                '\\' => {
                    self.direction = Direction::North;
                    None
                }
                '|' => {
                    self.direction = Direction::North;
                    Some(self.split(Direction::South))
                }
                _ => None,
            },
            Direction::South => match device {
                '/' => {
                    self.direction = Direction::West;
                    None
                }
                '\\' => {
                    self.direction = Direction::East;
                    None
                }
                '-' => {
                    self.direction = Direction::East;
                    Some(self.split(Direction::West))
                }
                _ => None,
            },
        }
    }

    pub fn split(&self, direction: Direction) -> Self {
        Self::new(self.row, self.col, direction)
    }
}

struct Contraption {
    layout: Vec<Vec<char>>,
    visited_from_east: Vec<Vec<bool>>,
    visited_from_north: Vec<Vec<bool>>,
    visited_from_west: Vec<Vec<bool>>,
    visited_from_south: Vec<Vec<bool>>,
    rays: Vec<Ray>,
}

impl<I: Iterator<Item = String>> From<I> for Contraption {
    fn from(value: I) -> Self {
        let mut layout = Vec::<Vec<char>>::new();
        let mut visited = Vec::new();
        for line in value {
            layout.push(line.chars().into_iter().collect());
            visited.push(vec![false; line.len()]);
        }

        let mut visited_from_west = visited.clone();
        visited_from_west[0][0] = true;

        let mut init_ray = Ray::new(0, 0, Direction::East);
        let mut rays = Vec::new();
        if let Some(ray_2) = init_ray.redirect(layout[0][0]) {
            rays.push(ray_2);
        }
        rays.push(init_ray);

        Self {
            layout,
            visited_from_east: visited.clone(),
            visited_from_north: visited.clone(),
            visited_from_west,
            visited_from_south: visited,
            rays,
        }
    }
}

impl Contraption {
    pub fn num_energized_tiles(&self) -> usize {
        let mut count = 0;
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.visited_from_east[i][j]
                    || self.visited_from_north[i][j]
                    || self.visited_from_west[i][j]
                    || self.visited_from_south[i][j]
                {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn trace(&mut self) -> bool {
        let width = self.width();
        let height = self.height();
        let mut new_rays = Vec::new();
        for ray in self.rays.iter_mut() {
            match ray.direction {
                Direction::East => {
                    if ray.col == width - 1 {
                        ray.kill();
                        continue;
                    }
                    ray.col += 1;
                    if self.visited_from_west[ray.row][ray.col] {
                        // Some ray has already played this out
                        ray.kill();
                        continue;
                    }
                    self.visited_from_west[ray.row][ray.col] = true;
                }
                Direction::North => {
                    if ray.row == 0 {
                        ray.kill();
                        continue;
                    }
                    ray.row -= 1;
                    if self.visited_from_south[ray.row][ray.col] {
                        // Some ray has already played this out
                        ray.kill();
                        continue;
                    }
                    self.visited_from_south[ray.row][ray.col] = true;
                }
                Direction::West => {
                    if ray.col == 0 {
                        ray.kill();
                        continue;
                    }
                    ray.col -= 1;
                    if self.visited_from_east[ray.row][ray.col] {
                        // Some ray has already played this out
                        ray.kill();
                        continue;
                    }
                    self.visited_from_east[ray.row][ray.col] = true;
                }
                Direction::South => {
                    if ray.row == height - 1 {
                        ray.kill();
                        continue;
                    }
                    ray.row += 1;
                    if self.visited_from_north[ray.row][ray.col] {
                        // Some ray has already played this out
                        ray.kill();
                        continue;
                    }
                    self.visited_from_north[ray.row][ray.col] = true;
                }
            }

            if let Some(new_ray) = ray.redirect(self.layout[ray.row][ray.col]) {
                new_rays.push(new_ray);
            }
        }

        self.rays.append(&mut new_rays);
        self.rays.retain(Ray::alive);
        !self.rays.is_empty()
    }

    fn height(&self) -> usize {
        self.layout.len()
    }

    fn width(&self) -> usize {
        self.layout[0].len()
    }
}

pub fn num_energized_tiles(it: impl Iterator<Item = String>) -> usize {
    let mut contraption = Contraption::from(it);
    while contraption.trace() {}
    contraption.num_energized_tiles()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#};
        assert_eq!(num_energized_tiles(example.lines().map(String::from)), 46);
    }
}
