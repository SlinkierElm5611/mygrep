fn main() {
    let config: Config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments!: {err}");
        std::process::exit(1);
    });
    if config.data == ""{
        display(&std::fs::read_to_string(&config.file_path).unwrap_or_else(|err|{
            eprintln!("Error opening file! {}", err);
            std::process::exit(1);
        }), &config)
    }else {
        display(&config.data, &config);
    }
}

fn display(content: &str, config: &Config){
    for line in content.lines(){
        if line.contains(&config.query){
            println!("{}", line);
        }
    }
}

#[derive(Debug)]
struct Config {
    data: String,
    query: String,
    file_path: String,
    after_context: i32,
    before_context: i32,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut query: String = String::from("");
        let mut file_path: String = String::from("");
        let mut after_context: i32 = 0;
        let mut before_context: i32 = 0;
        let mut data: String = String::from("");
        loop {
            let mut input: String = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read from pipe");

            if input == "" {
                break;
            } else {
                data.push_str(&(input.clone()));
            }
        }

        loop {
            let arg = args.next().unwrap();
            if arg == "-A"{
                after_context = (args.next().unwrap()).parse().unwrap_or_else(|err| {
                    eprintln!("Error parsing after context number: {}", err);
                    std::process::exit(1);
                });
            } else {
                if query == ""{
                    query = arg.clone();
                }else if file_path == "" {
                    file_path = arg.clone();
                }else{
                    break;
                }
            }
        }

        Ok(Config {
            query: query,
            file_path: file_path,
            after_context: after_context,
            before_context: before_context,
            data: data,
        })
    }
}