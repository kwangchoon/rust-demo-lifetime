#![allow(non_snake_case)]

/**
 * Lifetimes for functions
*/

#[cfg(feature = "skip")]
fn foo(s: &str, t: &str) -> &str {
    todo!()
}

fn foo<'a>(s: &'a str, t: &'a str) -> &'a str {
    todo!()
}

#[test]
fn lifetime_binding_demo1() {
    'l1: {
        let s1 = "hello";
        let s2 = "rustaceans";

        let s3 = foo(s1, s2);
        //     fn: foo(&'a str, &'a str) -> &'a str
        // caller: foo(&'a str, &'a str) -> &'a str

        println!("s3: {}", s3);
    }
}

#[test]
fn lifetime_binding_demo2() {
    'l1: {
        let s1 = "hello";
        'l2: {
            let s2 = "rustaceans";

            let s3 = foo(s1, s2);
            //     fn: foo(&'a str, &'a str) -> &'a str
            // caller: foo(&'a str, &'a str) -> &'a str

            println!("s3: {}", s3);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn ok_case() {
    'l1: {
        let s1 = String::from("abcd");
        let s2 = String::from("xyz");

        // func  : longest(&'a, &'a) -> &'a
        // caller: longest(&'a, &'a) -> &'a
        let result = longest(&s1, &s2);

        println!("The longest string is {result}");
    }
}

#[cfg(feature = "skip")]
#[test]
fn error_case() {
    'l1: {
        let s1 = String::from("abcd");

        let result;
        'l2: {
            let s2 = String::from("xyz");
            // func  : longest(&'a, &'a) -> &'a
            // caller: longest(&'a, &'a) -> &'a
            result = longest(&s1, &s2);
        }
        println!("The longest string is {result}");
    }
}

// Why is this OK?
#[test]
fn ok_another_case() {
    let s1 = String::from("abcd");

    let result;
    {
        let s2 = "xyzxyz";
        // func  : longest(&'a, &'a) -> &'a
        // caller: longest(&'a, &'a) -> &'a
        result = longest(&s1, s2);
    }
    println!("The longest string is {result}");
}

#[cfg(feature = "skip")]
#[test]
fn what_is_the_lifetime_of_the_return_value_of_f() {
    // 'a can be omitted thanks to lifetime ellision rules
    fn f<'a>(s: &'a str) -> &'a str {
        s
    }

    'l1: {
        let r;
        'l2: {
            let s = String::from("hello");
            r = f(&s); // by the time r is assigned, it is guaranteed that s is still valid
        }

        println!("r: {}", r);
    }
}

#[cfg(feature = "skip")]
#[test]
fn make_this_code_compile() {
    fn my_push_back(vs: &mut Vec<&str>, v: &str) {
        vs.push(v);
    }

    let mut vs = Vec::new();
    {
        let s = String::from("hello");
        my_push_back(&mut vs, &s);
    }

    dbg!(vs);
}
