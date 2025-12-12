mod article;
use article::api::router as article_router;

use axum::Router;
use std::net::SocketAddr;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use mms_tui::MMSTuiApplication;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

#[derive(Parser)]
#[command(version, about, long_about = None, bin_name = "mms")]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn verbose logging on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    UI {},
    Serve {
        #[clap(short, long, default_value = "3000")]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.verbose {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::UI {}) => {
            println!("start terminal ui");
            println!("{}", cli.verbose);
            let tui = MMSTuiApplication::new().expect("default mms tui application");
            println!("{:?}", tui)
        }
        Some(Commands::Serve { port }) => {
            // Eigene Tokio Runtime erzeugen
            let rt = Runtime::new().expect("Tokio runtime");

            rt.block_on(async move {
                serve(port).await;
            });
        }
        None => {}
    }
}

async fn serve(port: u16) {
    let app = Router::new().nest("/articles", article_router());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
