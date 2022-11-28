use catr::{ Config, Catr };

fn main() {
    //if let Err(e) = catr::run(Config::default()) {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);

        std::process::exit(1);
    }
}
