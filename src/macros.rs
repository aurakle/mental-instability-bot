#[macro_export]
macro_rules! get_mappings_cache {
    ($arg:expr) => {{
        $arg.data
            .write()
            .await
            .get_mut::<$crate::MappingsCacheKey>()
            .expect("No mappings cache?")
    }};
}
