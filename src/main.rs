use dialoguer::{theme::ColorfulTheme, Select};

use crate::collector::Collector;

mod collector;

fn main() {
    println!("WELCOME TO LEPRECHAUN");
    let selections = &[
        "Collect Data",
        "Learn",
        "BackTest",
        "Deploy",
        "Exit"
    ];
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your action: ")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

        match selection {
            0 => {Collector{};},
            1 => {},
            2 => {},
            3 => {},
            _ => break,
        }
    }
}
