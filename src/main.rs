use log::{error, info, warn};
use log4rs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    error!("Log as error");
    warn!("Log as warn");
    info!("Log as info");
}
