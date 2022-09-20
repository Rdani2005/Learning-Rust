mod front_of_house;

fn deliver_order() {

}

mod back_of_the_house {
    fn coock_order() {}

    fn fix_incorrect_order() {
        coock_order();
        super::deliver_order();
    }

    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("apples"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting;
pub use crate::back_of_the_house::Breakfast;

md costumer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        let mut meal = Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    }
}

