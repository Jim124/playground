#[cfg(test)]

mod test {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn tests_hashmap() {
        let mut results: HashMap<&str, u32> = HashMap::new();
        let person_1 = "alice";
        let person_2 = "bob";
        results.insert(&person_1, 55);
        results.insert(&person_2, 66);
        if results.contains_key(person_1) {
            println!("{}", results.get(person_1).unwrap());
        }
        for (k, v) in results {
            println!("key is {}, value is {}", k, v);
        }
    }

    #[test]
    fn tests_hs() {
        let mut names: HashSet<&str> = HashSet::new();
        names.insert("alice");
        names.insert("bob");
        names.insert("alice");
        println!("{:?}", names);
        if names.contains("alice") {
            dbg!("right");
        }
        for name in names {
            println!("{}", name);
        }
    }
}
