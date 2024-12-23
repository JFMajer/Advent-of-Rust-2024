use std::collections::HashMap;

pub struct SantaList {
    // 1. Define the records field
    records: HashMap<String, bool>,
}

impl SantaList {
    // 2. Implement the new method
    pub fn new() -> SantaList {
        Self {
            records: HashMap::new(),
        }
    }

    // 3. Implement the add method
    pub fn add(&mut self, name: &str, behavior: bool) {
        self.records.insert(name.to_string(), behavior);
    }

    // 4. Implement the remove method
    pub fn remove(&mut self, key: &str) {
        self.records.remove(key);
    }

    // 5. Implement the get method
    pub fn get(&self, name: &str) -> Option<bool> {
        match self.records.get(name) {
            Some(behaviour) => Some(*behaviour),
            None => None,
        }
    }

    // 6. Implement the count method
    pub fn count(&self) -> (i32, i32) {
        let mut nice = 0;
        let mut naughty = 0;
        for val in self.records.values() {
            if *val {
                nice += 1
            } else {
                naughty += 1
            }
        }
        (nice, naughty)
    }

    // 7. Implement the list_by_behavior method
    pub fn list_by_behavior(&self, behaviour: bool) -> Vec<String> {
        let mut kids_by_behaviour: Vec<String> = Vec::new();
        for (n, b) in &self.records {
            if b == &behaviour {
                kids_by_behaviour.push(n.clone());
            }
        }
        kids_by_behaviour
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
