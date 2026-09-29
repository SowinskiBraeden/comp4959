struct Pair<I> {
    iter: I
}

impl<I> Pair<I> {
    fn new(iter: I) -> Self {
        Self {
            iter
        }
    }
}

impl<I: Iterator> Iterator for Pair<I> {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
       Some((self.iter.next()?, self.iter.next()?))
    }
}

use std::collections::HashMap;

fn main() {
    let v = vec!["homer", "25", "bart", "55", "monty", "101"];
    let h: HashMap<_, _> = Pair::new(v.into_iter()).filter_map(|(c, s)| 
        match s.parse::<u8>() {
            Ok(score) if score <= 100 => Some((c, score)),
            _ => None,
        }).collect();
    println!("{h:?}");
}
