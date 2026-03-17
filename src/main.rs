mod cdp;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

const DEFAULT_CDP_PORT: u16 = 9222;

#[derive(Parser)]
#[command(name = "browser-cli")]
#[command(about = "Browser automation CLI using Chrome DevTools Protocol")]
struct Cli {
    /// CDP port to connect to
    #[arg(long, default_value_t = DEFAULT_CDP_PORT)]
    port: u16,

    /// Output as JSON
    #[arg(long)]
    json: bool,

    /// Execute command on a specific tab id (see `tabs list`)
    #[arg(short = 't', long)]
    tab_id: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Evaluate JavaScript
    Eval { script: String },
    /// Get page information
    Get {
        #[command(subcommand)]
        what: GetCommand,
    },
    /// Manage tabs
    Tabs {
        #[command(subcommand)]
        action: TabsCommand,
    },
}

#[derive(Subcommand)]
pub enum GetCommand {
    /// Get page title
    Title,
    /// Get current URL
    Url,
    /// Get element text
    Text { selector: Option<String> },
    /// Get element HTML
    Html { selector: String },
    /// Get input value
    Value { selector: String },
    /// Get element attribute
    Attr { selector: String, name: String },
    /// Count matching elements
    Count { selector: String },
}

#[derive(Subcommand)]
pub enum TabsCommand {
    /// List open tabs
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let port = cli.port;
    let json = cli.json;
    let tab_id = cli.tab_id.as_deref();

    match cli.command {
        Command::Eval { script } => commands::cmd_eval(port, &script, json, tab_id).await,
        Command::Get { what } => commands::cmd_get(port, &what, json, tab_id).await,
        Command::Tabs { action } => commands::cmd_tabs(port, &action, json).await,
    }
}
