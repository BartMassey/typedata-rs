pub struct StackQue<T>(T);

pub struct EmptySQ;

pub trait PopFront<E, R> {
    type Popped;

    fn pop_front(self) -> (E, Self::Popped);
}

impl<E> PopFront<E, EmptySQ> for StackQue<(E, EmptySQ)> {
    type Popped = EmptySQ;

    fn pop_front(self) -> (E, Self::Popped) {
        (self.0 .0, EmptySQ)
    }
}

impl<E, E1, E2, R, R2> PopFront<E, StackQue<R>> for StackQue<(E1, (E2, R2))>
    where (E2, R2): PopFront<E, R>
{
    type Popped = StackQue<(E1, <(E2, R2) as PopFront<E, R>>::Popped)>;

    fn pop_front(self) -> (E, Self::Popped) {
        let (e, q) = self.0 .1.pop_front();
        (e, StackQue((self.0 .0, q)))
    }
}

pub trait Push<E, I> {
    fn push(self, insert: E) -> I;
}

impl<E> Push<E, StackQue<(E, EmptySQ)>> for EmptySQ {
    fn push(self, insert: E) -> StackQue<(E, EmptySQ)> {
        StackQue((insert, EmptySQ))
    }
}

impl<E, I> Push<E, StackQue<(E, I)>> for StackQue<I> {
    fn push(self, insert: E) -> StackQue<(E, I)> {
        StackQue((insert, self.0))
    }
}

#[test]
fn test_basic() {
    let q = EmptySQ.push(Some(1u32)).push("thing").push(true);
    let (mu, q) = q.pop_front();
    assert_eq!(1u32, mu.unwrap());
    let (s, q) = q.pop_front();
    assert_eq!("thing", s);
    let (b, EmptySQ) = q.pop_front();
    assert_eq!(true, b);
}
