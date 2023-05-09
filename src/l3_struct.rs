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

// #[cfg(feature = "skip")]
#[test]
fn struct_lifetimes() {
    struct Adventurer<'a> {
        name: &'a str,
        hit_points: u32,
    }

    impl Adventurer<'_> {
        fn take_damage(&mut self) {
            self.hit_points -= 20;
            println!("{} has {} hit points left!", self.name, self.hit_points);
        }
    }

    impl std::fmt::Display for Adventurer<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} has {} hit points.", self.name, self.hit_points)
        }
    }

    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };

    println!("{billy}");
    billy.take_damage();
}
