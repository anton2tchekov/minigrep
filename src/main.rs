use minigrep::Config;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let request = Config::new(&args)?;

    if let Err(e) = minigrep::run(request) {
        panic!("Application error: {e}")
    };

    Ok(())
}
