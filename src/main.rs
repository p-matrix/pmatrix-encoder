// main.rs — P-MATRIX Reference Encoder CLI
//
// Reference encoder for schema conformance. Not an execution engine.
//
// Usage:
//   pmatrix-encoder emit --baseline 0.25 --norm 0.70 --stability 0.30 --meta-control 0.20
//   pmatrix-encoder validate < record.json
//   echo '{"spec_version":...}' | pmatrix-encoder validate

use clap::{Parser, Subcommand};
use pmatrix_encoder::{emit_demo_record, validate_record};
use pmatrix_encoder::schema::RuntimeStateRecord;
use std::io::{self, Read};

#[derive(Parser)]
#[command(
    name = "pmatrix-encoder",
    about = "P-MATRIX Runtime State Reference Encoder — schema conformance tool, not an execution engine.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Emit a demonstration runtime state record from four function values.
    Emit {
        #[arg(long)]
        baseline: f64,
        #[arg(long)]
        norm: f64,
        #[arg(long)]
        stability: f64,
        #[arg(long, name = "meta-control")]
        meta_control: f64,
        /// Optional Unix timestamp (defaults to current time).
        #[arg(long)]
        timestamp: Option<u64>,
    },
    /// Validate a runtime state record (JSON from stdin) against all 12 invariants.
    Validate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Emit {
            baseline,
            norm,
            stability,
            meta_control,
            timestamp,
        } => {
            match emit_demo_record(baseline, norm, stability, meta_control, timestamp) {
                Ok(record) => {
                    let json = serde_json::to_string_pretty(&record).unwrap();
                    println!("{}", json);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Validate => {
            let mut input = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut input) {
                eprintln!("Error reading stdin: {}", e);
                std::process::exit(1);
            }

            let record: RuntimeStateRecord = match serde_json::from_str(&input) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("JSON parse error: {}", e);
                    eprintln!("The input must be a valid P-MATRIX runtime state record.");
                    std::process::exit(1);
                }
            };

            let results = validate_record(&record);
            let mut all_passed = true;

            for r in &results {
                let status = if r.passed { "PASS" } else { "FAIL" };
                if !r.passed {
                    all_passed = false;
                }
                println!("[{}] {} — {}", status, r.id, r.detail);
            }

            println!();
            if all_passed {
                println!("Result: ALL INVARIANTS SATISFIED — record is conforming.");
            } else {
                println!("Result: INVARIANT VIOLATION(S) DETECTED — record is malformed.");
                std::process::exit(1);
            }
        }
    }
}
