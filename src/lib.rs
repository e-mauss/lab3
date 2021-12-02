#[derive(Clone, Debug)]
pub struct Cartesian<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy>
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

pub trait CartesianIteratorExt<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy> {
    type Item;
    fn cartesian(self, other: O) -> Cartesian<I, O, B>;
}

impl<I: Iterator + Clone, O: Iterator<Item=B>, B: Copy> CartesianIteratorExt<I, O, B> for I {
    type Item = (I::Item, O::Item);
    fn cartesian(self, other: O) -> Cartesian<I, O, B> {
        Cartesian::new(self, other)
    }
}