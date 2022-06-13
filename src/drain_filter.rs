pub struct DrainFilter<'a, T, F>
where
    F: for<'b> Fn(&'b T) -> bool,
{
    f: F,
    items: &'a mut Vec<T>,
    idx: usize,
}

impl<'a, T, F> Iterator for DrainFilter<'a, T, F>
where
    F: for<'b> Fn(&'b T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.items.len() {
            if (self.f)(&self.items[self.idx]) {
                return Some(self.items.swap_remove(self.idx));
            } else {
                self.idx += 1;
            }
        }
        None
    }
}

pub fn drain_filter<T, F>(items: &mut Vec<T>, f: F) -> DrainFilter<T, F>
where
    F: for<'b> Fn(&'b T) -> bool,
{
    DrainFilter { f, items, idx: 0 }
}
