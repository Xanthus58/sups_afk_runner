use std::process::Command;

fn main() {
    print!("{esc}c", esc = 27 as char);

    let website = "https://xanthus58.github.io/Xanthus58/";
    let log_location =
        "'C:/Program Files (x86)/Steam/steamapps/common/GarrysMod/garrysmod/console.log'";

    println!("-Credits-");
    println!(
        "Made by Xanthus58 you can check out my other works at {}",
        website
    );
    println!("If you have any issue let me know.");
    println!("--------");
    println!("logs can be found in {}", log_location);

    let mut line = String::new();
    println!("Press enter to run the game");
    std::io::stdin().read_line(&mut line).unwrap();

    Command::new("afk_runner.bat")
        .output()
        .expect("Failed to execute command");
}
