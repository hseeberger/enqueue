# enqueue

Rust library providing an `Iterator` allowing for enqueuing elements to its end.

```rust
use enqueue::IteratorExt;

let i = std::iter::once(666);
let mut i = i.queue_iter();

i.enqueue(42);
assert_eq!(i.next(), Some(666));
assert_eq!(i.next(), Some(42));
assert_eq!(i.next(), None);

i.enqueue(42);
assert_eq!(i.next(), Some(42));
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
