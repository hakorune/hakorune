#[path = "rc_insertion_selfcheck/helpers.rs"]
mod helpers;
#[path = "rc_insertion_selfcheck/cases/mod.rs"]
mod cases;

fn main() {
    cases::main();
}
