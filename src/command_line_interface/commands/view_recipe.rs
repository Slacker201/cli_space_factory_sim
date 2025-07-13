use crate::{ command_line_interface::command_struct::Command, item_utils::recipe::recipe::Recipe };

/// This prints all the recipes
pub fn view_recipes_cmd(_cmd: Command, recipes: &mut Vec<Recipe>) {
    for rec in recipes {
        println!("{:?}", rec);
    }
}
