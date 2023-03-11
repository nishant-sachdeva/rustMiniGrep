fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = match minigrep::Config::build(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };
    println!("{}", config.query);
    println!("{}", config.file_name);

    match minigrep::run(config) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
