use clipstash::data::AppDatabase;
use clipstash::web::renderer::Renderer;
use dotenv::dotenv;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "httpd", about = "A simple HTTP daemon")]
pub struct Httpd {
    #[structopt(
        short,
        long,
        default_value = "sqlite:data.db",
        help = "The database connection string"
    )]
    pub connection_string: String,

    #[structopt(
        short,
        long,
        default_value = "templates/",
        parse(from_os_str),
        help = "The directory containing HTML templates"
    )]
    pub template_directory: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let opt = Httpd::from_args();
    let renderer = Renderer::new(opt.template_directory.clone());
    let database = AppDatabase::new(&opt.connection_string).await;

    let config = clipstash::RocketConfig { renderer, database };
    clipstash::rocket(config).launch().await?;

    Ok(())
}
