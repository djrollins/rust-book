// Has to be Sized so it can be consumed rather than referenced.
trait IteratorExt: Iterator + Sized {
    fn wat(self) -> Self
    {
        println!("wat");
        self
    }
}

// Blanket implementation for all types that implement Iterator
// This is how Itertools seemingly adds methods to the Iterator trait.
impl<I> IteratorExt for I where I: Iterator {}

fn main() {
    let vec1 = vec![1, 2, 3, 4];

    let result: Vec<_> = vec1.iter()
        .wat()
        .collect();

    println!("{:?}", result);
}
