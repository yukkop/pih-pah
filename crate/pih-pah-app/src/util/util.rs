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

// TODO &'static str -> Uniq(Module)
#[macro_export]
macro_rules! define_module {
    () => {
        use crate::util::module_cat_off;

        lazy_static::lazy_static! {
            /// Module path for this module, 
            /// Use it for translate text like `trans("text", Module(&MODULE))`
            /// where `Module(&MODULE)` is `Uniq` id for translate text
            static ref MODULE: &'static str = module_cat_off(module_path!());
        }
    };
}

#[macro_export]
macro_rules! rich_text {
    ($text:expr, $module:expr, $font:expr) => {
        rich_text($text.to_string(), Module($module), $font)
    };
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}