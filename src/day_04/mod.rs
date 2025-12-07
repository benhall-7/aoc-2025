use std::collections::HashSet;

use nalgebra::Vector2;

use crate::Solution;

// split off an arbitrary element from a (non-empty) set
pub fn pop<T>(set: &mut HashSet<T>) -> Option<T>
where
    T: Eq + Clone + std::hash::Hash,
{
    let elt = set.iter().next().cloned()?;
    set.remove(&elt);
    Some(elt)
}

pub struct Day4;

impl Solution for Day4 {
    fn problem1(&mut self) {
        let map = Map::new_from_file();
        let count = map
            .tp
            .iter()
            .filter(|position| map.get_neighbors(**position).iter().count() < 4)
            .count();

        println!("{count}");
    }

    fn problem2(&mut self) {
        let mut map = Map::new_from_file();
        let start_size = map.tp.len();

        let mut removables: HashSet<Vector2<isize>> = map
            .tp
            .clone()
            .into_iter()
            .filter(|position| map.get_neighbors(*position).iter().count() < 4)
            .collect();

        while removables.len() > 0 {
            // remove all the things
            map.tp.retain(|tp| !removables.contains(tp));
            // get all effected nodes and filter to get the ones that can be removed again
            let new_removables: HashSet<Vector2<isize>> = removables
                .iter()
                .flat_map(|removed| map.get_neighbors(*removed))
                .filter(|effected_node| map.get_neighbors(*effected_node).len() < 4)
                .collect();
            removables = new_removables;
        }

        let end_size = map.tp.len();
        let removed = start_size - end_size;

        println!("{removed}");
    }
}

#[derive(Debug, Clone)]
struct Map {
    tp: HashSet<Vector2<isize>>,
}

impl Map {
    fn new_from_file() -> Self {
        let input = include_str!("input.txt");
        let tp = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter(|(_, chr)| *chr == '@')
                    .map(move |(x, _)| Vector2::new(x as isize, y as isize))
            })
            .collect();

        Self { tp }
    }

    fn get_neighbors(&self, position: Vector2<isize>) -> HashSet<Vector2<isize>> {
        let position_signed = Vector2::new(position.x as isize, position.y as isize);
        let neighbors = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        neighbors
            .iter()
            .map(|(x, y)| Vector2::new(*x, *y))
            .map(|offset| offset + position_signed)
            .filter(|neighbor| self.tp.contains(neighbor))
            .collect()
    }
}
