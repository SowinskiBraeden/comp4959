// example to show how to implement an iterator
struct VecIntoIter {
    v: Vec<i32>,
    len: usize,
    i: usize,
}

impl VecIntoIter {
    fn new(v: Vec<i32>) -> VecIntoIter {
        Self {
            len: v.len(),
            v,
            i: 0
        }
    }
}

impl Iterator for VecIntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.len {
            return None;
        }
        let n = self.i;
        self.i += 1;
        Some(self.v[n])
    }
}

fn main() {
    let v = vec![1, 2, 3];
    let mut it = VecIntoIter::new(v);

    while let Some(x) = it.next() {
        println!("{x}");
    }
    // println!("{v:?}");  // v is consumed
}
