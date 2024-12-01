advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        left_list.push(parts[0]);
        right_list.push(parts[1]);
    }

    left_list.sort();
    right_list.sort();

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    Some(total_distance as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
     let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if parts.len() == 2 {
            left_list.push(parts[0]);
            right_list.push(parts[1]);
        }
    }

    let mut right_count = HashMap::new();
    for &num in &right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    let total_similarity_score: i32 = left_list
        .iter()
        .map(|&l| {
            let count = right_count.get(&l).unwrap_or(&0);
            l * count
        })
        .sum();

    Some(total_similarity_score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
