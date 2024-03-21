mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        
        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house;

    pub fn eat_at_restaurant() {
        // relative path:
        // front_of_house::hosting::add_to_waitlist();
        // can also use absolute path:
        // crate::front_of_house::hosting::add_to_waitlist();
        // with using, can just use hosting
        hosting::add_to_waitlist();

        // enums and variants are public
        // can use absolute
        let order1 = crate::back_of_house::Appetizer::Soup;
        // or use... use
        let order2 = back_of_house::Appetizer::Salad;

        // order breakfast in summer with Rye toast
        let mut meal = crate::back_of_house::Breakfast::summer("Rye");
        // change bread
        meal.toast = String::from("Wheat");
        // order
        println!("I'd like {} toast please!", meal.toast);

        // unable to view or change seasonal_fruit
        // meal.seasonal_fruit = String::from("Blueberries");
    }
}

fn deliver_order() {}