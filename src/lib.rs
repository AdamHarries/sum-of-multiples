#![feature(test)]
extern crate itertools;
use itertools::Itertools;

pub fn som_c_style(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();

    let filter = factors.into_iter().filter(|n| n > &&(0 as u32));

    for factor in filter {
        let mut mult: Vec<u32> = (1..limit).filter(|n| n % factor == 0).collect();

        multiples.append(&mut mult)
    }

    multiples.sort();
    multiples.dedup();
    return multiples.iter().sum::<u32>();
}

pub fn som_iterator_style(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = factors
        .into_iter()
        .filter(|n| n > &&(0 as u32))
        .map(|f| (1..limit).filter(move |n| n % f == 0))
        .flatten()
        .collect();

    multiples.sort();
    multiples.dedup();
    multiples.iter().sum::<u32>()
}

pub fn som_itertools_style(limit: u32, factors: &[u32]) -> u32 {
    factors
        .into_iter()
        .filter(|n| n > &&(0 as u32))
        .map(|f| (1..limit).filter(move |n| n % f == 0))
        .flatten()
        .sorted()
        .dedup()
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    static primes: [u32; 64] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311,
    ];
    #[test]
    fn iterator_correctness() {
        let limit: u32 = 1e4 as u32;
        let expected = super::som_c_style(limit, &primes);
        let iterator = super::som_iterator_style(limit, &primes);
        assert_eq!(expected, iterator);
    }

    #[test]
    fn itertools_correctness() {
        let limit: u32 = 1e4 as u32;
        let expected = super::som_c_style(limit, &primes);
        let itertools = super::som_itertools_style(limit, &primes);
        assert_eq!(expected, itertools);
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use test::{black_box, Bencher};

    static primes: [u32; 30] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113,
    ];

    #[bench]
    fn bench_som_c_style(b: &mut Bencher) {
        let limit: u32 = 1e4 as u32;
        b.iter(|| super::som_c_style(limit, &primes));
    }

    #[bench]
    fn bench_som_iterator_style(b: &mut Bencher) {
        let limit: u32 = 1e4 as u32;
        b.iter(|| super::som_iterator_style(limit, &primes));
    }

    #[bench]
    fn bench_som_itertools_style(b: &mut Bencher) {
        let limit: u32 = 1e4 as u32;
        b.iter(|| super::som_itertools_style(limit, &primes));
    }
}
