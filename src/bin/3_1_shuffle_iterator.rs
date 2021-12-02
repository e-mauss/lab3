use rand::{Rng};

struct ShuffleIterator<'a, T> {
    data: &'a [T],
    current: usize,
    seed: u32,
    elements_left: usize,
}

impl<'a, T> ShuffleIterator<'a, T> {
    pub fn new(data: &'a [T], current: usize) -> Self {
        ShuffleIterator {
            data,
            current,
            seed: rand::thread_rng().gen(),
            elements_left: 0,
        }
    }

    /// Permutes the index `i` within a range [0;length) using the given `seed` value. This function is taken from
    /// the paper 'Correlated Multi-Jittered Sampling' by Andrew Kensler (Pixar Technical Memo 13-01, 2013)
    fn permute(mut i: u32, length: u32, seed: u32) -> u32 {
        let mut w = length - 1;
        w |= w >> 1;
        w |= w >> 2;
        w |= w >> 4;
        w |= w >> 8;
        w |= w >> 16;

        loop {
            i ^= seed;
            i = i.wrapping_mul(0xe170893d);
            //i *= 0xe170893d;
            i ^= seed >> 16;
            i ^= (i & w) >> 4;
            i ^= seed >> 8;
            i = i.wrapping_mul(0x0929eb3f);
            //i *= 0x0929eb3f;
            i ^= seed >> 23;
            i ^= (i & w) >> 1;
            i = i.wrapping_mul(1 | seed >> 27);
            //i *= 1 | seed >> 27;
            i = i.wrapping_mul(0x6935fa69);
            //i *= 0x6935fa69;
            i ^= (i & w) >> 11;
            i = i.wrapping_mul(0x74dcb303);
            //i *= 0x74dcb303;
            i ^= (i & w) >> 2;
            i = i.wrapping_mul(0x9e501cc3);
            //i *= 0x9e501cc3;
            i ^= (i & w) >> 2;
            i = i.wrapping_mul(0xc860a3df);
            //i *= 0xc860a3df;
            i &= w;
            i ^= i >> 5;
            if i < length {
                break;
            }
        }

        (i + seed) % length
    }
}

impl<'a, T> Iterator for ShuffleIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret;
        if self.elements_left == self.data.len() {
            ret = None
        } else {
            self.current = ShuffleIterator::<T>::permute(
                self.elements_left as u32, self.data.len() as u32, self.seed) as usize;
            self.elements_left += 1;
            ret = Some(&self.data[self.current]);
        }
        ret
    }
}

trait ShuffleExt<'a, T> {
    type Item;
    fn shuffle(&'a self) -> ShuffleIterator<'a, T>;
}

impl<'a, T> ShuffleExt<'a, T> for &'a [T] {
    type Item = &'a T;

    fn shuffle(&'a self) -> ShuffleIterator<'a, T> {
        ShuffleIterator::new(self, 0)
    }
}

fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:#?}", data.as_slice().shuffle().collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn shuffle_is_exhaustive() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected = data.iter().copied().collect::<HashSet<_>>();
        let shuffled = data.as_slice().shuffle().copied().collect::<HashSet<_>>();
        assert_eq!(expected, shuffled);
    }

    #[test]
    fn shuffle_is_random() {
        let first_100_numbers = (0..100).collect::<Vec<_>>();
        // Shuffle twice, the shuffles must not be equal. Since there are 2^32 possible seed values, the chance for two shuffles to be
        // equal is 1/2^32. This is not a lot, but can happen, so we shuffle three times and require that at least one shuffle is unique,
        // the chance for that is 1/2^64
        let s1 = first_100_numbers
            .as_slice()
            .shuffle()
            .copied()
            .collect::<Vec<_>>();
        let s2 = first_100_numbers
            .as_slice()
            .shuffle()
            .copied()
            .collect::<Vec<_>>();
        let s3 = first_100_numbers
            .as_slice()
            .shuffle()
            .copied()
            .collect::<Vec<_>>();

        let s1_unique = s1 != s2 || s1 != s3;
        let s2_unique = s2 != s1 || s2 != s3;
        assert!(s1_unique || s2_unique);
    }

    #[test]
    fn shuffle_does_not_modify_slice() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        data.as_slice().shuffle().for_each(|_| {});
        // The assertion is technically irrelevant since data is declared as immutable
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
