mod utils;

use crate::utils::cli::app::run;

#[tokio::main]
async fn main() {
    run().await;
}
