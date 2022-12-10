use headr::{Headr, get_args};
fn main() {
    if let Err(err) = get_args().and_then(headr::run) {
        eprintln!("{}", err.to_string());
        std::process::exit(1);
    } 
}
