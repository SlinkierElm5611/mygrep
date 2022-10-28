fn main() {
    let config: Config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments!: {err}");
        std::process::exit(1);
    });
    let content = std::fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Error opening file! {}", err);
        std::process::exit(1);
    });
    for line in content.lines(){
        if line.contains(&config.query){
            println!("{}", &line);
        }
    }
}
#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No Entry For Query!"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No filepath found!"),
        };
        Ok(Config {
            query: query,
            file_path: file_path,
        })
    }
}
