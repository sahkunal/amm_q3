use {
    anchor_lang::solana_program::instruction::Instruction,
    anchor_spl::associated_token,
    litesvm::LiteSVM,
    litesvm_token::CreateMint,
    solana_transaction::Transaction,

    solana_sdk::{
        hash::Hash,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
};

mod ix_handlers;
use ix_handlers::*;

fn send_tx(
    svm: &mut LiteSVM,
    ixs: &[Instruction],
    payer: &Keypair,
    signers: &[&Keypair],
) {
    let blockhash: Hash = svm.latest_blockhash();

    let tx = Transaction::new_signed_with_payer(
            ixs,
            Some(&payer.pubkey()),
            signers,
            blockhash,
    );

    svm.send_transaction(tx).unwrap();
}

fn setup() -> (
    LiteSVM,
    Keypair,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
) {
    let program_id = amm_q3::id();

    let payer = Keypair::new();

    let mut svm = LiteSVM::new();

    let bytes = include_bytes!("../../../target/deploy/amm_q3.so");

    svm.add_program(program_id, bytes).unwrap();

    svm.airdrop(&payer.pubkey(), 10_000_000_000)
        .unwrap();

    let mint_x = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&payer.pubkey())
        .send()
        .unwrap();

    let mint_y = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&payer.pubkey())
        .send()
        .unwrap();

    let config = Pubkey::find_program_address(
        &[b"config", &123u64.to_le_bytes()],
        &amm_q3::id(),
    )
    .0;

    let mint_lp = Pubkey::find_program_address(
        &[b"lp", config.as_ref()],
        &amm_q3::id(),
    )
    .0;

    let vault_x =
        associated_token::get_associated_token_address(&config, &mint_x);

    let vault_y =
        associated_token::get_associated_token_address(&config, &mint_y);

    (
        svm,
        payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    )
}

#[test]
fn test_initialize() {
    let (
        mut svm,
        payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    ) = setup();

    let instruction = create_initialize_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    send_tx(
        &mut svm,
        &[instruction],
        &payer,
        &[&payer],
    );
}

#[test]
fn test_deposit() {
    let (
        mut svm,
        payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    ) = setup();

    let init_instruction = create_initialize_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    let deposit_instruction = create_deposit_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    send_tx(
        &mut svm,
        &[
            init_instruction,
            deposit_instruction,
        ],
        &payer,
        &[&payer],
    );
}

#[test]
fn test_withdraw() {
    let (
        mut svm,
        payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    ) = setup();

    let init_instruction = create_initialize_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    let deposit_instruction = create_deposit_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    let withdraw_instruction = create_withdraw_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    send_tx(
        &mut svm,
        &[
            init_instruction,
            deposit_instruction,
            withdraw_instruction,
        ],
        &payer,
        &[&payer],
    );
}

#[test]
fn test_swap() {
    let (
        mut svm,
        payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    ) = setup();

    let init_instruction = create_initialize_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    let deposit_instruction = create_deposit_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    let swap_instruction = create_swap_ix(
        &mut svm,
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    );

    send_tx(
        &mut svm,
        &[
            init_instruction,
            deposit_instruction,
            swap_instruction,
        ],
        &payer,
        &[&payer],
    );
}