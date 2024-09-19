use back_of_house::Breakfast;

mod front_of_house;
   

fn deliver_order() {}

pub mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorret_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}
pub use crate::front_of_house::hosting;
pub mod customer {
    use crate::hosting;

    // use crate::front_of_house::hosting::add_to_waitlist;
    pub fn eat_at_restaurant() {
        // Absolute path

        hosting::add_to_waitlist();
        hosting::seat_at_table();
        //     // or relative path'
        hosting::add_to_waitlist();
        hosting::seat_at_table();

        let mut meal = crate::back_of_house::Breakfast::summer("rye");
        meal.toast = String::from("wheat");
        println!("{:?}", meal);

        let order1 = crate::back_of_house::Appetizer::Salad;
    }
}
