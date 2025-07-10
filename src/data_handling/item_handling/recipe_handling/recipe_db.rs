use std::{collections::HashMap, sync::Mutex};
use once_cell::sync::Lazy;

use crate::item_utils::recipe::recipe::Recipe;





static RECIPES: Lazy<Mutex<HashMap<String, Recipe>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Get a recipe by name
pub fn get_recipe(name: &str) -> Option<Recipe> {
    let recipes = RECIPES.lock().unwrap();
    recipes.get(name).cloned()
}

/// Add a new recipe (if not already exists)
pub fn add_recipe(recipe: Recipe) -> bool {
    let mut recipes = RECIPES.lock().unwrap();
    if recipes.contains_key(recipe.name()) {
        false // Recipe already exists, do not overwrite
    } else {
        recipes.insert(recipe.name().clone(), recipe);
        true
    }
}

/// Remove a recipe by name
pub fn remove_recipe(name: &str) -> bool {
    let mut recipes = RECIPES.lock().unwrap();
    recipes.remove(name).is_some()
}

/// Check if a recipe exists by name
pub fn contains_recipe(name: &str) -> bool {
    let recipes = RECIPES.lock().unwrap();
    recipes.contains_key(name)
}