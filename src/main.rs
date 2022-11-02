pub mod config;

fn display(content: &str, config: &crate::config::Config) {
    let lines: Vec<&str> = content.lines().clone().collect();
    for (index, line) in content.lines().enumerate() {
        if line.contains(&config.query) {
            if config.after_context != 0 {
                for i in 0..config.after_context {
                    if index - ((i as usize) + 1) >= 0 as usize {
                        println!("{}", &lines[(index - ((i as usize) + 1))]);
                    }
                }
            }
            println!("{}", &line);
            if config.before_context != 0 {
                for i in 0..config.before_context {
                    if index + ((i as usize) + 1) < lines.len() as usize {
                        println!("{}", &lines[(index + ((i as usize) + 1))]);
                    }
                }
            }
        }
    }
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
