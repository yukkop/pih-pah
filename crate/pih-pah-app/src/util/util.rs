/// Remove the first part of a module path.
pub fn module_cat_off(module: &str) -> &str {
    module.splitn(3, ':').nth(2).unwrap_or(module.clone())
}

/// Remove the specified part of a module path.
/// return same module if part_to_remove not found
pub fn module_cut_out<'a>(module: &'a str, part_to_remove: &str) -> &'a str {
    module
        .split::<'a, &str>("::")
        .filter(|&part| part != part_to_remove)
        .collect::<Vec<&str>>()
        .join("::").leak()
}

use bevy::utils::hashbrown::HashMap;
use strum::IntoEnumIterator;

/// Validate that all enum variants are in the hash map
pub fn validate_hash_map<K, V>(hash_map: &HashMap<K, V>) -> bool 
where
    K: Eq + std::hash::Hash + Copy + IntoEnumIterator,
    K::Iterator: Iterator<Item = K>,
{
    let all_keys = K::iter().collect::<Vec<_>>();
    if hash_map.len() != all_keys.len() {
        return false;
    }

    for key in all_keys {
        if !hash_map.contains_key(&key) {
            return false;
        }
    }

    true
}