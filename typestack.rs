trait Reduce<E, R> {
    fn pop(self) -> (E, R);
}

impl<E, R> Reduce<E, R> for (E, R) {
    fn pop(self) -> (E, R) {
        (self.0, self.1)
    }
}

trait Increase<E> {
    fn push(self, insert: E) -> (E, Self);
}

impl<E, S> Increase<E> for S {
    fn push(self, insert: E) -> (E, Self) {
        (insert, self)
    }
}

fn main() {
    let q = ().push(Some(1u32)).push("thing").push(true);
    let (b, q) = q.pop();
    println!("{}", b);
    let (s, q) = q.pop();
    println!("{}", s);
    let (mu, ()) = q.pop();
    println!("{}", mu.unwrap());
}
