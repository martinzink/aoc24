use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct BlockData {
    size: u32,
    id: Option<u32>,
}

impl BlockData {
    fn split(&self, at: u32) -> [BlockData; 2] {
        assert!(at < self.size);
        [
            BlockData {
                size: at,
                id: self.id,
            },
            BlockData {
                size: self.size - at,
                id: self.id,
            },
        ]
    }
}

#[derive(Debug)]
struct DiskMap {
    blocks: Vec<BlockData>,
}

impl DiskMap {
    fn new(input: &str) -> DiskMap {
        let mut blocks: Vec<BlockData> = Vec::new();
        for (i, c) in input.chars().enumerate() {
            let d = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                blocks.push(BlockData {
                    size: d,
                    id: Some((i / 2) as u32),
                });
            } else {
                blocks.push(BlockData { size: d, id: None });
            }
        }
        DiskMap { blocks }
    }

    fn calc_checksum(&self) -> u64 {
        let mut sum = 0u64;
        let mut i = 0u64;
        for block in &self.blocks {
            for _ in 0..block.size {
                sum += (block.id.unwrap_or(0) as u64) * i;
                i += 1;
            }
        }
        sum
    }
}

fn part_one(input: &str) -> u64 {
    let mut disk_map = DiskMap::new(input);
    let mut left_pointer = 0;
    let mut right_pointer = disk_map.blocks.len() - 1;
    let mut new_blocks: Vec<BlockData> = Vec::new();
    while left_pointer <= right_pointer {
        if disk_map.blocks[left_pointer].id != None || disk_map.blocks[left_pointer].size == 0 {
            new_blocks.push(disk_map.blocks[left_pointer].clone());
            left_pointer += 1;
            continue;
        }
        if disk_map.blocks[right_pointer].id == None || disk_map.blocks[right_pointer].size == 0 {
            right_pointer -= 1;
            continue;
        }
        let left_block_size = disk_map.blocks.get(left_pointer).unwrap().size;
        let right_block_size = disk_map.blocks.get(right_pointer).unwrap().size;
        match left_block_size.cmp(&right_block_size) {
            Ordering::Equal => {
                new_blocks.push(disk_map.blocks.get(right_pointer).unwrap().clone());
                left_pointer += 1;
                right_pointer -= 1;
            }
            Ordering::Less => {
                let right_block = disk_map.blocks.get_mut(right_pointer).unwrap();
                let splits = right_block.split(right_block_size - left_block_size);
                new_blocks.push(splits[1]);
                right_block.size = splits[0].size;
                left_pointer += 1;
            }
            Ordering::Greater => {
                new_blocks.push(disk_map.blocks.get(right_pointer).unwrap().clone());
                let left_block = disk_map.blocks.get_mut(left_pointer).unwrap();
                left_block.size = left_block_size - right_block_size;
                right_pointer -= 1;
            }
        }
    }

    let new_map = DiskMap { blocks: new_blocks };
    new_map.calc_checksum()
}

fn part_two(input: &str) -> u64 {
    let mut disk_map = DiskMap::new(input);
    let mut left_pointer = 0;
    let mut new_blocks: Vec<BlockData> = Vec::new();
    while left_pointer < disk_map.blocks.len() {
        if disk_map.blocks[left_pointer].id != None || disk_map.blocks[left_pointer].size == 0 {
            new_blocks.push(disk_map.blocks[left_pointer].clone());
            disk_map.blocks[left_pointer].id = None;
            left_pointer += 1;
            continue;
        }
        let free_space = disk_map.blocks[left_pointer].size;
        let block_to_move = disk_map
            .blocks
            .iter_mut()
            .rfind(|b| b.size <= free_space && b.id.is_some());
        if block_to_move.is_none() {
            new_blocks.push(BlockData {
                size: free_space,
                id: None,
            });
            left_pointer += 1;
            continue;
        }
        let block_to_move = block_to_move.unwrap();
        let block_size = block_to_move.size;
        new_blocks.push(block_to_move.clone());
        block_to_move.id = None;
        disk_map.blocks[left_pointer].size = free_space - block_size;
    }

    let new_map = DiskMap { blocks: new_blocks };
    new_map.calc_checksum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 1928);
        assert_eq!(part_one(INPUT), 6382875730645);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 2858);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
