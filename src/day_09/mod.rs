use itertools::Itertools;
use nalgebra::Vector2;

use crate::Solution;

fn get_verteces() -> Vec<Vector2<i64>> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            let coords: [&str; 2] = parts.try_into().unwrap();
            let nums = coords.map(|s| i64::from_str_radix(s, 10).unwrap());
            Vector2::new(nums[0], nums[1])
        })
        .collect()
}

pub struct Day9;

impl Solution for Day9 {
    fn problem1(&mut self) {
        let verteces = get_verteces();
        // with fewer than 500 elements, it's perfectly reasonable to brute force it
        let len = verteces.len();
        let unique_pairs: Vec<(usize, usize)> = (0..len)
            .into_iter()
            .cartesian_product((0..len).into_iter())
            .filter(|(i, j)| i < j)
            .collect();
        let max_area = unique_pairs.iter().fold(0, |max, pair| {
            let coord_a = verteces[pair.0];
            let coord_b = verteces[pair.1];
            let diff = coord_a - coord_b;
            let area = (diff.x.abs() + 1) * (diff.y.abs() + 1);
            max.max(area)
        });

        println!("{max_area}");
    }

    fn problem2(&mut self) {
        let verteces = get_verteces();
        let len = verteces.len();

        // CONCERN: how do we know if a space is inside or outside of the shape???
        // the shape is a big convex circle with a thin slice taken out the middle,
        // so it won't affect us this time!
        let unique_pairs: Vec<(usize, usize)> = (0..len)
            .into_iter()
            .cartesian_product((0..len).into_iter())
            .filter(|(i, j)| i < j)
            .collect();

        // this might make our solution n^3 lol, but at least the ops are fast
        let valid_pairs: Vec<(usize, usize)> = unique_pairs
            .into_iter()
            .filter(|(i, j)| {
                let coord_a = verteces[*i];
                let coord_b = verteces[*j];
                let min_x = coord_a.x.min(coord_b.x);
                let min_y = coord_a.y.min(coord_b.y);
                let max_x = coord_a.x.max(coord_b.x);
                let max_y = coord_a.y.max(coord_b.y);
                let min_corner = Vector2::new(min_x, min_y);
                let max_corner = Vector2::new(max_x, max_y);

                // it's invalid if any edge between two consecutive vertices crosses our rect

                !verteces.iter().circular_tuple_windows().any(|(a, b)| {
                    // check if the edge [vec_a, vec_b] intersects the space inside our rectangle
                    // horizontal case
                    if a.y == b.y {
                        let y = a.y;
                        // check that y is within range
                        if y < max_corner.y && y > min_corner.y {
                            let outside =
                                a.x.max(b.x) <= min_corner.x || a.x.min(b.x) >= max_corner.x;
                            return !outside;
                        }
                    }
                    // vertical case
                    else if a.x == b.x {
                        let x = a.x;
                        // check that y is within range
                        if x < max_corner.x && x > min_corner.x {
                            let outside =
                                a.y.max(b.y) <= min_corner.y || a.y.min(b.y) >= max_corner.y;
                            return !outside;
                        }
                    }
                    false
                })
            })
            .collect();

        let max_area = valid_pairs.iter().fold(0, |max, pair| {
            let coord_a = verteces[pair.0];
            let coord_b = verteces[pair.1];
            let diff = coord_a - coord_b;
            let area = (diff.x.abs() + 1) * (diff.y.abs() + 1);
            max.max(area)
        });

        println!("{max_area}");
    }
}
