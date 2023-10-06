use argon2::{Config, Variant, Version};

pub fn get_argon2_config() -> Config<'static> {
    Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 192,
        lanes: 4,
        // thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    }
}
