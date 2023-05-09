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

#[cfg(feature = "skip")]
#[rustfmt::skip]
#[test]
fn overlapping_scopes() { // too conservative?
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

#[cfg(feature = "skip")]
#[rustfmt::skip]
#[test]
fn borrow_checker_is_conservative() { // too conservative
    fn dummy(s: &str) {
        unsafe {
            let ss = s as *const _ as *mut _;
            *ss = "world";
        }
    }

    let mut s = String::from("hello");
    let r2 = &mut s; // ---------------------+
                                         //  |
    dummy(&s); // We got a problem!!!    //  |
                                         //  |
    r2.push('!');                        //  |
    println!("r2: {r2:?}"); // --------------+ 
}

#[rustfmt::skip]
#[test]
fn borrow_checker_is_conservative2() { // too conservative
    fn dummy(s: &str) {
        unsafe {
            let ss = s as *const _ as *mut _;
            *ss = "world";
        }
    }

    let mut s = String::from("hello");
                                         //  |
    dummy(&s); // We got a problem!!!    //  |
                                         //  |
    println!("s: {s:?}"); // --------------+ 
}

#[cfg(feature = "skip")]
#[test]
fn borrow_checker_is_conservative_explation_not_really2() {
    // too conservative
    fn dummy(s: &String) {
        unsafe {
            let ss = s as *const String as *mut String;
            *ss = String::from("world");
        }
    }

    let mut s = String::from("hello");
    let r2 = &mut s;
    dummy(&s); // We got a problem!!!
    println!("r2: {}", r2);
}

#[rustfmt::skip]
#[test]
fn borrow_checker_is_conservative_explation_not_really() { // too conservative
    fn dummy(s: &String) {
        unsafe {
            let ss = s as *const String as *mut String;
            *ss = String::from("world");
        }
    }

    let mut s = String::from("hello");
    dummy(&s); // We got a problem!!!
    println!("s: {}", s); 
}
