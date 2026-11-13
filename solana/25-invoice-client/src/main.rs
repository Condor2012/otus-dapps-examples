use std::{str::FromStr, thread::sleep, time::Duration};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::{AccountMeta, Instruction}, message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program, sysvar, transaction::Transaction};
use uuid::Uuid;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Invoice {
    id: u128,
    amount: u64,
    paid: bool,
    destination: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionData {
    PayInvoice,
    CreateInvoice(Invoice)
}

const PROGRAM_ID: &str = "GfoQ4ENQYA6ow88hZYN1QBqQtwijt4rsN2sugzoUbmis";

fn main() -> anyhow::Result<()> {
    let client = RpcClient::new("http://localhost:8899");

    let program_id = Pubkey::from_str(PROGRAM_ID)?;

    let payer_kp = Keypair::new();
    let payer_pk = payer_kp.pubkey();
    let admin_kp = {
        let key = std::env::var("ADMIN_KEYPAIR")
            .expect("ADMIN_KEYPAIR env var is not set (comma-separated keypair bytes)");
        let bytes: Vec<u8> = key
            .split(',')
            .map(|b| b.trim().parse::<u8>().expect("invalid keypair byte"))
            .collect();
        Keypair::from_bytes(&bytes)?
    };
    let admin_pk = admin_kp.pubkey();
    let destination = Pubkey::new_unique();

    {
        let amount = 1 * LAMPORTS_PER_SOL;
        let _ = client.request_airdrop(&payer_pk, amount)?;
        loop {
            let balance = client.get_balance(&payer_pk)?;
            if balance == amount {
                break;
            }
            println!("payer balance - {}, waiting for airdrop", balance);
            sleep(Duration::from_secs(1));
        }
    }

    let id = Uuid::new_v4();

    let invoice = Invoice {
        id: id.as_u128(),
        amount: LAMPORTS_PER_SOL / 2,
        paid: false,
        destination: destination.to_bytes(),
    };

    let (pda, _) = Pubkey::find_program_address(&[id.as_bytes().as_slice()], &program_id);

    {
        let data = borsh::to_vec(&InstructionData::CreateInvoice(invoice))?;
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(admin_pk, true),
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
            ],
            data: data,
        };
        let msg = Message::new(&[instruction], Some(&admin_pk));

        let blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new(&[admin_kp], msg, blockhash);

        let sig = client.send_and_confirm_transaction(&tx)?;
        println!("signature - {}", sig);
    }

    {
        let data = borsh::to_vec(&InstructionData::PayInvoice)?;
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer_pk, true),
                AccountMeta::new(pda, false),
                AccountMeta::new(destination, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: data,
        };
        let msg = Message::new(&[instruction], Some(&payer_pk));

        let blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new(&[payer_kp], msg, blockhash);

        let sig = client.send_and_confirm_transaction(&tx)?;
        println!("signature - {}", sig);
    }

    {
        let data = client.get_account_data(&pda)?;
        let invoice = Invoice::try_from_slice(&data)?;
        println!("invoice status - {}", invoice.paid);
        println!("destination balance - {}", client.get_balance(&destination)?);
    }

    Ok(())
}
