// #![deny(elided_lifetimes_in_paths)]

/**
 * Lifetimes for structs
*/

#[test]
fn lifetime_for_struct() {
    #[derive(Debug)]
    // City has lifetime 'a
    struct City<'a> {
        name: &'a str, // and name also has lifetime 'a.
        date_founded: u32,
    }

    let city_names = vec!["Seoul".to_string(), "New York".to_string()];

    let my_city = City {
        name: &city_names[0],
        date_founded: 1946,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

#[cfg(feature = "skip")]
#[test]
fn struct_lifetimes() {
    use std::fmt;

    struct Adventurer<'a> {
        name: &'a str,
        hit_points: u32,
    }

    impl Adventurer {
        // Compiler currently let's you elide the lifetime
        // parameter when mentioning the struct.
        fn new(name: &str, hit_points: u32) -> Adventurer {
            Adventurer { name, hit_points }
        }

        fn take_damage(&mut self) {
            self.hit_points -= 20;
            println!("{self}");
        }
    }

    impl fmt::Display for Adventurer {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} has {} hit points.", self.name, self.hit_points)
        }
    }

    let mut billy = Adventurer::new("Billy", 100_000);
    println!("{billy}");
    billy.take_damage();
}

#[test]
fn different_lifetimes_for_mut_self() {
    struct Adventurer<'a> {
        name: &'a str,
    }

    impl<'a> Adventurer<'a> {
        fn change(&mut self, name: &'a str) {
            self.name = name;
        }

        fn modify(&'a mut self, name: &'a str) {
            self.name = name;
        }
    }

    let mut person = Adventurer { name: "John" };
    person.change("Jane"); // mutable borrow

    println!("person: {:?}", person.name); // immutable borrow

    /*
     * Why not compile?
     */

    // let mut person = Adventurer { name: "John" };
    // person.modify("Jane"); // mutable borrow

    // println!("person: {:?}", person.name); // immutable borrow
}

#[cfg(feature = "skip")]
mod exercise {
    struct First {}

    impl First {
        fn new() -> Self {
            Self {}
        }

        fn second(&self) -> Second<'_> {
            Second::new(self)
        }

        fn hello(&self) {
            println!("Hello");
        }
    }

    struct Second<'a> {
        owner: &'a First,
    }

    impl<'a> Second<'a> {
        fn new(owner: &'a First) -> Self {
            Self { owner }
        }

        fn hello(&self) {
            self.owner.hello();
        }

        fn third(&self) -> Third<'_> {
            Third::new(self.owner)
        }
    }

    struct Third<'a> {
        owner: &'a First,
    }

    impl<'a> Third<'a> {
        fn new(owner: &'a First) -> Self {
            Self { owner }
        }

        fn hello(&self) {
            self.owner.hello();
        }
    }

    #[test]
    fn pass_through() {
        let f = First::new();
        let t = {
            let sss = f.second();
            sss.third() // error: sss does not live long enough
        };
    }
}

#[test]
fn independently_borrowing_fields() {
    struct Pair {
        left: String,
        right: String,
    }

    impl Pair {
        fn foo(&mut self) {
            let left = &mut self.left;
            let right = &mut self.right;
            left.push_str("hi");
            right.push_str("there");
            println!("{left} {right}");
        }
    }
}

#[cfg(feature = "skip")]
#[test]
fn indexing_not_considered_as_splitting_borrows() {
    let mut v = vec![0, 1, 2];

    // These two do not overlap, but...
    let left = &mut v[..1];
    let right = &mut v[1..];

    // ...the borrow checker cannot recognize that
    println!("{left:?} {right:?}");
}

#[cfg(feature = "skip")]
#[test]
fn invariant_lifetime() {
    struct A<'a> {
        data: &'a mut &'a str,
    }

    let mut s = "hello";
    let a = &mut A { data: &mut s };
    *a.data = "world";

    println!("s: {}", s);
}
