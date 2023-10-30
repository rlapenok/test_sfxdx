use borsh::BorshDeserialize;
use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_test::processor;
use solana_program_test::{tokio, ProgramTest, ProgramTestContext};
use solana_sdk::signature::Signer;
use solana_sdk::{signature::Keypair, transaction::Transaction};

use spl_token::*;

use crate::instructions::store_instruction::{Payload, StoreInstructions};
use crate::process_instruction;
use crate::state::store_state::{Store, SEED};
struct Context {
    ctx: ProgramTestContext,
    init: Keypair,
    user: Keypair,
    program_id: Pubkey,
}

impl Context {
    async fn new() -> Self {
        //Mock создание program_id
        let program_id = Pubkey::new_unique();
        //Создание контекста
        let test = ProgramTest::new("sfxdx", program_id, processor!(process_instruction));
        let mut context = test.start_with_context().await;
        //Создание аккаунтов для admon и user
        let admin = Keypair::new();
        let user1 = Keypair::new();
        //Создание транзакций для депа по 1 Sol на их балансы
        let instr_dep_init_acc =
            system_instruction::transfer(&context.payer.pubkey(), &admin.pubkey(), 1000000000);
        let instr_dep_user1_acc =
            system_instruction::transfer(&context.payer.pubkey(), &user1.pubkey(), 1000000000);
        //Запуск транзакций
        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[instr_dep_init_acc, instr_dep_user1_acc],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        let (pda_key, bump_seed) = Pubkey::find_program_address(&[SEED.as_bytes()], &program_id);
        //Создание payload
        let payload = Payload {
            amount: 10.0,
            bump_seed: Some(bump_seed),
        };
        //Создание инструкции
        let instr = StoreInstructions::init_store(payload, program_id, admin.pubkey(), pda_key);
        //Создание транзакции
        let tx1 = Transaction::new_signed_with_payer(
            &[instr],
            Some(&admin.pubkey()),
            &[&admin],
            context.last_blockhash,
        );
        //Запуск транзакции
        context.banks_client.process_transaction(tx1).await.unwrap();
        //Получение данных
        let result = context
            .banks_client
            .get_account(pda_key)
            .await
            .unwrap()
            .unwrap();
        let data = Store::try_from_slice(&result.data).unwrap();
        //Тесты
        assert_eq!(data.init, admin.pubkey().as_ref());
        assert_eq!(data.price, 10.0);
        Self {
            ctx: context,
            init: admin,
            user: user1,
            program_id: program_id,
        }
    }
}
#[tokio::test]
async fn init_store() {
    Context::new().await;
}

#[tokio::test]
async fn update_price() {
    let mut context = Context::new().await;
    let (pda_key, _) = Pubkey::find_program_address(&[SEED.as_bytes()], &context.program_id);
    let payload2 = Payload {
        amount: 5.0,
        bump_seed: None,
    };
    let instr2 = StoreInstructions::update_store(
        payload2,
        context.program_id,
        context.init.pubkey(),
        pda_key,
    );
    let tx2 = Transaction::new_signed_with_payer(
        &[instr2],
        Some(&context.init.pubkey()),
        &[&context.init],
        context.ctx.last_blockhash,
    );
    context
        .ctx
        .banks_client
        .process_transaction(tx2)
        .await
        .unwrap();
    let resul = context
        .ctx
        .banks_client
        .get_account(pda_key)
        .await
        .unwrap()
        .unwrap();
    let data = Store::try_from_slice(&resul.data).unwrap();

    assert_eq!(data.price, 5.0)
}

