use std::collections::VecDeque;

/// Extension method `queue_iter` for any type implementing `IntoIterator`.
pub trait IteratorExt
where
    Self: IntoIterator + Sized,
{
    /// Create an `Iterator` allowing for enqueuing elements to its end.
    ///
    /// # Examples
    ///
    /// ```
    /// use enqueue::IteratorExt;
    ///
    /// let i = std::iter::once(666);
    /// let mut i = i.queue_iter();
    ///
    /// i.enqueue(42);
    /// assert_eq!(i.next(), Some(666));
    /// assert_eq!(i.next(), Some(42));
    /// assert_eq!(i.next(), None);
    ///
    /// i.enqueue(42);
    /// assert_eq!(i.next(), Some(42));
    /// ```
    fn queue_iter(self) -> QueueIter<Self::IntoIter>;
}

impl<T> IteratorExt for T
where
    T: IntoIterator,
{
    fn queue_iter(self) -> QueueIter<Self::IntoIter> {
        queue_iter(self)
    }
}

/// Create an `Iterator` allowing for enqueuing elements to its end.
///
/// # Examples
///
/// ```
/// use enqueue::queue_iter;
///
/// let i = std::iter::once(666);
/// let mut i = queue_iter(i);
///
/// i.enqueue(42);
/// assert_eq!(i.next(), Some(666));
/// assert_eq!(i.next(), Some(42));
/// assert_eq!(i.next(), None);
///
/// i.enqueue(42);
/// assert_eq!(i.next(), Some(42));
/// ```
pub fn queue_iter<I>(initial: I) -> QueueIter<I::IntoIter>
where
    I: IntoIterator,
{
    QueueIter {
        initial: initial.into_iter(),
        next: VecDeque::default(),
    }
}

/// An `Iterator` allowing for enqueuing elements to its end. Elements can be added anytime, even
/// after calling `next` has returned `None`, i.e. this `Iterator` can return `Some` after `None`.
pub struct QueueIter<I>
where
    I: Iterator,
{
    initial: I,
    next: VecDeque<I::Item>,
}

impl<I> QueueIter<I>
where
    I: Iterator,
{
    /// Enqueue an element to the end of this `Iterator`.
    pub fn enqueue(&mut self, item: I::Item) {
        self.next.push_back(item)
    }
}

impl<I> Iterator for QueueIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.initial.next().or_else(|| self.next.pop_front())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut numbers = (0..10).queue_iter();
        let mut sum = 0;

        while let Some(n) = numbers.next() {
            sum += n;
            if n < 5 {
                numbers.enqueue(20);
            }
        }

        // The sum of 0..10 is 45 and we enqueue 5 times 20, i.e. get another 100: 45 + 100.
        assert_eq!(sum, 145);

        numbers.enqueue(42);
        assert_eq!(numbers.next(), Some(42));
    }
}
