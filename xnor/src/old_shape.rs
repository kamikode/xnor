use core::fmt::Display;
use core::ops::Index;

#[derive(Debug, PartialEq)]
pub struct Shape<const N: usize>(pub [usize; N]);

impl<const N: usize> Shape<N> {
    pub fn ndim(&self) -> usize {
        N
    }

    pub fn size(&self) -> usize {
        self.0.iter().product()
    }
}

impl<const N: usize> Index<isize> for Shape<N> {
    type Output = usize;
    fn index(&self, index: isize) -> &Self::Output {
        let mut i = index;
        if i < 0 {
            i += isize::try_from(N)
                .expect("number of dimensions should be small enough to fit into isize");
            if i < 0 {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    N, index
                );
            }
        }
        &self.0[usize::try_from(i).expect("index should be positive and therefore fit into usize")]
    }
}

impl<const N: usize> Display for Shape<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            0 => write!(f, "()"),
            1 => write!(f, "({})", &self.0[0]),
            _ => {
                write!(f, "(")?;
                for i in 0..N - 1 {
                    write!(f, "{}, ", &self.0[i])?;
                }
                write!(f, "{})", &self.0[N - 1])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn ndim() {
        assert_eq!(Shape([]).ndim(), 0);
        assert_eq!(Shape([2]).ndim(), 1);
        assert_eq!(Shape([2, 3]).ndim(), 2);
        assert_eq!(Shape([2, 3, 4]).ndim(), 3);
    }

    #[test]
    fn size() {
        assert_eq!(Shape([]).size(), 1);
        assert_eq!(Shape([2]).size(), 2);
        assert_eq!(Shape([2, 3]).size(), 6);
        assert_eq!(Shape([2, 3, 4]).size(), 24);
    }

    #[test]
    fn index() {
        let s1 = Shape([7]);
        assert_eq!(s1[0], 7);
        assert_eq!(s1[-1], 7);
        let s2 = Shape([3, 8]);
        assert_eq!(s2[0], 3);
        assert_eq!(s2[1], 8);
        assert_eq!(s2[-1], 8);
        assert_eq!(s2[-2], 3);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let _ = Shape([2, 3])[-3];
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds_0d() {
        let _ = Shape([])[0];
    }

    #[test]
    fn to_string() {
        assert_eq!(Shape([]).to_string(), "()");
        assert_eq!(Shape([2]).to_string(), "(2)");
        assert_eq!(Shape([5, 8]).to_string(), "(5, 8)");
        assert_eq!(Shape([3, 9, 6]).to_string(), "(3, 9, 6)");
    }
}
