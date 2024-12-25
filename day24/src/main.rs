use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;

pub trait SleighTask {
    fn describe(&self) -> String;
}

pub struct SantaSleighQueue {
    pub records: Arc<Mutex<VecDeque<Box<dyn SleighTask + Send>>>>,
    pub done: Arc<Mutex<bool>>,
}

impl SantaSleighQueue {
    // 2. Define the `new` constructor
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(VecDeque::new())),
            done: Arc::new(Mutex::new(false)),
        }
    }

    // 3. Define the `enqueue` method
    pub fn enqueue(&self, task: Box<dyn SleighTask + Send>) {
        let mut queue = self.records.lock().unwrap();
        queue.push_back(task);
    }

    // 4. Define the `get_task` method
    pub fn get_task(&self) -> Option<Box<dyn SleighTask + Send>> {
        let mut queue = self.records.lock().unwrap();
        queue.pop_front()
    }
}

pub struct ElfTask {
    // 5. Define the fields
    name: String,
    urgency: u32,
}

impl ElfTask {
    // 6. Define the `new` constructor
    pub fn new(name: &str, urgency: u32) -> Self {
        ElfTask {
            name: name.to_string(),
            urgency,
        }
    }
}

impl SleighTask for ElfTask {
    fn describe(&self) -> String {
        format!("Elf task: {} (urgency {})", self.name, self.urgency)
    }
}

pub struct ReindeerTask {
    // 7. Define the fields
    name: String,
    weight: u32,
}

impl ReindeerTask {
    // 8. Define the `new` constructor
    pub fn new(name: &str, weight: u32) -> Self {
        ReindeerTask {
            name: name.to_string(),
            weight,
        }
    }
}

impl SleighTask for ReindeerTask {
    fn describe(&self) -> String {
        format!("Reindeer task: {} ({} kg)", self.name, self.weight)
    }
}

pub fn main() {
    let queue = Arc::new(SantaSleighQueue::new());

    let producer_queue = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        producer_queue.enqueue(Box::new(ReindeerTask::new("Deliver Toys", 100)));
        producer_queue.enqueue(Box::new(ElfTask::new("Wrap Gifts", 3)));
        producer_queue.enqueue(Box::new(ReindeerTask::new("Collect Reindeer Feed", 50)));
        producer_queue.enqueue(Box::new(ElfTask::new("Decorate Tree", 7)));
    });

    thread::sleep(std::time::Duration::from_millis(10));

    let consumer_queue = Arc::clone(&queue);
    let consumer = thread::spawn(move || loop {
        if let Some(task) = consumer_queue.get_task() {
            println!("{}", task.describe());
        } else {
            break;
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}