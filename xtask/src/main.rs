mod error_idl;
mod event_idl;
mod idl_upgrade;
mod instruction_idl;
mod pda_idl;
mod type_resolution;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has no parent directory")
        .to_path_buf()
}

// todo change
fn shank_bin() -> PathBuf {
    Path::new("/Users/user/Work/shank").join("target/debug/shank")
}

fn execute_shank(crate_root: &Path, out_dir: &Path, program_id: &str) {
    let status = Command::new(shank_bin())
        .args([
            "idl",
            "--crate-root",
            crate_root.to_str().unwrap(),
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--program-id",
            program_id,
        ])
        .status()
        .expect("failed to run shank");

    assert!(
        status.success(),
        "shank idl failed for {}",
        crate_root.display()
    );
}

/// IDL generation pipeline for drv-smart-contract-common.
///
/// By default (no step flags) only shank is run.
/// Pass one or more step flags to run those steps.
/// Pass all flags to run the full pipeline.
#[derive(Parser)]
#[command(name = "xtask", about = "IDL generation pipeline")]
struct Args {
    /// On-chain program ID to embed in the generated IDL.
    #[arg(long, required = true)]
    program_id: String,

    /// Run shank to generate the raw IDL from Rust source (default when no step flags are given).
    #[arg(long)]
    shank: bool,

    /// Merge instruction accounts and args from source into the IDL.
    #[arg(long)]
    merge_instructions: bool,

    /// Generate and inject error codes into the IDL.
    #[arg(long)]
    inject_errors: bool,

    /// Inject events into the IDL.
    #[arg(long)]
    inject_events: bool,

    /// Inject PDA seeds into the IDL.
    #[arg(long)]
    inject_pdas: bool,

    /// Inline primitive newtypes, removing defined-type wrappers.
    #[arg(long)]
    resolve_types: bool,

    /// Upgrade IDL to Anchor v1 format (spec, writable/signer rename, publicKey→pubkey).
    #[arg(long)]
    upgrade: bool,

    /// Directory where the IDL JSON file is written.
    /// Defaults to <workspace-root>/crates/models/idl
    #[arg(long)]
    out_dir: Option<PathBuf>,
}

impl Args {
    /// Returns true if any step flag was explicitly passed.
    fn any_step_flag(&self) -> bool {
        self.shank
            || self.merge_instructions
            || self.inject_errors
            || self.inject_events
            || self.inject_pdas
            || self.resolve_types
            || self.upgrade
    }
}

fn main() {
    let args = Args::parse();

    let root = workspace_root();
    let models_crate = root.join("crates/models");
    let idl_dir = args.out_dir.clone().unwrap_or_else(|| root.join("idl"));
    let idl_path = idl_dir.join("drv_models.json");

    std::fs::create_dir_all(&idl_dir).expect("failed to create IDL output directory");

    // If no step flags given, run shank by default.
    let run_shank = args.shank || !args.any_step_flag();

    if run_shank {
        println!("Generating IDL for models...");
        execute_shank(&models_crate, &idl_dir, &args.program_id);
    }
    if args.merge_instructions {
        println!("Merging instructions into IDL...");
        instruction_idl::merge_instructions_into_idl(&root, &idl_path);
    }
    if args.inject_errors {
        println!("Injecting error codes into IDL...");
        let errors = error_idl::generate_error_idl(&root);
        error_idl::inject_errors_into_idl(&root, errors, &idl_path);
    }
    if args.inject_events {
        println!("Injecting events into IDL...");
        event_idl::inject_events_into_idl(&root, &idl_path);
    }
    if args.inject_pdas {
        println!("Injecting PDAs into IDL...");
        pda_idl::inject_pdas_into_idl(&root, &idl_path);
    }
    if args.resolve_types {
        println!("Resolving defined types to primitives...");
        type_resolution::resolve_types(&root, &idl_path);
    }
    if args.upgrade {
        println!("Upgrading IDL to Anchor v1 format...");
        idl_upgrade::upgrade_to_v1(&root, &idl_path);
    }
}
