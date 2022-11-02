use atty;

#[derive(Debug)]
pub struct Config {
    pub data: String,
    pub query: String,
    pub file_path: String,
    pub after_context: i32,
    pub before_context: i32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut query: String = String::from("");
        let mut file_path: String = String::from("");
        let mut after_context: i32 = 0;
        let mut before_context: i32 = 0;
        let mut data: String = String::from("");

        loop {
            if atty::is(atty::Stream::Stdin) {
                break;
            }
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
            let mut temp_arg = args.next();
            if temp_arg == None {
                break;
            }
            let arg = temp_arg.unwrap();
            if arg == "-A" {
                temp_arg = args.next();
                if temp_arg == None {
                    eprintln!("There is no argument given after -A call, please enter a number!");
                    std::process::exit(1);
                }
                after_context = (temp_arg.unwrap()).parse().unwrap_or_else(|err| {
                    eprintln!(
                        "Error parsing after_context number, please provide valid number: {}",
                        err
                    );
                    std::process::exit(1);
                });
            } else if arg == "-B" {
                temp_arg = args.next();
                if temp_arg == None {
                    eprintln!("There is no argument given after -B call, please enter a number!");
                    std::process::exit(1);
                }
                before_context = (temp_arg.unwrap()).parse().unwrap_or_else(|err| {
                    eprintln!(
                        "Error parsing before_context number, please provide valid number: {}",
                        err
                    );
                    std::process::exit(1);
                });
            } else {
                if query == "" {
                    query = arg;
                } else if file_path == "" {
                    file_path = arg;
                } else {
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
