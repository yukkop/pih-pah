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
        .join("::")
        .leak()
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

#[cfg(test)]
pub mod test {
    use std::time::{Duration, Instant};

    use bevy::prelude::{Deref, DerefMut};
    use log::Level;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
    pub struct Times(u64);

    impl Into<u64> for Times {
        fn into(self) -> u64 {
            self.0
        }
    }

    impl Into<usize> for Times {
        fn into(self) -> usize {
            self.0 as usize
        }
    }

    impl Into<u32> for Times {
        fn into(self) -> u32 {
            self.0 as u32
        }
    }

    impl Into<i32> for Times {
        fn into(self) -> i32 {
            self.0 as i32
        }
    }

    impl From<u64> for Times {
        fn from(times: u64) -> Self {
            Self(times)
        }
    }

    impl From<usize> for Times {
        fn from(times: usize) -> Self {
            Self(times as u64)
        }
    }

    impl From<u32> for Times {
        fn from(times: u32) -> Self {
            Self(times as u64)
        }
    }

    impl From<i32> for Times {
        fn from(times: i32) -> Self {
            Self(times as u64)
        }
    }

    impl Default for Times {
        /// Value that may be enough for most cases
        fn default() -> Self {
            Self(100000)
        }
    }

    /// Enable logging for debug
    pub fn enable_loggings() {
        use std::env;
        use std::io::Write;

        let _ = env::set_var("RUST_LOG", "debug");
        // FIXME: colorize logs
        // TODO: colorize thorwed args
        let _ = env_logger::builder()
            .is_test(true)
            .format(|buf, record| {
                let mut style = buf.style();
                let level = record.level();
                match level {
                    Level::Trace => style.set_color(env_logger::fmt::Color::Magenta),
                    Level::Debug => style.set_color(env_logger::fmt::Color::Blue),
                    Level::Info => style.set_color(env_logger::fmt::Color::Green),
                    Level::Warn => style.set_color(env_logger::fmt::Color::Yellow),
                    Level::Error => style.set_color(env_logger::fmt::Color::Red),
                };

                writeln!(buf, "{}: {}", style.value(level), record.args())
            })
            .try_init();
    }

    /// Measure time of predicate
    pub fn measure_time<F: Copy>(predicate: F, times: Times) -> Duration
    where
        F: FnOnce() -> (),
    {
        let start = Instant::now();
        for _ in 0..times.clone().into() {
            predicate();
        }
        let global_duration = start.elapsed();
        global_duration / times.into()
    }
}
