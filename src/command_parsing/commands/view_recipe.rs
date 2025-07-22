use std::collections::HashMap;

use crate::{command_parsing::command_struct::Command, item_utils::recipe::recipe::Recipe};

/// This prints all the recipes
pub fn view_recipes_cmd(_cmd: Command, recipes: &mut HashMap<String, Recipe>) {
    for rec in recipes {
        println!("{rec:?}");
    }
}
