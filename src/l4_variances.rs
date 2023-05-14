#![deny(elided_lifetimes_in_paths)]

mod motivations {
    use std::collections::HashSet;

    // if a string lasts for the whole process it should also last for any part of it.
    fn lifetime_shortener<'a>(s: &'static str) -> &'a str {
        s
    }

    // Let's consider more complex case.
    // #[rustfmt::skip]
    #[cfg(feature = "skip")]
    fn hash_set_shortener<'a, 'b>(s: &'a mut HashSet<&'static str>) -> &'a mut HashSet<&'b str> {
        s
    }

    /**
     * Does this work? Why or why not?
     */
    #[cfg(feature = "skip")]
    fn hash_set_example() {
        // Consider this HashSet over static strings.
        let mut my_set: HashSet<&'static str> = HashSet::from_iter(["static"]);

        // Do you think this can work?
        let owned_string: String = "non_static".to_owned();
        my_set.insert(&owned_string);

        // Doesn't seem like it can, right? my_set promises that the &strs inside it
        // are all 'static, but we tried to put in an owned string scoped to this
        // function.
    }

    #[cfg(feature = "skip")]
    fn hash_set_counterexample() {
        let mut my_set: HashSet<&'static str> = HashSet::from_iter(["static"]);
        let owned_string: String = "non_static".to_owned();

        // If we pretend that hash_set_shortener works...
        let shorter_set = hash_set_shortener(&mut my_set);

        // then you could use `shorter_set` to insert a non-static string:
        shorter_set.insert(&owned_string);

        // Now we can drop `shorter_set` to regain the ability to use `my_set`:
        std::mem::drop(shorter_set);

        // And my_set now has a non-static string in it. Whoops!
    }
    /// It isn't just &mut which is problematic in this way. This also occurs with
    /// any sort of interior mutability, like RefCell, OnceCell, or Mutex -- anything
    /// inside some sort of mutable context has this issue.

    // Now, what about a hypothetical "lengthener" function?
    #[cfg(feature = "skip")]
    fn lifetime_lengthener<'a>(s: &'a str) -> &'static str {
        s
    } // This is clearly bogus, right? You can't just turn an arbitrary borrowed
      // string and make it last the duration of the entire process.

    #[rustfmt::skip]
    #[cfg(feature = "skip")]
    fn hash_set_lengthener<'a, 'b>(
        s: &'a mut HashSet<&'b str>,
    ) -> &'a mut HashSet<&'static str> {
        s
    }

    // But what about this? fn is a pointer to a function that takes an arbitrary borrowed string.
    fn fn_ptr_lengthener<'a>(f: fn(&'a str) -> ()) -> fn(&'static str) -> () {
        f
    }

    fn fn_ptr_example() {
        fn short(arg: &str) {
            todo!();
        }

        let s = "hello";
        short(s);
        let long_fn = fn_ptr_lengthener(short);
        long_fn(s);
    }
}

#[test]
fn covariance_lifetime_can_be_shortened() {
    // T <: U => F[T] <: F[U]
    fn covariant<'a, 'b: 'a>(value: &'b str) -> &'a str {
        value
    }

    let mut long = "hello";
    {
        let short = long;
        let short = covariant(long);
        //     fn: covariant(&'b str) -> &'a str
        // caller: covariant(&'b str) -> &'a str
    }
}

#[test]
fn covariance_lifetime_can_be_shortened2() {
    fn covariant<'a>(short: &'a str, long: &'a str) -> &'a str {
        short
    }

    let long: &'static str = "hello";
    {
        let short = String::from("world");
        let mut value = covariant(&short, long);
        //     fn: covariant(&'short str, &'long str) -> &'short str
        // caller: covariant(&'short str, &'long str) -> &'short str
    }
}

///
/// Nested borrows and invariance
///
/// A &'medium &'long U coerces to a &'short &'short U
/// A &'medium mut &'long mut U coerces to a &'short mut &'long mut U...
///     ...but not to a &'short mut &'short mut U
///
/// We say that &mut T is invariant in T, which means any lifetimes in T
/// cannot change (grow or shrink) at all.

#[cfg(feature = "skip")]
#[test]
fn invariance_lifetime_cannot_be_changed() {
    fn bar(vs: &mut Vec<&'static str>) {
        let w: &mut Vec<&'_ str> = vs; // call the lifetime 'w

        let local = "Local data".to_string();
        w.push(&local);
    } // `local` drops
} // Now, *vs have dangling reference ...inside the Vec

#[cfg(feature = "skip")]
#[test]
fn invariance_other_cases() {
    use std::cell::Cell;

    fn bar(cell: &Cell<&'static str>) {
        let c: &Cell<&'_ str> = cell; // call the lifetime 'c

        let local = "Local data".to_string();
        c.set(&local);
    } // `local` drops
} // Now *cell have dangling reference ...inside the Cell

#[test]
fn test() {
    // But what about this? fn is a pointer to a function that takes an arbitrary borrowed string.
    fn fn_ptr_lengthener<'a>(f: fn(&'a str) -> ()) -> fn(&'static str) -> () {
        f
    }

    fn fn_ptr_example() {
        fn short(arg: &str) {
            todo!();
        }

        let s = "hello";
        short(s);
        let long_fn = fn_ptr_lengthener(short);
        long_fn(s);
    }
}

#[cfg(feature = "skip")]
mod variances_strikes_back {
    use std::collections::HashSet;
    use std::fmt;

    #[derive(Debug)]
    struct Message<'msg> {
        message: &'msg str,
    }

    struct SimpleMessageCollector<'a> {
        list: &'a mut Vec<Message<'a>>,
    }

    impl<'a> SimpleMessageCollector<'a> {
        // This adds a message to the end of the list.
        fn add_message(&mut self, message: Message<'a>) {
            self.list.push(message);
        }
    }

    fn message_example() {
        // Here's a simple pool of messages.
        let mut message_pool: HashSet<String> = HashSet::new();
        message_pool.insert("ten".to_owned());
        message_pool.insert("twenty".to_owned());

        // All right, let's try collecting some messages!
        collect(&message_pool);
    }

    fn collect<'msg>(message_pool: &'msg HashSet<String>) {
        // OK, one more time.
        let mut list = vec![];

        // Collect some messages.
        let mut collector = SimpleMessageCollector { list: &mut list };
        for message in message_pool {
            collector.add_message(Message { message });
        }

        // Finally, display them.
        let m = &list;
    }
}

#[test]
fn anti_pattern() {
    #[derive(Debug)]
    struct Node<'a>(&'a str);
    fn example_1<'a>(node: &'a mut Node<'a>) {}

    struct DroppingNode<'a>(&'a str);
    impl Drop for DroppingNode<'_> {
        fn drop(&mut self) {}
    }
    fn example_2<'a>(node: &'a mut DroppingNode<'a>) {}

    let local = String::new();

    // let mut node_a = Node(&local);
    // // You can do this once and it's ok...
    // example_1(&mut node_a);

    // let mut node_b = Node(&local);
    // // ...but then you can't use the node directly ever again
    // example_1(&mut node_b);
    // println!("{node_b:?}");

    // let mut node_c = DroppingNode(&local);
    // // And this doesn't work at all
    // example_2(&mut node_c);
}
