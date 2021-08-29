/// Create a HashSet with the provided parameters, similar to vec!.
/// If no parameters are provided, an empty set is returned.
macro_rules! set {
    () => (
        std::collections::HashSet::new()
    );
    ( $( $x:expr ),* ) => {
        {
            let mut set = std::collections::HashSet::new();
            $(
                set.insert($x);
            )*
            set
        }
    };
}

/// Create a BTreeSet with the provided parameters, similar to vec!.
/// If no parameters are provided, an empty set is returned.
macro_rules! bt_set {
    () => (
        std::collections::BTreeSet::new()
    );
    ( $( $x:expr ),* ) => {
        {
            let mut set = std::collections::BTreeSet::new();
            $(
                set.insert($x);
            )*
            set
        }
    };
}

/// Create a HashMap with the provided parameters, similar to vec!.
/// If no parameters are provided, an empty map is returned.
/// The parameter amount must be a multiple of 2, where the first parameter
/// is the key and the second parameter is the value.
macro_rules! map {
    () => (
        std::collections::HashMap::new()
    );
    ( $( $x:expr, $y:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($x, $y);
            )*
            map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, HashMap, BTreeSet};

    /// set! with no parameters should create an empty HashSet.
    #[test]
    fn empty_set_works() {
        let set: HashSet<()> = set!();
        assert_eq!(set, HashSet::new())
    }

    /// set! with a parameter sequence should create a HashSet that is equal
    /// to a set filled with single insert-calls an the same parameters.
    #[test]
    fn set_works() {
        let macro_set = set!("foo", "bar");
        let mut manual_set = HashSet::new();
        manual_set.insert("foo");
        manual_set.insert("bar");
        assert_eq!(macro_set, manual_set)
    }

    /// bt_set! with no parameters should create an empty BTreeSet.
    #[test]
    fn empty_bt_set_works() {
        let set: BTreeSet<()> = bt_set!();
        assert_eq!(set, BTreeSet::new())
    }

    /// bt_set! with a parameter sequence should create a BTreeSet that is equal
    /// to a set filled with single insert-calls an the same parameters.
    #[test]
    fn bt_set_works() {
        let macro_set = bt_set!("foo", "bar");
        let mut manual_set = BTreeSet::new();
        manual_set.insert("foo");
        manual_set.insert("bar");
        assert_eq!(macro_set, manual_set)
    }

    /// map! with no parameters should create an empty HashMap.
    #[test]
    fn empty_map_works() {
        let map: HashMap<(), ()> = map!();
        assert_eq!(map, HashMap::new())
    }

    /// map! with a key-value-sequence as parameters should create a HashMap
    /// that is equal to a map filled with single insert-calls and the same values.
    #[test]
    fn map_works() {
        let key_one = "foo";
        let key_two = "bar";

        let macro_map = map!(key_one, 0, key_two, 0);
        let mut manual_map = HashMap::new();
        manual_map.insert(key_one, 0);
        manual_map.insert(key_two, 0);

        assert_eq!(macro_map, manual_map)
    }
}
