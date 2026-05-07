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
        recipient: Option<String>,

        #[arg(long)]
        append: bool,

        #[arg(long)]
        json: bool,
    },

    /// Inspect the local ledger state.
    LedgerStatus {
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
        force: bool,

        #[arg(long)]
        json: bool,
    },

    /// Show the local wallet address.
    NewAddress {
        #[arg(long)]
        json: bool,
    },

    /// Sign a message with local wallet or explicit MVP private key hex.
    SignMessage {
        #[arg(long)]
        private_key: Option<String>,

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

    /// Create a local transfer event.
    CreateTransfer {
        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,

        #[arg(long)]
        amount: String,

        #[arg(long)]
        append: bool,

        #[arg(long)]
        json: bool,
    },

    /// Verify a local transfer event by transaction id.
    VerifyFlow {
        #[arg(long)]
        tx: String,

        #[arg(long)]
        json: bool,
    },

    /// Export a complete local proof snapshot.
    ExportProofSnapshot {
        #[arg(long)]
        output: Option<String>,

        #[arg(long)]
        json: bool,
    },

    /// Export proof JSON files for local Proof Explorer usage.
    ExportProofSiteData,

    /// List local release event proofs.
    ListReleaseEvents {
        #[arg(long)]
        json: bool,
    },

    /// List local transfer event proofs.
    ListTransferEvents {
        #[arg(long)]
        json: bool,
    },

    /// Prepare a static local Proof Explorer.
    PrepareExplorer,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::StartingState { json } => commands::starting_state::run(json),
        Commands::SimulateRelease {
            interval,
            recipient,
            append,
            json,
        } => commands::release::simulate_release(interval, recipient, append, json),
        Commands::LedgerStatus { json } => commands::ledger::ledger_status(json),
        Commands::VerifySupply { json } => commands::verify::verify_supply(json),
        Commands::VerifyRule { json } => commands::verify::verify_rule(json),
        Commands::IntegrityStatus { json } => commands::status::integrity_status(json),
        Commands::CreateWallet { force, json } => commands::wallet::create_wallet(force, json),
        Commands::NewAddress { json } => commands::wallet::new_address(json),
        Commands::SignMessage {
            private_key,
            message,
            json,
        } => commands::wallet::sign_message(private_key, message, json),
        Commands::VerifyOwnership {
            address,
            message,
            signature,
            public_key,
            json,
        } => commands::wallet::verify_ownership(address, message, signature, public_key, json),
        Commands::CreateTransfer {
            from,
            to,
            amount,
            append,
            json,
        } => commands::transfer::create_transfer(from, to, amount, append, json),
        Commands::VerifyFlow { tx, json } => commands::transfer::verify_flow_by_tx(tx, json),
        Commands::ExportProofSnapshot { output, json } => {
            commands::proof_api::export_proof_snapshot(output, json)
        }
        Commands::ExportProofSiteData => commands::proof_api::export_proof_site_data(),
        Commands::ListReleaseEvents { json } => commands::proof_api::list_release_events(json),
        Commands::ListTransferEvents { json } => commands::proof_api::list_transfer_events(json),
        Commands::PrepareExplorer => commands::proof_api::prepare_explorer(),
    }
}
