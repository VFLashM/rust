use std::hash::BuildHasherDefault;

pub use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<FxHasher>>;
pub type FxIndexSet<V> = indexmap::IndexSet<V, BuildHasherDefault<FxHasher>>;

#[macro_export]
macro_rules! fx_hash_map {
    () => { $crate::fx::FxHashMap::new_with_prof(file!(), line!()) }
}

#[macro_export]
macro_rules! fx_hash_map_with_capacity {
    ($e:expr) => { $crate::fx::FxHashMap::with_capacity_and_prof($e, file!(), line!()) }
}

#[macro_export]
macro_rules! fx_hash_set {
    () => { $crate::fx::FxHashSet::new_with_prof(file!(), line!()) }
}

#[macro_export]
macro_rules! fx_hash_set_with_capacity {
    ($e:expr) => { $crate::fx::FxHashSet::with_capacity_and_prof($e, file!(), line!()) }
}

#[macro_export]
macro_rules! define_id_collections {
    ($map_name:ident, $set_name:ident, $key:ty) => {
        pub type $map_name<T> = $crate::fx::FxHashMap<$key, T>;
        pub type $set_name = $crate::fx::FxHashSet<$key>;
    };
}
