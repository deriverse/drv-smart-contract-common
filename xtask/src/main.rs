mod anchor_spec;
mod error_idl;
mod event_idl;
mod idl_upgrade;
mod instruction_idl;
mod pda_idl;
mod solana_spec;
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
/// Shank always runs first to produce the base IDL.
/// Pass enhancement flags to enrich it before writing the final output.
/// Pass --solana-spec to convert the result to Solana IDL spec v0.1.0.
#[derive(Parser)]
#[command(name = "xtask", about = "IDL generation pipeline")]
struct Args {
    /// On-chain program ID to embed in the generated IDL.
    #[arg(long, required = true)]
    program_id: String,

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

    /// Convert the IDL to Solana IDL spec v0.1.0.
    ///
    /// Runs the full conversion pipeline on top of the base Shank IDL:
    ///   - inline primitive newtypes (resolve-types)
    ///   - upgrade to Anchor v1 field names (writable/signer, publicKey→pubkey)
    ///   - fix defined-type references to object form
    ///   - convert instruction discriminants to byte-array form
    ///   - restructure root address / metadata.name / metadata.version
    ///   - remove pda:false markers from instruction accounts
    ///   - convert PDA seed kind:param → kind:arg
    ///   - extract inline account type definitions into the types section
    ///   - inject account discriminators (LE u32 tag bytes)
    #[arg(long)]
    solana_spec: bool,

    /// Convert the IDL to anchor-lang-idl-spec 0.1.0 format.
    ///
    /// Applies the same pipeline as --solana-spec and additionally:
    ///   - removes Shank extension fields (top-level `pdas`, `metadata.origin`)
    ///   - omits default-false boolean fields (writable/signer/optional) from
    ///     instruction accounts per the spec's skip_serializing_if semantics
    #[arg(long)]
    anchor: bool,

    /// Directory where the IDL JSON file is written.
    /// Defaults to <workspace-root>/idl
    #[arg(long)]
    out_dir: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let root = workspace_root();
    let models_crate = root.join("crates/models");
    let idl_dir = args.out_dir.clone().unwrap_or_else(|| root.join("idl"));
    let idl_path = idl_dir.join("drv_models.json");

    std::fs::create_dir_all(&idl_dir).expect("failed to create IDL output directory");

    // Shank always runs first to produce the base IDL.
    println!("Generating IDL for models...");
    execute_shank(&models_crate, &idl_dir, &args.program_id);

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

    if args.solana_spec {
        println!("Resolving defined types to primitives...");
        type_resolution::resolve_types(&root, &idl_path);

        println!("Upgrading IDL to Anchor v1 format...");
        idl_upgrade::upgrade_to_v1(&root, &idl_path);

        println!("Aligning IDL to Solana spec v0.1.0...");
        solana_spec::align_to_solana_spec(&root, &idl_path);
    }

    if args.anchor {
        println!("Resolving defined types to primitives...");
        type_resolution::resolve_types(&root, &idl_path);

        println!("Upgrading IDL to Anchor v1 format...");
        idl_upgrade::upgrade_to_v1(&root, &idl_path);

        println!("Aligning IDL to anchor-lang-idl-spec 0.1.0...");
        anchor_spec::align_to_anchor_spec(&root, &idl_path);
    }
}
