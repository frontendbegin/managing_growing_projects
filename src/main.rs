use ::rand::Rng;
use std::{collections::HashMap, fmt::Result, io::Result as IoResult};

use crate::garden::vegetables::Asparagus;
use managing_growing_projects::customer::eat_at_restaurant;

pub mod garden;
fn main() {
    let plant = Asparagus {};
    println!("The plant is {:?}", plant);
    eat_at_restaurant();

    let mut map = HashMap::new();
    map.insert(1, 2);
}
// fn function()-> Result{
//     /* */
// }
// fn function() -> IoResult<>{

// }
