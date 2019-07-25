pub struct TypeStack;

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

#[test]
fn test_basic() {
    let q = TypeStack.push(Some(1u32)).push("thing").push(true);
    let (b, q) = q.pop();
    assert_eq!(true, b);
    let (s, q) = q.pop();
    assert_eq!("thing", s);
    let (mu, TypeStack) = q.pop();
    assert_eq!(1u32, mu.unwrap());
}
