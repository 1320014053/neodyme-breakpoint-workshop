use std::{env, str::FromStr};

use solana_program::rent::Rent;
use solana_program::sysvar;

use poc_framework::{
    keypair, solana_sdk::signer::Signer, Environment, LocalEnvironment, PrintableTransaction,
};
// Anchor
//use anchor_client::solana_sdk::system_instruction;
//use anchor_client::{RequestBuilder, RequestNamespace};
//use helloworld::accounts as helloworld_accounts;
//use helloworld::instruction as helloworld_instruction;

use solana_program::{native_token::sol_to_lamports, pubkey::Pubkey, system_program};

use solana_program::instruction::{AccountMeta, Instruction};

pub fn main() {
    let _env = setup();
}

fn setup() -> LocalEnvironment {
    let mut dir = env::current_exe().unwrap();
    let path_hello_world_binary = {
        dir.pop();
        dir.pop();
        dir.pop();
        dir.push("liblevel0.so");
        dir.to_str()
    }
    .unwrap();

    let helloworld_program =
        Pubkey::from_str("EXBuYPNgBUXMTsjCbezENRUtFQzjUNZxvPGTd11Pznk5").unwrap();


    let buyer = keypair(0);
    let masterMintKey = keypair(1);
    let masterEditionPda = keypair(2);
    let masterMetadataPda = keypair(3);
    let seller = keypair(4);
    let saleStateAccount = keypair(5);
    let newEditionMetadataPda = keypair(6);
    let newEditionPda = keypair(7);
    let newEditionMintKey = keypair(8);
    let walletMintingState = keypair(9);
    let editionMarkPda = keypair(10);
    let depositAccountAddress = keypair(11);
    let buyerEditionTokenAccount = keypair(12);
    let exchangeFeeRecipient = keypair(13);
    let pdaDepositAuthority = keypair(14);
    let tokenProgram = keypair(15);
    let tokenMetadataProgram = keypair(16);
    let systemProgram = keypair(17);
    let rent = keypair(18);
    let instructions = keypair(19);

    let creator1 = keypair(20);
    let creator2 = keypair(21);
    let creator3 = keypair(22);
    let creator4 = keypair(23);

    let mut env = LocalEnvironment::builder()
        .add_program(helloworld_program, path_hello_world_binary)
        .add_account_with_lamports(buyer.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(masterMintKey.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(masterEditionPda.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(masterMetadataPda.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(seller.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(saleStateAccount.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(newEditionMetadataPda.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(newEditionPda.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(newEditionMintKey.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(walletMintingState.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(editionMarkPda.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(depositAccountAddress.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(buyerEditionTokenAccount.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(exchangeFeeRecipient.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(pdaDepositAuthority.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(creator1.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(creator2.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(creator3.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .add_account_with_lamports(creator4.pubkey(), system_program::ID, sol_to_lamports(10000.0))
        .build();




    env.execute_as_transaction(
        &[Instruction {
            program_id: helloworld_program,
            accounts: vec![
                AccountMeta::new(buyer.pubkey(), true),
                AccountMeta::new_readonly(masterMintKey.pubkey(), false),
                AccountMeta::new(masterEditionPda.pubkey(), false),
                AccountMeta::new_readonly(masterMetadataPda.pubkey(), false),
                AccountMeta::new_readonly(seller.pubkey(), false),
                AccountMeta::new(saleStateAccount.pubkey(), false),
                AccountMeta::new(newEditionMetadataPda.pubkey(), false),
                AccountMeta::new(newEditionPda.pubkey(), false),
                AccountMeta::new(newEditionMintKey.pubkey(), false),
                AccountMeta::new(walletMintingState.pubkey(), false),
                AccountMeta::new(editionMarkPda.pubkey(), false),
                AccountMeta::new(depositAccountAddress.pubkey(), false),
                AccountMeta::new(buyerEditionTokenAccount.pubkey(), false),
                AccountMeta::new(exchangeFeeRecipient.pubkey(), false),
                AccountMeta::new_readonly(pdaDepositAuthority.pubkey(), false),
                AccountMeta::new_readonly(tokenProgram.pubkey(), false),
                AccountMeta::new_readonly(tokenMetadataProgram.pubkey(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(instructions.pubkey(), false),
                AccountMeta::new(creator1.pubkey(), false),  
                AccountMeta::new(creator2.pubkey(), false),  
                AccountMeta::new(creator3.pubkey(), false),  
                AccountMeta::new(creator4.pubkey(), false)],
            data: vec![0xa7, 0x34, 0xe2, 0xad, 0xfd, 0xe9, 0xbf, 0x3e, 0x61, 0x5a, 0x12, 0x63, 0x80, 0x90, 0x20, 0x29, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xfd, 0x1b, 0x50, 0xb6, 0x52, 0xdc, 0x85, 0xc9, 0x47, 0xa1, 0x33, 0xee, 0x34, 0xf6, 0xec, 0x8e, 0xe2, 0x9e, 0x1e, 0xda, 0xe7, 0x1a, 0x6a, 0x05, 0x2d, 0x2d, 0xea, 0x12, 0xd8, 0xd1, 0xc1],
        }],
        &[&buyer],
    )
    .print();

    env
}
