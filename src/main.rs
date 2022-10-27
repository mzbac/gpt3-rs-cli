#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = correct::run(correct::get_args()).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}
