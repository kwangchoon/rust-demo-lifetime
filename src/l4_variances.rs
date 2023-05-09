mod static_lifetime {
    // Make a constant with `'static` lifetime.
    static NUM: i32 = 18;

    // Returns a reference to `NUM` where its `'static`
    // lifetime is coerced to that of the input argument.
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    /**
     * As a trait bound, it means the type does not contain any non-static references.
     * Eg. the receiver can hold on to the type for as long as they want and it will
     * never become invalid until they drop it.
     *
     * It's important to understand this means that any owned data always passes a
     * 'static lifetime bound, but a reference to that owned data generally does not:
     */
    #[test]
    fn bounds() {
        use std::fmt::Debug;

        fn print_it(input: impl Debug + 'static) {
            println!("'static value passed in is: {input:?}");
        }

        // i is owned and contains no references, thus it's 'static:
        let i = 5;

        print_it(i);
        print_it(String::from("hello"));

        // Oops, &i only has the lifetime defined by the scope of
        // print_it(), so it's not 'static:

        // print_it(&i);
        // print_it(&String::from("hello")); // same here
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

mod variances_strikes_back2 {
    use std::collections::HashSet;
    use std::fmt;

    #[derive(Debug)]
    struct Message<'msg> {
        message: &'msg str,
    }

    struct SimpleMessageCollector<'a, 'msg> {
        list: &'a mut Vec<Message<'msg>>,
    }

    impl<'a, 'msg> SimpleMessageCollector<'a, 'msg> {
        // This adds a message to the end of the list.
        fn add_message(&mut self, message: Message<'msg>) {
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
