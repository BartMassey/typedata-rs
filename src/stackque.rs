//! This module provides a "stack queue", a structure
//! supporting adding to the back of the queue and popping
//! from the back or front.
//!
//! This data structure is both heterogenous and statically
//! typed.  See the crate docs for an explanation, and the
//! example below as an illustration of usage.
//! 
//! Trait implementations here are such that only legitimate
//! `StackQue`s can be pushed onto: this is a consequence of
//! the private field of `StackQue` and the orphan rule.
//!
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
//! let (s, EmptySQ) = q.pop_front();
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

// Case: Push onto empty stack.
impl<E> Push<E, StackQue<(E, EmptySQ)>> for EmptySQ {
    fn push(self, insert: E) -> StackQue<(E, EmptySQ)> {
        StackQue((insert, EmptySQ))
    }
}

// Case: Push onto non-empty stack.
impl<E, I> Push<E, StackQue<(E, I)>> for StackQue<I> {
    fn push(self, insert: E) -> StackQue<(E, I)> {
        StackQue((insert, self.0))
    }
}

/// Trait for popping a `StackQue`: provides the
/// `pop()` operation. From a queue's point of view,
/// `pop()` is "`pop_back()`".
pub trait Pop<E, R> {
    fn pop(self) -> (E, R);
}

// Case: Pop last element.
impl<E> Pop<E, EmptySQ> for StackQue<(E, EmptySQ)> {
    fn pop(self) -> (E, EmptySQ) {
        (self.0 .0, EmptySQ)
    }
}

// Case: Pop non-last element.
impl<E, E1, R1> Pop<E, StackQue<(E1, R1)>> for StackQue<(E, (E1, R1))> {
    fn pop(self) -> (E, StackQue<(E1, R1)>) {
        (self.0 .0, StackQue(self.0 .1))
    }
}

/// Trait for popping the front element from a `StackQue`
/// viewed as a queue: provides the `pop_front()` operation.
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

impl<E, E1, E2, R, R2> PopFront<E, R> for StackQue<(E1, (E2, R2))>
    where (E1, (E2, R2)): PopFront<E, R>
{
    type Popped = StackQue<<(E1, (E2, R2)) as PopFront<E, R>>::Popped>;

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
    let (mu, EmptySQ) = q.pop();
    assert_eq!(1u32, mu.unwrap());
}

#[test]
fn test_que_basic() {
    let q = EmptySQ.push(Some(1u32)).push("thing").push(true);
    let (mu, q) = q.pop_front();
    assert_eq!(1u32, mu.unwrap());
    let (s, q) = q.pop_front();
    assert_eq!("thing", s);
    let (b, EmptySQ) = q.pop_front();
    assert_eq!(true, b);
}
