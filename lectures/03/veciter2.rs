// example on how to implement iterators
struct VecIntoIter<T> {
    v: Vec<T>,
    len: usize,
    i: usize,
}

impl<T> VecIntoIter<T> {
    fn new(v: Vec<T>) -> Self {
        Self {
            len: v.len(),
            v,
            i: 0
        }
    }
}

struct VecIter<'a, T> {
    v: &'a Vec<T>,
    len: usize,
    i: usize,
}


impl<'a, T> VecIter<'a, T> {
    fn new(v: &'a Vec<T>) -> Self {
        Self {
            len: v.len(),
            v,
            i: 0
        }
    }
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.len {
            return None;
        }
        let n = self.i;
        self.i += 1;
        Some(&self.v[n])
    }
}


impl<T: Copy> Iterator for VecIntoIter<T> {
    type Item = T;

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

    let mut it = VecIter::new(&v);

    while let Some(x) = it.next() {
        println!("{x}");
    }

    println!("{v:?}");
 
    let mut it = VecIntoIter::new(v);

    while let Some(x) = it.next() {
        println!("{x}");
    }
    // println!("{v:?}");  // v is consumed
}
