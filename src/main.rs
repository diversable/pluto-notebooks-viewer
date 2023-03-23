fn main() {
    if let Err(err) = pluto::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
