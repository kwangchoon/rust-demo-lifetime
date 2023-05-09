/**
 * Lifetimes for functions
*/
mod lifetimes_for_functions {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    #[test]
    fn ok1() {
        let s1 = String::from("abcd");
        let s2 = String::from("xyz");

        // func  : longest(&'f, &'f) -> &'f
        // caller: longest(&'f, &'f) -> &'f
        let result = longest(&s1, &s2);
        println!("The longest string is {result}");
    }

    #[cfg(feature = "skip")]
    #[test]
    fn oops() {
        let s1 = String::from("abcd");

        let result;
        {
            // func  : longest(&'s, &'s) -> &'s
            // caller: longest(&'l, &'s) -> &'s
            let s2 = String::from("xyz");
            result = longest(&s1, &s2);
        }
        println!("The longest string is {result}");
    }

    // Why is this OK?
    #[test]
    fn ok2() {
        let s1 = String::from("abcd");

        let result;
        {
            let s2 = "xyzxyz";
            // func  : logest(&'s1, &'staic) -> &'s1
            // caller: logest(&'s1, &'static) -> &'s1
            result = longest(s1.as_str(), s2);
        }
        println!("The longest string is {result}");
    }
}

#[cfg(feature = "skip")]
#[test]
fn what_is_the_lifetime_of_the_return_value_of_f() {
    // 'a can be omitted thanks to lifetime ellision rules
    fn f<'a>(s: &'a str) -> &'a str {
        s
    }

    let r;
    {
        let s = String::from("hello");
        r = f(&s); // by the time r is assigned, it is guaranteed that s is still valid
    }

    println!("r: {}", r);
}

#[cfg(feature = "skip")]
#[test]
fn make_this_code_compile() {
    fn my_push_back(vs: &mut Vec<str>, v: &str) {
        vs.push(v);
    }

    let mut vs = Vec::new();
    {
        let s = String::from("hello");
        my_push_back(&mut vs, &s);
    }

    dbg!(vs);
}
