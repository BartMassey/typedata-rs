pub struct TypeQueue;

pub trait PopFront<E, R> {
    type Popped;

    fn pop_front(self) -> (E, Self::Popped);
}

impl<E> PopFront<E, TypeQueue> for (E, TypeQueue) {
    type Popped = TypeQueue;

    fn pop_front(self) -> (E, Self::Popped) {
        (self.0, TypeQueue)
    }
}

impl<E, E1, E2, R, R2> PopFront<E, R> for (E1, (E2, R2))
    where (E2, R2): PopFront<E, R>
{
    type Popped = (E1, <(E2, R2) as PopFront<E, R>>::Popped);

    fn pop_front(self) -> (E, Self::Popped) {
        let (e, q) = self.1.pop_front();
        (e, (self.0, q))
    }
}

pub trait PushBack<E, I> {
    fn push_back(self, insert: E) -> I;
}

impl<E, I> PushBack<E, (E, I)> for I {
    fn push_back(self, insert: E) -> (E, I) {
        (insert, self)
    }
}

#[test]
fn test_basic() {
    let q = TypeQueue.push_back(Some(1u32)).push_back("thing").push_back(true);
    let (mu, q) = q.pop_front();
    assert_eq!(1u32, mu.unwrap());
    let (s, q) = q.pop_front();
    assert_eq!("thing", s);
    let (b, TypeQueue) = q.pop_front();
    assert_eq!(true, b);
}
