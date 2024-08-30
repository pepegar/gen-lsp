use anyhow::Result;
use clap::Parser;
use tower_lsp::{LspService, Server};
use tokio::io::{stdin, stdout};

mod lsp_server;
use lsp_server::GenericLspServer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    tcp: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.tcp {
        println!("TCP mode is not implemented yet.");
        return Ok(());
    }

    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(|client| GenericLspServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
