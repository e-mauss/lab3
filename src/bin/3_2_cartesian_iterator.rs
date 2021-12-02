#[derive(Clone, Debug)]
struct Cartesian<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy>
{
    inner_clone: I,
    inner: I,
    curr_outer: Option<O::Item>,
    outer: O,
}

impl<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy> Cartesian<I, O, B>
{
    fn new(inner: I, mut outer: O) -> Self {
        Cartesian { inner_clone: inner.clone(), inner, curr_outer: outer.next(), outer }
    }

    fn reset_inner_advance_outer(&mut self) -> Option<I::Item> {
        self.curr_outer = self.outer.next();
        self.inner = self.inner_clone.clone();
        self.inner.next()
    }

    fn get_curr_outer(&self) -> Option<O::Item> {
        match self.curr_outer {
            None => { None }
            Some(value) => { Some(value.clone()) }
        }
    }
}

impl<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy> Iterator for Cartesian<I, O, B> {
    type Item = (I::Item, O::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next().or_else(|| {
            self.reset_inner_advance_outer()
        }) {
            None => { None }
            Some(inner) => {
                match self.get_curr_outer() {
                    None => { None }
                    Some(value) => { Some((inner, value)) }
                }
            }
        }
    }
}

trait CartesianIteratorExt<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy> {
    type Item;
    fn cartesian(self, other: O) -> Cartesian<I, O, B>;
}

impl<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy> CartesianIteratorExt<I, O, B> for I {
    type Item = (I::Item, O::Item);
    fn cartesian(self, other: O) -> Cartesian<I, O, B> {
        Cartesian::new(self, other)
    }
}

fn main() {
    // A small demonstration of how to use the cartesian iterator:
    // This first example is equivalent to a nested for-loop:
    // for y in 0..4 {
    //    for x in 0..4 {
    //        println!("{},{}", x, y);
    //    }
    // }
    for (x, y) in (0..4).cartesian(0..4) {
        println!("{},{}", x, y);
    }

    // It also works in three dimensions, which is equivalent to three nested for-loop!
    for ((x, y), z) in (0..3).cartesian(0..3).cartesian(0..3) {
        println!("{},{},{}", x, y, z);
    }

    // It also works for string slices, since these are Copy as well!
    let strs1 = ["Tir", "Nef", "Eth", "Ith", "Tal"];
    let strs2 = ["Sur", "Ber", "Jah", "Cham", "Zod"];
    for (x, y) in strs1.iter().cartesian(strs2.iter()) {
        println!("{}{}", x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian_of_empty_ranges() {
        let actual = (0..0).cartesian(1..1).collect::<Vec<_>>();
        assert_eq!(Vec::<(i32, i32)>::new(), actual);
    }

    #[test]
    fn cartesian_with_one_empty_range() {
        // Test with both the inner range empty and the outer range empty
        assert_eq!(
            Vec::<(i32, i32)>::new(),
            (0..0).cartesian(1..10).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<(i32, i32)>::new(),
            (0..10).cartesian(1..1).collect::<Vec<_>>()
        );
    }

    #[test]
    fn cartesian_2d_of_integers() {
        let actual = (0..3).cartesian(4..7).collect::<Vec<_>>();
        //Notice the order of the elements in the tuple! This defines which iterator is the outer one and which
        //the inner one!
        let expected = vec![
            (0, 4),
            (1, 4),
            (2, 4),
            (0, 5),
            (1, 5),
            (2, 5),
            (0, 6),
            (1, 6),
            (2, 6),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn cartesian_3d_of_integers() {
        // You should also be able to chain cartesian iterators!
        // Here, the order of inner/outer iterators becomes even more important!
        let actual = (0..2).cartesian(2..4).cartesian(4..6).collect::<Vec<_>>();
        let expected = vec![
            ((0, 2), 4),
            ((1, 2), 4),
            ((0, 3), 4),
            ((1, 3), 4),
            ((0, 2), 5),
            ((1, 2), 5),
            ((0, 3), 5),
            ((1, 3), 5),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn cartesian_2d_of_string_slices() {
        let str1 = ["a", "b", "c"];
        let str2 = ["1", "2"];
        let actual = str1
            .iter()
            .copied()
            .cartesian(str2.iter().copied())
            .collect::<Vec<_>>();
        let expected = vec![
            ("a", "1"),
            ("b", "1"),
            ("c", "1"),
            ("a", "2"),
            ("b", "2"),
            ("c", "2"),
        ];
        assert_eq!(expected, actual);
    }
}
