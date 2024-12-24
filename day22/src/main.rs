use std::collections::HashMap;
use std::ops::BitXor;

fn mix(secret: i64, value: i64) -> i64 {
    secret.bitxor(value)
}

fn prune(secret: i64) -> i64 {
    secret % 16777216i64
}

fn evolve_phase_1(secret: i64) -> i64 {
    let secret_mul_64 = secret * 64i64;
    let mixed = mix(secret, secret_mul_64);
    prune(mixed)
}

fn evolve_phase_2(secret: i64) -> i64 {
    let secret_div_32 = secret / 32i64;
    let mixed = mix(secret, secret_div_32);
    prune(mixed)
}

fn evolve_phase_3(secret: i64) -> i64 {
    let secret_mul_2024 = secret * 2048i64;
    let mixed = mix(secret, secret_mul_2024);
    prune(mixed)
}

fn evolve(secret: i64) -> i64 {
    let secret_phase_1 = evolve_phase_1(secret);
    let secret_phase_2 = evolve_phase_2(secret_phase_1);
    let secret_phase_3 = evolve_phase_3(secret_phase_2);
    secret_phase_3
}

fn evolve_n(mut secret: i64, n: usize) -> i64 {
    for _i in 0..n {
        secret = evolve(secret);
    }
    secret
}

fn get_prices(mut secret: i64, n: usize) -> Vec<i64> {
    let mut prices: Vec<i64> = Vec::new();
    for _i in 0..n {
        secret = evolve(secret);
        prices.push(secret % 10);
    }
    prices
}

fn part_one(input: &str) -> i64 {
    input.lines().map(|line| evolve_n(line.parse::<i64>().unwrap(), 2000)).sum::<i64>()
}

fn part_two(input: &str) -> ([i64; 4], i64) {
    let mut overall_hashmap : HashMap<[i64; 4], i64> = HashMap::new();
    input.lines().for_each(|line| {
        let start_price = line.parse::<i64>().unwrap();
        let mut hashmap : HashMap<[i64; 4], i64> = HashMap::new();
        let prices = get_prices(start_price, 2000);
        let price_changes = prices.windows(2).map(|window| window[1] - window[0]).collect::<Vec<_>>();
        for (i, window) in price_changes.windows(4).enumerate() {
            hashmap.entry(<[i64; 4]>::try_from(window).unwrap()).or_insert(*prices.get(i + 4).unwrap());
        }
        for (key, val) in hashmap {
            *overall_hashmap.entry(key).or_default() += val;
        }
    });
    let asd = overall_hashmap.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    (*asd.0, *asd.1)
}



fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part one: {:?}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_evolve() {
        let expected = [
            123i64,
            15887950i64,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ];
        let mut actual = vec![123];
        for _i in 0..expected.len() - 1 {
            actual.push(evolve(*actual.last().unwrap()));
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evolve_n() {
        assert_eq!(8685429, evolve_n(1, 2000));
        assert_eq!(4700978, evolve_n(10, 2000));
        assert_eq!(15273692, evolve_n(100, 2000));
        assert_eq!(8667524, evolve_n(2024, 2000));

    }

    #[test]
    fn test_part_two() {
        let input = "1\n2\n3\n2024";
        assert_eq!(part_two(input), ([-2, 1, -1, 3], 23));
    }
}
