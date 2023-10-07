use clap::{Arg, Command};

/// binary file name(ckb)
pub const BIN_NAME: &str = "ckb";

/// Subcommand `run`.
pub const CMD_RUN: &str = "run";
/// Subcommand `miner`.
pub const CMD_MINER: &str = "miner";
/// Subcommand `export`.
pub const CMD_EXPORT: &str = "export";
/// Subcommand `import`.
pub const CMD_IMPORT: &str = "import";
/// Subcommand `init`.
pub const CMD_INIT: &str = "init";
/// Subcommand `replay`.
pub const CMD_REPLAY: &str = "replay";
/// Subcommand `stats`.
pub const CMD_STATS: &str = "stats";
/// Subcommand `list-hashes`.
pub const CMD_LIST_HASHES: &str = "list-hashes";
/// Subcommand `reset-data`.
pub const CMD_RESET_DATA: &str = "reset-data";
/// Subcommand `peer-id`.
pub const CMD_PEERID: &str = "peer-id";
/// Subcommand `gen`.
pub const CMD_GEN_SECRET: &str = "gen";
/// Subcommand `from-secret`.
pub const CMD_FROM_SECRET: &str = "from-secret";
/// Subcommand `migrate`.
pub const CMD_MIGRATE: &str = "migrate";

/// Command line argument `--config-dir`.
pub const ARG_CONFIG_DIR: &str = "config-dir";
/// Command line argument `--format`.
pub const ARG_FORMAT: &str = "format";
/// Command line argument `--target`.
pub const ARG_TARGET: &str = "target";
/// Command line argument `--source`.
pub const ARG_SOURCE: &str = "source";
/// Command line argument `--data`.
pub const ARG_DATA: &str = "data";
/// Command line argument `--list-chains`.
pub const ARG_LIST_CHAINS: &str = "list-chains";
/// Command line argument `--interactive`.
pub const ARG_INTERACTIVE: &str = "interactive";
/// Command line argument `--chain`.
pub const ARG_CHAIN: &str = "chain";
/// Command line argument `--import-spec`.
pub const ARG_IMPORT_SPEC: &str = "import-spec";
/// The argument for the genesis message.
pub const ARG_GENESIS_MESSAGE: &str = "genesis-message";
/// Command line argument `--p2p-port`.
pub const ARG_P2P_PORT: &str = "p2p-port";
/// Command line argument `--rpc-port`.
pub const ARG_RPC_PORT: &str = "rpc-port";
/// Command line argument `--force`.
pub const ARG_FORCE: &str = "force";
/// Command line argument `--log-to`.
pub const ARG_LOG_TO: &str = "log-to";
/// Command line argument `--bundled`.
pub const ARG_BUNDLED: &str = "bundled";
/// Command line argument `--ba-code-hash`.
pub const ARG_BA_CODE_HASH: &str = "ba-code-hash";
/// Command line argument `--ba-arg`.
pub const ARG_BA_ARG: &str = "ba-arg";
/// Command line argument `--ba-hash-type`.
pub const ARG_BA_HASH_TYPE: &str = "ba-hash-type";
/// Command line argument `--ba-message`.
pub const ARG_BA_MESSAGE: &str = "ba-message";
/// Command line argument `--ba-advanced`.
pub const ARG_BA_ADVANCED: &str = "ba-advanced";
/// Command line argument `--indexer`.
pub const ARG_INDEXER: &str = "indexer";
/// Command line argument `--from`.
pub const ARG_FROM: &str = "from";
/// Command line argument `--to`.
pub const ARG_TO: &str = "to";
/// Command line argument `--all`.
pub const ARG_ALL: &str = "all";
/// Command line argument `--limit`.
pub const ARG_LIMIT: &str = "limit";
/// Command line argument `--database`.
pub const ARG_DATABASE: &str = "database";
/// Command line argument `--network`.
pub const ARG_NETWORK: &str = "network";
/// Command line argument `--network-peer-store`.
pub const ARG_NETWORK_PEER_STORE: &str = "network-peer-store";
/// Command line argument `--network-secret-key`.
pub const ARG_NETWORK_SECRET_KEY: &str = "network-secret-key";
/// Command line argument `--logs`.
pub const ARG_LOGS: &str = "logs";
/// Command line argument `--tmp-target`.
pub const ARG_TMP_TARGET: &str = "tmp-target";
/// Command line argument `--secret-path`.
pub const ARG_SECRET_PATH: &str = "secret-path";
/// Command line argument `--profile`.
pub const ARG_PROFILE: &str = "profile";
/// Command line argument `--sanity-check`.
pub const ARG_SANITY_CHECK: &str = "sanity-check";
/// Command line argument `--full-verification`.
pub const ARG_FULL_VERIFICATION: &str = "full-verification";
/// Command line argument `--skip-spec-check`.
pub const ARG_SKIP_CHAIN_SPEC_CHECK: &str = "skip-spec-check";
/// Present `overwrite-spec` arg to force overriding the chain spec in the database with the present configured chain spec
pub const ARG_OVERWRITE_CHAIN_SPEC: &str = "overwrite-spec";
/// Command line argument `--assume-valid-target`.
pub const ARG_ASSUME_VALID_TARGET: &str = "assume-valid-target";
/// Command line argument `--check`.
pub const ARG_MIGRATE_CHECK: &str = "check";

fn main() {
    run();
}

fn run() -> Command {
    Command::new(CMD_RUN)
    .about("Runs ckb node")
    .arg(
        Arg::new(ARG_BA_ADVANCED)
            .long(ARG_BA_ADVANCED)
            .action(clap::ArgAction::SetTrue)
            .help("Allows any block assembler code hash and args"),
    )
    .arg(
        Arg::new(ARG_SKIP_CHAIN_SPEC_CHECK)
            .long(ARG_SKIP_CHAIN_SPEC_CHECK)
            .action(clap::ArgAction::SetTrue)
            .help("Skips checking the chain spec with the hash stored in the database"),
    ).arg(
        Arg::new(ARG_OVERWRITE_CHAIN_SPEC)
            .long(ARG_OVERWRITE_CHAIN_SPEC)
            .action(clap::ArgAction::SetTrue)
            .help("Overwrites the chain spec in the database with the present configured chain spec")
    ).arg(
    Arg::new(ARG_ASSUME_VALID_TARGET)
        .long(ARG_ASSUME_VALID_TARGET)
        .action(clap::ArgAction::Set)
        .help("This parameter specifies the hash of a block. \
        When the height does not reach this block's height, the execution of the script will be disabled, \
        that is, skip verifying the script content. \
        \
        It should be noted that when this option is enabled, the header is first synchronized to \
        the highest currently found. During this period, if the assume valid target is found, \
        the download of the block starts; If the assume valid target is not found or it's \
        timestamp within 24 hours of the current time, the target will automatically become invalid, \
        and the download of the block will be started with verify")
    ).arg(
        Arg::new(ARG_INDEXER)
        .long(ARG_INDEXER)
        .action(clap::ArgAction::SetTrue)
        .help("Start the built-in indexer service"),
    )
}
