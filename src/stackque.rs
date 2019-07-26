//! Trait implementations here are such that only legitimate
//! `StackQue`s can be pushed onto. The traits are not
//! sealed to provide the opportunity to extend function,
//! but the orphan rule makes this difficult.
//! # Examples
//!
//! ```
//! use typedata_rs::stackque::*;
//! 
//! let q = EmptySQ.push(Some(1u32)).push("thing").push(true);
//! // true thing 1 →
//! let (mu, q) = q.pop_front();
//! assert_eq!(Some(1u32), mu);
//! // true thing →
//! let q = q.push('a');
//! // a thing true →
//! let (b, q) = q.pop();
//! assert_eq!('a', b);
//! // thing true →
//! let (s, q) = q.pop();
//! assert_eq!(true, s);
//! // true →
//! let (s, StackQue(EmptySQ)) = q.pop_front();
//! assert_eq!("thing", s);
//! ```

/// Wrapper type to help with type safety vs incorrect usage.
pub struct StackQue<T>(T);

/// Type of empty `StackQue`. Pushing onto this creates a
/// new `StackQue`. Popping from a `StackQue` with one
/// element from either end returns this as the queue.
pub struct EmptySQ;

/// Trait for pushing onto a `StackQue`: provides the
/// `push()` operation. From a queue's point of view,
/// `push()` is "`push_back()`".
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

/// Trait for popping a `StackQue`: provides the
/// `pop()` operation. From a queue's point of view,
/// `pop()` is "`pop_back()`".
pub trait Pop<E, R> {
    fn pop(self) -> (E, StackQue<R>);
}

impl<E, R> Pop<E, R> for StackQue<(E, R)> {
    fn pop(self) -> (E, StackQue<R>) {
        (self.0 .0, StackQue(self.0 .1))
    }
}

/// Trait for popping the front element from a `StackQue`
/// viewed as a queue: provides the `pop_front()` operation.
pub trait PopFront<E, R> {
    type Popped;

    fn pop_front(self) -> (E, Self::Popped);
}

impl<E, Q, R> PopFront<E, R> for StackQue<Q>
    where Q: PopFront<E, R>
{
    type Popped = StackQue<<Q as PopFront<E, R>>::Popped>;

    fn pop_front(self) -> (E, Self::Popped) {
        let (e, q) = self.0.pop_front();
        (e, StackQue(q))
    }
}

impl<E> PopFront<E, EmptySQ> for (E, EmptySQ) {
    type Popped = EmptySQ;

    fn pop_front(self) -> (E, Self::Popped) {
        (self.0, EmptySQ)
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

#[test]
fn test_stack_basic() {
    let q = EmptySQ.push(Some(1u32)).push("thing").push(true);
    let (b, q) = q.pop();
    assert_eq!(true, b);
    let (s, q) = q.pop();
    assert_eq!("thing", s);
    let (mu, StackQue(EmptySQ)) = q.pop();
    assert_eq!(1u32, mu.unwrap());
}

#[test]
fn test_que_basic() {
    let q = EmptySQ.push(Some(1u32)).push("thing").push(true);
    let (mu, q) = q.pop_front();
    assert_eq!(1u32, mu.unwrap());
    let (s, q) = q.pop_front();
    assert_eq!("thing", s);
    let (b, StackQue(EmptySQ)) = q.pop_front();
    assert_eq!(true, b);
}
