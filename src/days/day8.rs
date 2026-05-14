use std::collections::HashMap;

use crate::Solution;

pub struct Day8 {}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] != i {
        parent[i] = find(parent, parent[i]);
    }
    parent[i]
}

fn union(parent: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(parent, a);
    let root_b = find(parent, b);
    if root_a != root_b {
        parent[root_a] = root_b;
    }
}

impl Solution for Day8 {
    fn part_one(&self, input: &str) -> String {
        let mut box_vec: Vec<(i64, i64, i64)> = Vec::new();

        for line in input.lines() {
            let three_num_str = line.split(',').collect::<Vec<_>>();
            if three_num_str.len() == 3 {
                let v = three_num_str
                    .iter()
                    .map(|s| s.parse::<i64>().expect("failed to parse number"))
                    .collect::<Vec<_>>();
                let num_pair: (i64, i64, i64) = (v[0], v[1], v[2]);
                box_vec.push(num_pair);
            }
        }

        let mut distances: Vec<(i64, i64, i64)> = Vec::new();
        for i in 0..box_vec.len() {
            for j in (i + 1)..box_vec.len() {
                let distance = (box_vec[j].0 - box_vec[i].0).pow(2)
                    + (box_vec[j].1 - box_vec[i].1).pow(2)
                    + (box_vec[j].2 - box_vec[i].2).pow(2);

                distances.push((distance, i as i64, j as i64));
            }
        }

        distances.sort();

        let mut parent: Vec<usize> = (0..box_vec.len()).collect();

        for (_, i, j) in distances.iter().take(1000) {
            union(&mut parent, *i as usize, *j as usize);
        }

        let mut size_map: HashMap<usize, usize> = HashMap::new();

        for i in 0..box_vec.len() {
            let root = find(&mut parent, i);
            *size_map.entry(root).or_insert(0) += 1;
        }

        let mut sizes: Vec<usize> = size_map.values().cloned().collect();
        sizes.sort_by(|a, b| b.cmp(a));
        let result = sizes[0] * sizes[1] * sizes[2];

        result.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut box_vec: Vec<(i64, i64, i64)> = Vec::new();

        for line in input.lines() {
            let three_num_str = line.split(',').collect::<Vec<_>>();
            if three_num_str.len() == 3 {
                let v = three_num_str
                    .iter()
                    .map(|s| s.parse::<i64>().expect("failed to parse number"))
                    .collect::<Vec<_>>();
                let num_pair: (i64, i64, i64) = (v[0], v[1], v[2]);
                box_vec.push(num_pair);
            }
        }

        let mut distances: Vec<(i64, i64, i64)> = Vec::new();
        for i in 0..box_vec.len() {
            for j in (i + 1)..box_vec.len() {
                let distance = (box_vec[j].0 - box_vec[i].0).pow(2)
                    + (box_vec[j].1 - box_vec[i].1).pow(2)
                    + (box_vec[j].2 - box_vec[i].2).pow(2);

                distances.push((distance, i as i64, j as i64));
            }
        }

        distances.sort();

        let mut parent: Vec<usize> = (0..box_vec.len()).collect();

        let mut num_circuits = box_vec.len();
        let mut answer = 0;
        for (_, i, j) in distances.iter() {
            let a = *i as usize;
            let b = *j as usize;
            if find(&mut parent, a) != find(&mut parent, b) {
                union(&mut parent, a, b);
                num_circuits -= 1;
                if num_circuits == 1 {
                    answer = box_vec[a].0 * box_vec[b].0;
                    break;
                }
            }
        }

        answer.to_string()
    }
}
