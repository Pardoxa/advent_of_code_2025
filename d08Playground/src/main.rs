// Ich habe das hier erst versucht effizient zu lösen.
// Brute force reicht aus.
// Naja, ich habe keine Lust hier aufzuräumen,
// So bless this mess ~~~~~~~

use core::f64;
use std::{cmp::Reverse, collections::BTreeSet, i64, iter, num::NonZeroU32};

const LATICE_SIZE: i32 = 200;

fn main() {
    let input = include_str!("../puzzle_input.dat");

    puzzle2_brute_force(input);
}

#[derive(Clone, Copy, Debug)]
pub struct Junction {
    x: i32,
    y: i32,
    z: i32,
    group: Option<NonZeroU32>,
}

impl Junction {
    pub fn distance(&self, other: &Self) -> i64 {
        let x = (self.x - other.x) as i64;
        let y = (self.y - other.y) as i64;
        let z = (self.z - other.z) as i64;

        x * x + y * y + z * z
    }
}

pub fn puzzle1_brute_force(input: &str, connections: usize) {
    let mut junctions: Vec<_> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let z = iter.next().unwrap().parse().unwrap();

            Junction {
                x,
                y,
                z,
                group: None,
            }
        })
        .collect();

    let mut direct_connections: Vec<Vec<usize>> = junctions.iter().map(|_| Vec::new()).collect();

    let len = junctions.len();
    let mut group_counter = NonZeroU32::new(1).unwrap();

    for _ in 0..connections {
        let mut a = 0;
        let mut b = 0;
        let mut minimum = i64::MAX;

        for (i, connected) in direct_connections.iter().enumerate() {
            // Die Distanz ist symmetrisch
            for j in i + 1..len {
                if connected.contains(&j) {
                    continue;
                }
                let d = junctions[i].distance(&junctions[j]);
                if d < minimum {
                    minimum = d;
                    a = i;
                    b = j;
                }
            }
        }

        let group_a = junctions[a].group;
        let group_b = junctions[b].group;

        match (group_a, group_b) {
            (None, None) => {
                // No group yet, so a new group :)
                junctions[a].group = Some(group_counter);
                junctions[b].group = Some(group_counter);
                group_counter = group_counter.saturating_add(1);
            }
            (Some(group), None) | (None, Some(group)) => {
                junctions[a].group = Some(group);
                junctions[b].group = Some(group);
            }
            (Some(group_a), Some(group_b)) => {
                if group_a != group_b {
                    // Okay, we relabel the group
                    let min = group_a.min(group_b);
                    let max = group_a.max(group_b);
                    for j in junctions.iter_mut() {
                        if let Some(group) = j.group.as_mut()
                            && *group == max
                        {
                            *group = min;
                        }
                    }
                }
            }
        }
        direct_connections[a].push(b);
    }

    let max_curcuit = junctions.iter().filter_map(|e| e.group).max().unwrap();

    let mut sizes = Vec::new();

    for i in 1..=max_curcuit.get() {
        let group = NonZeroU32::new(i);
        let size = junctions.iter().filter(|item| item.group == group).count();
        sizes.push(size);
    }
    sizes.sort_by_key(|g| Reverse(*g));

    let product = sizes[0] * sizes[1] * sizes[2];

    println!("Product: {product}");
}

pub fn puzzle2_brute_force(input: &str) {
    let mut junctions: Vec<_> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let z = iter.next().unwrap().parse().unwrap();

            Junction {
                x,
                y,
                z,
                group: None,
            }
        })
        .collect();

    let mut direct_connections: Vec<BTreeSet<usize>> =
        junctions.iter().map(|_| BTreeSet::new()).collect();

    let len = junctions.len();
    let mut group_counter = NonZeroU32::new(1).unwrap();

    loop {
        let mut a = 0;
        let mut b = 0;
        let mut minimum = i64::MAX;

        for (i, connected) in direct_connections.iter().enumerate() {
            // Die Distanz ist symmetrisch
            for j in i + 1..len {
                if connected.contains(&j) {
                    continue;
                }
                let d = junctions[i].distance(&junctions[j]);
                if d < minimum {
                    minimum = d;
                    a = i;
                    b = j;
                }
            }
        }

        let group_a = junctions[a].group;
        let group_b = junctions[b].group;

        match (group_a, group_b) {
            (None, None) => {
                // No group yet, so a new group :)
                junctions[a].group = Some(group_counter);
                junctions[b].group = Some(group_counter);
                group_counter = group_counter.saturating_add(1);
            }
            (Some(group), None) | (None, Some(group)) => {
                junctions[a].group = Some(group);
                junctions[b].group = Some(group);
            }
            (Some(group_a), Some(group_b)) => {
                if group_a != group_b {
                    // Okay, we relabel the group

                    // we also check if we see any other group
                    let min = group_a.min(group_b);
                    let max = group_a.max(group_b);
                    for j in junctions.iter_mut() {
                        if let Some(group) = j.group.as_mut()
                            && *group == max
                        {
                            *group = min;
                        }
                    }
                }
            }
        }

        if junctions.iter().all(|j| j.group.is_some()) {
            let number_of_groups: BTreeSet<_> = junctions
                .iter()
                .map(|item| item.group.map_or_else(|| 0, NonZeroU32::get))
                .collect();
            println!("{}", number_of_groups.len());

            if number_of_groups.len() == 1 {
                let result = junctions[a].x as u64 * junctions[b].x as u64;
                println!("{result}");
                return;
            }
        }

        direct_connections[a].insert(b);
    }
}

pub fn puzzle1(input: &str) {
    let mut junctions: Vec<_> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let z = iter.next().unwrap().parse().unwrap();

            Junction {
                x,
                y,
                z,
                group: None,
            }
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for Junction { x, y, z, group: _ } in junctions.iter().copied() {
        max_x = x.max(max_x);
        max_y = y.max(max_y);
        max_z = z.max(max_z);
    }

    // Nun will ich meinen Grid bauen
    // Sagen wir mal, wir nehmen LATICE_SIZE für die Kantenlänge

    let x_tiles = max_x / LATICE_SIZE + 1;
    let y_tiles = max_y / LATICE_SIZE + 1;
    let z_tiles = max_z / LATICE_SIZE + 1;

    let mut grid = Grid::new(x_tiles, y_tiles, z_tiles);
    grid.extend(junctions);

    dbg!(&grid);
    grid.distances();
}

#[derive(Debug, Clone)]
pub struct Grid {
    // Inner is vector of all junctions in this grid space
    g: Vec<Vec<Vec<Vec<Junction>>>>,
    x_tiles: usize,
    y_tiles: usize,
    z_tiles: usize,
}

impl Grid {
    pub fn new(x_tiles: i32, y_tiles: i32, z_tiles: i32) -> Self {
        let g = (0..x_tiles)
            .map(|_| {
                (0..y_tiles)
                    .map(|_| {
                        (0..z_tiles)
                            .map(|_| Vec::<Junction>::new())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            g,
            x_tiles: x_tiles as usize,
            y_tiles: y_tiles as usize,
            z_tiles: z_tiles as usize,
        }
    }

    pub fn distances(&self) {
        for x in 0..self.x_tiles {
            for y in 0..self.y_tiles {
                for z in 0..self.z_tiles {
                    let junctions = &self.g[x][y][z];
                    for i in 0..junctions.len() {
                        let this = junctions[i];
                        // so, die niedrigste distanz ist zu einem in der selben junction oder einer umliegenden junction.
                        // vergleich mit self:

                        let mut finder = Finder {
                            minimum: None,
                            this_i: i,
                            this_x: x,
                            this_y: y,
                            this_z: z,
                        };

                        finder.find(-8..8, x as i32, y as i32, z as i32, self);

                        if let Some(min) = finder.minimum {
                            let min = self.g[min.x][min.y][min.z][min.i];
                            println!("{this:?} {:?} ", min);
                        }
                    }
                }
            }
        }
    }
}

pub struct Minimum {
    x: usize,
    y: usize,
    z: usize,
    i: usize,
    distance: i64,
}

pub struct Finder {
    minimum: Option<Minimum>,
    this_x: usize,
    this_y: usize,
    this_z: usize,
    this_i: usize,
}

impl Finder {
    pub fn find<I>(&mut self, range: I, x: i32, y: i32, z: i32, grid: &Grid)
    where
        I: IntoIterator<Item = i32> + Clone,
    {
        let this = &grid.g[self.this_x][self.this_y][self.this_z][self.this_i];
        let mut minimum = None;
        for x_change in range.clone() {
            let x = x + x_change;
            let x = if x >= 0 {
                x as usize
            } else {
                continue;
            };
            for y_change in range.clone() {
                let y = y + y_change;
                let y = if y >= 0 {
                    y as usize
                } else {
                    continue;
                };
                for z_change in range.clone() {
                    let z = z + z_change;
                    let z = if z >= 0 {
                        z as usize
                    } else {
                        continue;
                    };

                    let iter = grid
                        .g
                        .get(x)
                        .and_then(|slice| slice.get(y).and_then(|slice| slice.get(z)));
                    if let Some(liste) = iter {
                        for (idx, other) in liste.iter().enumerate() {
                            if self.this_x == x
                                && self.this_y == y
                                && self.this_z == z
                                && self.this_i == idx
                            {
                                continue;
                            }
                            let d = this.distance(other);
                            match &mut minimum {
                                None => {
                                    minimum = Some(Minimum {
                                        x,
                                        y,
                                        z,
                                        i: idx,
                                        distance: d,
                                    })
                                }
                                Some(minimum) => {
                                    if minimum.distance > d {
                                        minimum.distance = d;
                                        minimum.x = x;
                                        minimum.y = y;
                                        minimum.z = z;
                                        minimum.i = idx;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.minimum = minimum;
    }
}

impl Extend<Junction> for Grid {
    fn extend<T: IntoIterator<Item = Junction>>(&mut self, iter: T) {
        for junction in iter {
            let x = (junction.x / LATICE_SIZE) as usize;
            let y = (junction.y / LATICE_SIZE) as usize;
            let z = (junction.z / LATICE_SIZE) as usize;

            self.g[x][y][z].push(junction);
        }
    }
}
