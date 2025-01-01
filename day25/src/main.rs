#[derive(Debug)]
struct Lock {
    heights: [i32; 5],
}

#[derive(Debug)]
struct Key {
    heights: [i32; 5]
}

impl Lock {
    fn new(lock_str: &str) -> Self {
        let mut heights = [-1; 5];
        for line in lock_str.lines() {
            for (i,c) in line.chars().enumerate() {
                if c=='#' {
                    heights[i] += 1;
                }
            }
        }
        Lock { heights }
    }

    fn key_fits(&self, key: &Key) -> bool {
        for i in 0..self.heights.len() {
            if self.heights[i] + key.heights[i] > 5 {
                return false;
            }
        }
        true
    }
}

impl Key {
    fn new(key_str: &str) -> Self {
        let mut heights = [-1; 5];
        for line in key_str.lines().rev() {
            for (i,c) in line.chars().enumerate() {
                if c=='#' {
                    heights[i] += 1;
                }
            }
        }
        Key { heights }
    }
}

fn parse(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    input.split("\n\n").for_each(|key_or_lock_str| {
        if key_or_lock_str.starts_with("#") {
            keys.push(Key::new(key_or_lock_str));
        } else if key_or_lock_str.starts_with(".") {
            locks.push(Lock::new(key_or_lock_str));
        } else {
            panic!("Neither key nor lock")
        }
    });

    (keys, locks)
}


fn main() {
    let (keys, locks) = parse(include_str!("input.txt"));
    println!("{:?}", keys);
    println!("{:?}", locks);
    let mut sum = 0;
    for lock in &locks {
        for key in &keys {
            if lock.key_fits(key) {
                sum += 1;
            }
        }
    }
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lock_parse() {
        let lock = Lock::new("#####
.####
.####
.####
.#.#.
.#...
.....");
        assert_eq!(lock.heights, [0,5,3,4,3]);
    }

    #[test]
    fn test_key_parse() {
        let key = Key::new(".....
#....
#....
#...#
#.#.#
#.###
#####");
        assert_eq!(key.heights, [5,0,2,1,3]);
    }
}
