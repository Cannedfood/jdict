pub struct NumberCompressor {
    pub sort: bool,
    pub delta: bool,
    pub gcd: bool,
}
impl NumberCompressor {
    pub fn compress(&self, mut numbers: Vec<u32>) {
        if self.sort {
            numbers.sort_unstable();
        }
        if self.delta {
            delta_encode(&mut numbers);
        }
        if self.gcd {
            gcd_encode(&mut numbers);
        }
    }
}

fn delta_encode(numbers: &mut [u32]) {
    let mut prev = 0;
    for i in numbers.iter_mut() {
        let current = *i;
        *i = current - prev;
        prev = current;
    }
}
fn delta_decode(numbers: &mut Vec<u32>) {
    let mut prev = 0;
    for i in numbers {
        *i += prev;
        prev = *i;
    }
}

fn gcd_encode(numbers: &mut Vec<u32>) {
    let mut gcd = *numbers.first().unwrap_or(&1);
    for num in numbers.iter() {
        gcd = num::integer::gcd(gcd, *num);
    }
    for n in numbers.iter_mut() {
        *n /= gcd;
    }
    numbers.push(gcd);
}
fn gcd_decode(numbers: &mut Vec<u32>) {
    let gcd = numbers.pop().unwrap_or(1);
    for n in numbers.iter_mut() {
        *n *= gcd;
    }
}

// TODO: actually return SMALLEST set - currently [2*3*5, 2*3] will return [2, 3, 5], but should return [6, 5]
fn find_smallest_set_of_factors(numbers: &[u32]) -> Vec<u32> {
    let primes = find_primes_up_to(*numbers.iter().max().unwrap_or(&1));

    let factors = Vec::<Vec<(u32, u32)>>::new();
    for n in numbers {
        let _n_factors = prime_factors(*n, &primes);
        // TODO: add n_factors to factors
    }

    factors.iter()
    .map(
        |fac| {
            let mut result = 1;
            for (prime, count) in fac {
                result *= prime.pow(*count);
            }
            result
        }
    )
    .collect()
}

/// Gives you the prime factors of a number
/// The prime factors are returned as a vector of tuples (prime, count)
/// ```
/// assert_eq!(jdict_tiny::num_compression::prime_factors(1, &[]), vec![]);
/// assert_eq!(jdict_tiny::num_compression::prime_factors(2, &[2, 3]), vec![(2, 1)]);
/// assert_eq!(jdict_tiny::num_compression::prime_factors(4, &[2, 3]), vec![(2, 2)]);
/// assert_eq!(jdict_tiny::num_compression::prime_factors(12, &[2, 3]), vec![(2, 2), (3, 1)]);
/// ```
pub fn prime_factors(mut n: u32, primes: &[u32]) -> Vec<(u32, u32)> {
    let mut result = Vec::new();

    for prime in primes {
        let mut count = 0;
        while n % prime == 0 {
            n /= prime;
            count += 1;
        }
        if count > 0 {
            result.push((*prime, count));
        }
    }

    result
}

/// Naive prime search
/// ```
/// assert_eq!(jdict_tiny::num_compression::find_primes_up_to(1),  vec![1]);
/// assert_eq!(jdict_tiny::num_compression::find_primes_up_to(2),  vec![1, 2]);
/// assert_eq!(jdict_tiny::num_compression::find_primes_up_to(3),  vec![1, 2, 3]);
/// assert_eq!(jdict_tiny::num_compression::find_primes_up_to(10), vec![1, 2, 3, 5, 7]);
/// ```
pub fn find_primes_up_to(n: u32) -> Vec<u32> {
    if n == 1 {
        return vec![1];
    }

    let mut primes = vec![1, 2];
    for i in (3..=n).step_by(2) {
        if primes.iter().take_while(|&&p| p < i / 2).all(|&p| i % p != 0) {
            primes.push(i);
        }
    }
    primes
}
