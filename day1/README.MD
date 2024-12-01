# Advent of Rust 2024

## Day 1

**Description**  
It's 1st December and the countdown has just begun. The elves are busy preparing for Christmas, and Santa is busy checking the list of children who have been good this year. It was supposed to be a smooth day until all of a sudden, two of Santa's elves burst into Santa's office with a problem.

> “Santa!” one of the elves shouted. “The code won’t compile! We’ve hit a wall, and it’s all Rust’s fault!”

Santa, sipping his triple-shot peppermint latte, raised an eyebrow.  
> “Rust’s fault? Or your fault?”

> “It’s the ownership rules!” the other elf blurted. “I think we violated them. We’re used to Python, where variables just... work. Look at this!”

The elves tried their best, but their code won't compile:

```rust
fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message);
 
    println!("{}", gift_message);
}
 
fn attach_message_to_present(message: String) {
    println!("The present now has this message: {}", message);
}
