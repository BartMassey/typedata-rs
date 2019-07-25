trait Reduce<E, R> {
    type Reduced;

    fn pop(self) -> (E, Self::Reduced);
}

impl<E> Reduce<E, ()> for (E, ()) {
    type Reduced = ();

    fn pop(self) -> (E, Self::Reduced) {
        (self.0, ())
    }
}

impl<E, E1, E2, R, R2> Reduce<E, R> for (E1, (E2, R2))
    where (E2, R2): Reduce<E, R>
{
    type Reduced = (E1, <(E2, R2) as Reduce<E, R>>::Reduced);

    fn pop(self) -> (E, Self::Reduced) {
        let (e, q) = self.1.pop();
        (e, (self.0, q))
    }
}

trait Increase<E, I> {
    fn push(self, insert: E) -> I;
}

impl<E, I> Increase<E, (E, I)> for I {
    fn push(self, insert: E) -> (E, I) {
        (insert, self)
    }
}

fn main() {
    let q = ().push(Some(1u32)).push("thing").push(true);
    let (mu, q) = q.pop();
    println!("{}", mu.unwrap());
    let (s, q) = q.pop();
    println!("{}", s);
    let (b, ()) = q.pop();
    println!("{}", b);
}
