pub mod config;

fn display(content: &str, config: &crate::config::Config) {
    println!("{}", content);
    dbg!(config);
}

fn main() {
    let config: crate::config::Config = crate::config::Config::build(std::env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem passing arguments!: {err}");
            std::process::exit(1);
        });
    if config.file_path == "" && config.data != "" {
        display(&config.data, &config);
    } else if config.file_path != "" && config.data == "" {
        display(
            &std::fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
                eprintln!("Error opening file! {}", err);
                std::process::exit(1);
            }),
            &config,
        );
    } else {
        eprintln!("No file path or data was given/piped in!!");
        std::process::exit(1);
    }
}
