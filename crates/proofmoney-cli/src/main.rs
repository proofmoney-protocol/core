use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "proofmoney")]
#[command(about = "ProofMoney local MVP CLI for verifiable money integrity.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify the starting state.
    StartingState {
        #[arg(long)]
        json: bool,
    },

    /// Simulate a Proof Release Curve event.
    SimulateRelease {
        #[arg(long)]
        interval: u64,

        #[arg(long)]
        json: bool,
    },

    /// Verify current supply.
    VerifySupply {
        #[arg(long)]
        json: bool,
    },

    /// Verify active rule set.
    VerifyRule {
        #[arg(long)]
        json: bool,
    },

    /// Show overall integrity status.
    IntegrityStatus {
        #[arg(long)]
        json: bool,
    },

    /// Create a local experimental wallet.
    CreateWallet {
        #[arg(long)]
        json: bool,
    },

    /// Sign a message with an MVP private key hex.
    SignMessage {
        #[arg(long)]
        private_key: String,

        #[arg(long)]
        message: String,

        #[arg(long)]
        json: bool,
    },

    /// Verify ownership by address, message, signature, and public key.
    VerifyOwnership {
        #[arg(long)]
        address: String,

        #[arg(long)]
        message: String,

        #[arg(long)]
        signature: String,

        #[arg(long)]
        public_key: String,

        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::StartingState { json } => commands::starting_state::run(json),
        Commands::SimulateRelease { interval, json } => {
            commands::release::simulate_release(interval, json)
        }
        Commands::VerifySupply { json } => commands::verify::verify_supply(json),
        Commands::VerifyRule { json } => commands::verify::verify_rule(json),
        Commands::IntegrityStatus { json } => commands::status::integrity_status(json),
        Commands::CreateWallet { json } => commands::wallet::create_wallet(json),
        Commands::SignMessage { private_key, message, json } => {
            commands::wallet::sign_message(private_key, message, json)
        }
        Commands::VerifyOwnership { address, message, signature, public_key, json } => {
            commands::wallet::verify_ownership(address, message, signature, public_key, json)
        }
    }
}
