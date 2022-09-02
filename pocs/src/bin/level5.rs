use std::{env, str::FromStr};

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
    let payer = keypair(0);
    let greeting_account = keypair(1);
    let data: [u8; 4] = [0; 4];

    let mut env = LocalEnvironment::builder()
        .add_program(helloworld_program, path_hello_world_binary)
        .add_account_with_lamports(payer.pubkey(), system_program::ID, sol_to_lamports(1.0))
        .add_account_with_data(greeting_account.pubkey(), helloworld_program, &data, false)
        .build();

    buyer = Pubkey::from_str("EE6qkv2tmjv5tmBsuwk2DGLrz99MEaNrp1sFA1cWjiZi").unwrap()
    masterMintKey = Pubkey::from_str("7jeCqXCecoTmPATf5mZEkgK2yFStyyGkBJotZt6wFWNz").unwrap()
    masterEditionPda = Pubkey::from_str("CPQpZ5yhnXQ4NRv4w4HeinbSdGZpqyMSsArZLyD7rgMd").unwrap()
    masterMetadataPda = Pubkey::from_str("J9MFAArJbLreJ9mbhRji6zaDQkMBkc76SFsV1wKJURT6").unwrap()
    seller = Pubkey::from_str("J61NWUXQPfmxinVXBqsbUT4q1j3d3ArXLvZD2Cj72QAc").unwrap()
    saleStateAccount = Pubkey::from_str("G8nhDmjrwBGAEcd4HtZA5F9fDbtPzuKQgRwZg5gbkZWM").unwrap()
    newEditionMetadataPda = Pubkey::from_str("8o11wgLYE713uGDgagmDnXarRWcecwDMs8TadE3tW5MA").unwrap()
    newEditionPda = Pubkey::from_str("GdThCcbG1FkosrDGeNX8mpEJ4U4rbqpsuR9XkFNq2EDG").unwrap()
    newEditionMintKey = Pubkey::from_str("5MS1q35S6EggGas4ny1NcPBERrCr86PSXfDrBKDgmdkV").unwrap()
    walletMintingState = Pubkey::from_str("BQVq2vUwZYgFnN5ywQHv6AixVSs4iGGatrFbGD2mRCQM").unwrap()
    editionMarkPda = Pubkey::from_str("5Lvg3pkgkv5N1yRzuP2xvBE1Dki3zQi5sh7fi1rRDKRp").unwrap()
    depositAccountAddress = Pubkey::from_str("CMCifpeUBJdJv6zDV99V5g7vRYSMyofc8mutPyGZaYwY").unwrap()
    buyerEditionTokenAccount = Pubkey::from_str("CeUBGiWvjP9DiJridu6XkLKj9bzSn1gRFPJjgtqMeC6e").unwrap()
    exchangeFeeRecipient = Pubkey::from_str("6482e33zrerYfhKAjPR2ncMSrH2tbTy5LDjdhB5PXzxd").unwrap()
    pdaDepositAuthority = Pubkey::from_str("DnAL2SfSaXzJSuaaMfjq9YuzGzy8RqmoxYqC7qExNhNG").unwrap()
    tokenProgram = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap()
    tokenMetadataProgram = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap()
    systemProgram = Pubkey::from_str("11111111111111111111111111111111").unwrap()
    rent = Pubkey::from_str("SysvarRent111111111111111111111111111111111").unwrap()
    instructions = Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap()

    creator1 = Pubkey::from_str("J61NWUXQPfmxinVXBqsbUT4q1j3d3ArXLvZD2Cj72QAc").unwrap()
    creator2 = Pubkey::from_str("HvwPy24x79V95y1wqdqj69vcuSKYe3TvKA1QHLWR1REX").unwrap()
    creator3 = Pubkey::from_str("3sTPDK9XdoF8uSTdzYWhrHm8Vqy5ic6sULDTYKnZFa2J").unwrap()
    creator4 = Pubkey::from_str("EZGd7N1x1dFBA3e3N3erkCYWktBRh8c23BWSHyqBnsvj").unwrap()
    env.execute_as_transaction(
        &[Instruction {
            program_id: helloworld_program,
            accounts: !vec[
                AccountMeta::new(buyer, true),  
                AccountMeta::new_readonly(masterMintKey, false),  
                AccountMeta::new(masterEditionPda, false),  
                AccountMeta::new_readonly(masterMetadataPda, false),  
                AccountMeta::new_readonly(seller, false),  
                AccountMeta::new(saleStateAccount, false),  
                AccountMeta::new(newEditionMetadataPda, false),  
                AccountMeta::new(newEditionPda, false),  
                AccountMeta::new(newEditionMintKey, false),  
                AccountMeta::new(walletMintingState, false),  
                AccountMeta::new(editionMarkPda, false),  
                AccountMeta::new(depositAccountAddress, false),  
                AccountMeta::new(buyerEditionTokenAccount, false),  
                AccountMeta::new(exchangeFeeRecipient, false),  
                AccountMeta::new_readonly(pdaDepositAuthority, false),  
                AccountMeta::new_readonly(tokenProgram, false),  
                AccountMeta::new_readonly(tokenMetadataProgram, false),  
                AccountMeta::new_readonly(systemProgram, false),  
                AccountMeta::new_readonly(rent, false),  
                AccountMeta::new_readonly(instructions, false),  

                AccountMeta::new(creator1, false),  
                AccountMeta::new(creator2, false),  
                AccountMeta::new(creator3, false),  
                AccountMeta::new(creator4, false)],
            data: vec![0xa7, 0x34, 0xe2, 0xad, 0xfd, 0xe9, 0xbf, 0x3e, 0x61, 0x5a, 0x12, 0x63, 0x80, 0x90, 0x20, 0x29, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xfd, 0x1b, 0x50, 0xb6, 0x52, 0xdc, 0x85, 0xc9, 0x47, 0xa1, 0x33, 0xee, 0x34, 0xf6, 0xec, 0x8e, 0xe2, 0x9e, 0x1e, 0xda, 0xe7, 0x1a, 0x6a, 0x05, 0x2d, 0x2d, 0xea, 0x12, 0xd8, 0xd1, 0xc1],
        }],
        &[&greeting_account],
    )
    .print();

    env
}
