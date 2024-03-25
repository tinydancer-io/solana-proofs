use alloc::rc::Rc;
use std::str::FromStr;

use account_proof_geyser::types::Update;
use account_proof_geyser::utils::verify_leaves_against_bankhash;
use borsh::de::BorshDeserialize;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use anchor_client::{Client, Cluster};
use anchor_lang::solana_program::sysvar::clock::Clock;
// use anchor_lang::AccountDeserialize;
use clap::Parser;
use clap::Subcommand;
use copy::{
    account_hasher, accounts as copy_accounts, instruction as copy_instruction, CopyAccount, PREFIX,
};
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::system_program;
use solana_sdk::sysvar::SysvarId;

extern crate alloc;

// const DEFAULT_RPC_URL: &str = "http://api.testnet.solana.com:8899";
// const DEFAULT_WS_URL: &str = "ws://api.testnet.solana.com:8900";
// const LOCAL: &str = "http://localhost:8899";

const DEFAULT_RPC_URL: &str = "http://localhost:8899";
const DEFAULT_WS_URL: &str = "ws://localhost:8900";
pub struct CopyClient {
    pub rpc_url: String,
    pub ws_url: String,
    pub signer: Keypair,
    pub copy_program: Pubkey,
    pub copy_pda: (Pubkey, u8),
    pub clock_account: Pubkey,
    pub system_program: Pubkey,
}

impl CopyClient {
    pub fn new(rpc_url: String, ws_url: String, signer: Keypair, copy_program: &str) -> Self {
        let copy_program_pubkey = Pubkey::from_str(copy_program).unwrap();
        let (copy_pda, bump) =
            Pubkey::find_program_address(&[PREFIX.as_bytes()], &copy_program_pubkey);

        CopyClient {
            rpc_url,
            ws_url,
            signer,
            copy_program: Pubkey::from_str(copy_program).unwrap(),
            copy_pda: (copy_pda, bump),
            clock_account: Clock::id(),
            system_program: system_program::id(),
        }
    }

    pub fn send_transaction(&self, source_account: &Pubkey) -> anyhow::Result<Signature> {
        let creator_pubkey = self.signer.pubkey();
        let c = Client::new(
            Cluster::Custom(self.rpc_url.clone(), self.ws_url.clone()),
            Rc::new(self.signer.insecure_clone()),
        );
        let prog = c.program(self.copy_program).unwrap();

        let signature = prog
            .request()
            .accounts(copy_accounts::CopyHash {
                creator: creator_pubkey,
                source_account: *source_account,
                copy_account: self.copy_pda.0,
                clock: self.clock_account,
                system_program: self.system_program,
            })
            .args(copy_instruction::CopyHash {
                bump: self.copy_pda.1,
            })
            .options(CommitmentConfig {
                commitment: CommitmentLevel::Processed,
            })
            .send()?;
        Ok(signature)
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CopyTransaction {
        copy_program: String,
        account_for_proof: String,
        #[arg(long, required = true)]
        /// Path to the signer key
        signer: String,
        #[arg(short, long, default_value_t=DEFAULT_RPC_URL.to_string())]
        /// URL for solana RPC
        rpc_url: String,

        #[arg(short, long, default_value_t=DEFAULT_WS_URL.to_string())]
        /// URL for solana Websocket
        ws_url: String,
    },
    CopyPda {
        copy_program: String,
    },
}

fn query_account(addr: &Pubkey) -> Account {
    let url = DEFAULT_RPC_URL.to_string();
    let client = RpcClient::new(url);
    client.get_account(addr).unwrap()
}

async fn monitor_and_verify_updates(
    rpc_pubkey: &Pubkey,
    rpc_account: &Account,
) -> anyhow::Result<()> {
    println!("starting monitor");
    let mut stream = TcpStream::connect("127.0.0.1:5000")
        .await
        .expect("unable to connect to 127.0.0.1 on port 5000");
    println!("got stream");
    let mut buffer = vec![0u8; 65536];
    let mut n = stream
        .read(&mut buffer)
        .await
        .expect("unable to read to mutable buffer");
    println!("got object");
    loop {
        if n == 0 {
            n = stream
                .read(&mut buffer)
                .await
                .expect("unable to read to mutable buffer");
            println!("got object {:?}", n);
            // anyhow::bail!("Connection closed");
        } else {
            break;
        }
    }
    let received_update: Update = BorshDeserialize::try_from_slice(&buffer[..n]).unwrap();

    let bankhash = received_update.root;
    let bankhash_proof = received_update.proof;
    let slot_num = received_update.slot;
    for p in bankhash_proof.proofs {
        verify_leaves_against_bankhash(
            &p,
            bankhash,
            bankhash_proof.num_sigs,
            bankhash_proof.account_delta_root,
            bankhash_proof.parent_bankhash,
            bankhash_proof.blockhash,
        )
        .unwrap();

        println!(
            "\nBankHash proof verification succeeded for account with Pubkey: {:?} in slot {}",
            &p.0, slot_num
        );
        let copy_account: CopyAccount =
            anchor_lang::AccountDeserialize::try_deserialize(&mut p.1 .0.account.data.as_slice())?;
        let rpc_account_hash = account_hasher(
            &rpc_pubkey,
            rpc_account.lamports,
            &rpc_account.data,
            &rpc_account.owner,
            rpc_account.rent_epoch,
        );
        assert_eq!(rpc_account_hash.as_ref(), &copy_account.digest);
        println!(
            "Hash for rpc account matches Hash verified as part of the BankHash: {}",
            rpc_account_hash
        );
        println!("{:?}", &rpc_account);
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CopyTransaction {
            copy_program,
            account_for_proof,
            signer,
            rpc_url,
            ws_url,
        } => {
            let copy_program_pubkey = Pubkey::from_str(copy_program).unwrap();
            let account_for_proof =
                Pubkey::find_program_address(&[b"copy_hash"], &copy_program_pubkey).0;
            println!("CopyPda: {:?}", account_for_proof.to_string());
            let signer_keypair = read_keypair_file(signer).unwrap();
            let account_state_from_rpc = query_account(&account_for_proof);

            let monitor_handle = std::thread::spawn(move || {
                let rt = Runtime::new().unwrap(); // Create a new Tokio runtime
                rt.block_on(monitor_and_verify_updates(
                    &account_for_proof,
                    &account_state_from_rpc,
                ))
                .unwrap(); // Run the async function `monitor_updates` to completion
            });

            let copy_client = CopyClient::new(
                rpc_url.to_string(),
                ws_url.to_string(),
                signer_keypair,
                copy_program,
            );

            copy_client.send_transaction(&account_for_proof).unwrap();
            println!("sent txn");
            monitor_handle.join().unwrap();
        }
        Commands::CopyPda { copy_program } => {
            let copy_program_pubkey = Pubkey::from_str(copy_program).unwrap();
            let (copy_pda, _) =
                Pubkey::find_program_address(&[PREFIX.as_bytes()], &copy_program_pubkey);
            println!("account: {}", copy_pda);
        }
    }
}
