use num_traits::Num;
use std::iter::FusedIterator;

pub struct Partitions<T: Num + PartialOrd + Copy> {
    n: T,
    m: T,
}

impl<T: Num + PartialOrd + Copy> Partitions<T> {
    pub fn new(n: T, m: T) -> Self {
        assert!(n >= m);
        Self { n, m }
    }
}

impl<T: Num + PartialOrd + Copy> Iterator for Partitions<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.n.is_zero() {
            return None;
        }
        let v = self.n / self.m;
        self.n = self.n - v;
        self.m = self.m - T::one();
        Some(v)
    }
}

impl ExactSizeIterator for Partitions<usize> {
    #[inline]
    fn len(&self) -> usize {
        self.m
    }
}

impl<T: Num + PartialOrd + Copy> FusedIterator for Partitions<T> {}

#[cfg(test)]
mod tests {
    use super::Partitions;

    #[test]
    fn case_1() {
        let iter = Partitions::new(10, 3);
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.sum::<usize>(), 10);
    }

    #[test]
    fn case_1_next() {
        let mut iter = Partitions::new(10, 3);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
    }

    #[test]
    fn case_2() {
        let iter = Partitions::new(123, 14);
        assert_eq!(iter.len(), 14);
        assert_eq!(iter.sum::<usize>(), 123usize);
    }

    #[test]
    fn case_3() {
        let iter = Partitions::new(739845, 192873);
        assert_eq!(iter.len(), 192873);
        assert_eq!(iter.sum::<usize>(), 739845usize);
    }

    #[test]
    fn case_4() {
        let iter = Partitions::<f64>::new(10.0, 3.0);
        assert_eq!(iter.sum::<f64>(), 10.0);
    }

    fn between(v: f64, min: f64, max: f64) -> bool {
        min <= v && v <= max
    }

    #[test]
    fn case_4_next() {
        let mut iter = Partitions::<f64>::new(10.0, 3.0);
        assert!(between(iter.next().unwrap(), 3.33, 3.34));
        assert!(between(iter.next().unwrap(), 3.33, 3.34));
        assert!(between(iter.next().unwrap(), 3.33, 3.34));
    }
}
