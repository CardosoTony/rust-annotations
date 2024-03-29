use std::fmt;

use crate::utils::terminal::selected_option;

pub fn structs() {
    selected_option("Structs");

    struct User {
        name: String,
        age: u8,
        email: String,
        active: bool,
    }

    let user = User {
        name: String::from("John"),
        age: 25,
        email: String::from("john@email.com"),
        active: true,
    };

    println!(
        "User: {}, {} years old, active: {}",
        user.name, user.age, user.active
    );
    println!("Email: {}\n", user.email);
}

pub fn enums() {
    selected_option("Enums");

    enum DayOfWeek {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    impl fmt::Display for DayOfWeek {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DayOfWeek::Sunday => write!(f, "Sunday"),
                DayOfWeek::Monday => write!(f, "Monday"),
                DayOfWeek::Tuesday => write!(f, "Tuesday"),
                DayOfWeek::Wednesday => write!(f, "Wednesday"),
                DayOfWeek::Thursday => write!(f, "Thursday"),
                DayOfWeek::Friday => write!(f, "Friday"),
                DayOfWeek::Saturday => write!(f, "Saturday"),
            }
        }
    }

    println!("Day: {}", DayOfWeek::Sunday);
    println!("Day: {}", DayOfWeek::Monday);
    println!("Day: {}", DayOfWeek::Tuesday);
    println!("Day: {}", DayOfWeek::Wednesday);
    println!("Day: {}", DayOfWeek::Thursday);
    println!("Day: {}", DayOfWeek::Friday);
    println!("Day: {}\n", DayOfWeek::Saturday);
}

pub fn trais() {
    selected_option("Trais");

    trait Animal {
        fn make_sound(&self);
    }

    struct Dog;
    impl Animal for Dog {
        fn make_sound(&self) {
            println!("Woof!");
        }
    }

    struct Cat;
    impl Animal for Cat {
        fn make_sound(&self) {
            println!("Meow\n");
        }
    }

    fn make_animal_sound(animal: &dyn Animal) {
        animal.make_sound();
    }

    make_animal_sound(&Dog);
    make_animal_sound(&Cat);
}
