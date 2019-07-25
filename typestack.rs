struct TypeStack;

trait Pop<E, R> {
    fn pop(self) -> (E, R);
}

impl<E, R> Pop<E, R> for (E, R) {
    fn pop(self) -> (E, R) {
        (self.0, self.1)
    }
}

trait Push<E> {
    fn push(self, insert: E) -> (E, Self);
}

impl<E, S> Push<E> for S {
    fn push(self, insert: E) -> (E, Self) {
        (insert, self)
    }
}

fn main() {
    let q = TypeStack.push(Some(1u32)).push("thing").push(true);
    let (b, q) = q.pop();
    println!("{}", b);
    let (s, q) = q.pop();
    println!("{}", s);
    let (mu, TypeStack) = q.pop();
    println!("{}", mu.unwrap());
}
