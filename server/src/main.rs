use clap::{arg, value_parser, Command};
use mariadb_kb_server::{run, Config};

pub fn main() {
    run(&read_args());
}

fn read_args() -> Config {
    let matches = Command::new("mariadb_kb_server")
        .arg(
            arg!(-p --port <PORT> "Port number for the server (default = 7032)")
                .value_parser(value_parser!(u32)),
        )
        .arg(arg!(-s --source <SRC>
                "Source Directory to read from (default = '../archive')"))
        .get_matches();

    let port = matches.get_one::<u32>("port").copied().unwrap_or(7032);

    let source = matches
        .get_one::<String>("source")
        .map_or("../archive", String::as_str)
        .into();

    Config { port, source }
}
