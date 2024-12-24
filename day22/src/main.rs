use std::ops::BitXor;

fn mix(secret: u128, value: u128) -> u128 {
    secret.bitxor(value)
}

fn prune(secret: u128) -> u128 {
    secret % 16777216u128
}

fn evolve_phase_1(secret: u128) -> u128 {
    let secret_mul_64 = secret * 64u128;
    let mixed = mix(secret, secret_mul_64);
    prune(mixed)
}

fn evolve_phase_2(secret: u128) -> u128 {
    let secret_div_32 = secret / 32u128;
    let mixed = mix(secret, secret_div_32);
    prune(mixed)
}

fn evolve_phase_3(secret: u128) -> u128 {
    let secret_mul_2024 = secret * 2048u128;
    let mixed = mix(secret, secret_mul_2024);
    prune(mixed)
}

fn evolve(secret: u128) -> u128 {
    let secret_phase_1 = evolve_phase_1(secret);
    let secret_phase_2 = evolve_phase_2(secret_phase_1);
    let secret_phase_3 = evolve_phase_3(secret_phase_2);
    secret_phase_3
}

fn evolve_n(mut secret: u128, n: usize) -> u128 {
    for _i in 0..n {
        secret = evolve(secret);
    }
    secret
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    0
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
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
            123u128,
            15887950u128,
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
        let start = 123;
        let mut actual = vec![123];
        for i in 0..expected.len() - 1 {
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
}
