fn main() {
    if let Err(err) = readmd::cli::run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
