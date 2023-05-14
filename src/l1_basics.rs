///
/// References, sometimes known as "borrowed pointers', are only valid for a limited duration.
/// References are zero-cost abstractions, which means that they don't incur any runtime overhead.
/// 
/// At runtime a reference is simply a pointer, nothing more.
/// Therefore, avoiding C's problems with dangling pointers requires a compile-time safety check.
///
/// The basis for the check is the notion of "lifetimes".
///

///
/// The compiler will only allow a borrow if it can guarantee that the data will not
/// be reassigned or moved or dropped for the lifetime of the pointer.
///
/// 

#[rustfmt::skip]
#[test]
fn non_overlapping_scopes1() {
    let mut s = String::from("hello");
    let r2 = &mut s; // r2's scope starts and ends here
    {
        let r1 = &s; // ------------ +
                                //   |
        println!("r1: {r1:?}"); // --+
    } 
}

#[rustfmt::skip]
#[test]
fn non_overlapping_scope2() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s; // ------------ +
                                    //   |
        println!("r1: {r1:?}");     // --+
    } // r1 goes out of scope here,
      // so we can make
      // a new reference with no problems.
    let r2 = &mut s; // -----------------+
    r2.push_str(", world!"); //          |
    println!("r2: {r2:?}");  //----------+
}

#[rustfmt::skip]
#[test]
fn non_overlapping_scope3() {
    let mut s = String::from("hello");
    let r1 = &mut s; // ---------------- +
                                    //   |
    println!("r1: {r1:?}");         // --+

    let r2 = &mut s; // -----------------+
    r2.push_str(", world!"); //          |
    println!("r2: {r2:?}");  //----------+
}

#[cfg(feature = "skip")]
#[rustfmt::skip]
#[test]
fn overlapping_scopes1() { // too conservative?
    let mut s = String::from("hello");
    let r2 = &mut s; // ---------------------+
                                         //  |
    let r1 = &s; // ---------------- +   //  |
                                //   |   //  |
    println!("r1: {r1:?}");     // --+   //  |
                                         //  |
    r2.push('!');                        //  |
    println!("r2: {r2:?}"); // --------------+ 
}

#[cfg(feature = "skip")]
#[rustfmt::skip]
#[test]
fn overlapping_scopes2() { // too conservative?
    let mut s = String::from("hello");
    let r2 = &mut s; // ---------------------+
    {                                    //  |
        let r1 = &s; // ------------ +   //  |
                                //   |   //  |
        println!("r1: {r1:?}"); // --+   //  |
    }                                    //  |
    r2.push('!');                        //  |
    println!("r2: {r2:?}"); // --------------+ 
}

#[rustfmt::skip]
#[test]
fn is_borrow_checker_too_conservative() {
    fn dummy(s: &String) {
        unsafe {
            let ss = s as *const String as *mut String;
            *ss = String::from("world");
        }
    }

    let mut s = String::from("hello");
    {
        dummy(&s); // We got a problem!!!
    }

    println!("s: {}", s); 
}

#[cfg(feature = "skip")]
#[test]
fn is_borrow_checker_too_conservative2() {
    // too conservative
    fn dummy(s: &String) {
        unsafe {
            let ss = s as *const String as *mut String;
            *ss = String::from("world");
        }
    }

    let mut s = String::from("hello");
    let r2 = &mut s;
    {
        dummy(&s); // We got a problem!!!
    }
    println!("r2: {}", r2);
}

#[rustfmt::skip]
#[test]
fn non_overlapping_scope4() {
    struct Foo {
        f: i32,
    }

    fn some_condition() -> bool {
        true
    }

    fn example3() -> i32 {
        let mut x = Foo { f: 3 };
        if some_condition() {
            let y = &x.f; // -+ // borrow of x
            return *y;    //  |
        }                 // -+

        x = Foo { f: 4 }; // -+ // drop occurs here
        // ...            //  |
        todo!()           //  |
    }                     // -+
}
#[cfg(feature = "skip")]
#[rustfmt::skip]
#[test]
fn overlapping_scope4() {
    struct Foo {
        f: i32,
    }

    fn some_condition() -> bool {
        true
    }

    fn example3() -> i32 {
        let mut x = Foo {f: 3};
        let y = &x.f;
        x = Foo {f: 4};  // Error reported here.
        *y
    }    

    println!("{}", example3())               
}

#[test]
fn ref_mut_do_not_downgrade_to_ref() {
    #[derive(Debug)]
    struct Foo;

    impl Foo {
        fn bar(&self) -> &Foo {
            self
        }

        fn baz(&mut self) -> &Foo {
            self
        }
    }

    let mut poo = Foo;
    let poo_ref1 = &poo;
    let poo_ref2 = poo.bar();

    println!("{poo_ref1:?}");

    /* Why not compile? */
    let mut poo = Foo;
    let poo_ref1 = &poo;
    // let poo_ref2 = poo.baz(); // mutable borrow occurs here

    println!("{poo_ref1:?}");
}

/**
 * Reference Types
 */

// The exact lifetime of `'a` is determined at each call site.  We'll explore
// what this means in more depth later.
//
// The lifetime of `b` works the same, we just didn't give it a name.
fn example<'a>(a: &'a str, b: &str) {
    // Literal strings are `&'static str`
    let s = "literal";

    // The lifetime of local borrows are determined by compiler analysis
    // and have no names (but it's still a single lifetime).
    let local = String::new();
    let borrow = local.as_str();

    // These are the same and they just tell the compiler to infer the
    // lifetime.  In this small example that means the same thing as not
    // having a type annotation at all.
    let borrow: &str = local.as_str();
    let borrow: &'_ str = local.as_str();
}

/**
 * Static Lifetimes
 */

mod static_lifetime {

    #[test]
    fn literals() {
        let s = "hello";
        let s: &'static str = "hello";

        let k = &42;
        let mut k: &'static i32 = &42;
    }

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

/**
 * Lifetime Bounds
 */

#[test]
fn bounds_between_lifetimes() {
    /// A 'a: 'b bound means, roughly speaking, 'long: 'short.
    /// It's often read as "'a outlives 'b" and it sometimes called an "outlives bound"
    /// or "outlives relation" ==> "'a is valid for (at least) 'b".

    fn example<'a: 'b, 'b: 'a>(a: &'a str, b: &'b str) {}
    // 'a and 'b must actually be the same lifetime.

    //Nested reference &'b Foo<'a> => 'a: 'b is inferred.
}

#[test]
fn bounds_between_generic_types_and_lifetimes() {
    /// T: 'a means if the type T contains any references or other lifetimes,
    ///       they must be at least as long as 'a.
    /// => "(the type) T is valid for 'a".
    ///
    /// The most common bound of this form is T: 'static => "type T has no non-'static lifetimes"

    fn example<'a, T: 'a>(a: &'a str, t: T) {}
}

#[test]
fn reference_lifetimes() {
    /// Here's something you'll utilize in Rust all the time without thinking about it:
    ///
    /// - A &'long T coerces to a &'short T
    /// - A &'long mut T coerces to a &'short mut T
    ///
    /// The technical term is "covariant (in the lifetime)"
    /// => "the (outer) lifetime of references can shrink".

    fn from_long_to_short<'a>(value: &'a str) -> &'a str {
        value
    }

    let long = "hello, world";
    {
        let short = from_long_to_short(long);
        // fn:     &'a str -> &'a str
        // caller: &'a str -> &'a str
    }
}

/**
 * Copy and Reborrow
 *
 * Shared references (&T) implement `Copy`, but mutable references (&mut T) do not.
 */
#[test]
fn reborrows() {
    fn foo<'a>(vs: &'a mut Vec<i32>) {
        vs.push(0); // line 1 => Vec::push(&mut *v, 0)
                    // *v is reborrowed for some shorter lifetime than 'a, which ends on line 1.
        println!("{vs:?}"); // line 2
    }

    let mut vs = Vec::new();
    foo(&mut vs); // line 3
}
