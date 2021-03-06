use std::time::Duration;

use crate::{
    pairs::{PairAuthority, XPredictKeystore, XPredictPairs},
    pallets::ProposalStatus,
};
use logic::XPredictLogic;
use runtime::XPredictRuntime;
use sp_core::Pair;
use sp_runtime::traits::IdentifyAccount;
use subxt::ClientBuilder;

mod logic;
mod pairs;
mod pallets;
mod runtime;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::<XPredictRuntime>::new()
        .set_url("ws://127.0.0.1:9944")
        .build()
        .await?;

    let keystore = XPredictKeystore::new();
    let admin_signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_default_signer(
        &keystore,
        PairAuthority::Admin,
    )
    .unwrap();

    let max_currency_id = XPredictLogic::number_of_currency(&client).await?;

    let (currency_id, decimals) = if max_currency_id <= 2 {
        let mut currency_id = max_currency_id;
        let decimals: u8 = 8;
        while currency_id <= 2 {
            currency_id = XPredictLogic::new_asset(&client, &admin_signer, decimals).await?;
        }
        (currency_id, decimals)
    } else {
        let currency_id = 2;
        let token_info = XPredictLogic::currencies(&client, currency_id).await?;
        (currency_id, token_info.decimals)
    };

    println!(
        "use currency: {:?}, decimals: {:?} to make proposal",
        currency_id, decimals
    );
    let bob_number = 100 * 10u128.pow(decimals as u32);
    let charlie_number = 3125 * 10u128.pow((decimals - 2) as u32);
    let number = bob_number + charlie_number;

    let balance = XPredictLogic::balance_of(
        &client,
        currency_id,
        &admin_signer.signer().public().into_account().into(),
    )
    .await?;

    println!("waiting for trasfer tokens...");
    if balance < number {
        XPredictLogic::mint_token(
            &client,
            &admin_signer,
            currency_id,
            &admin_signer.signer().public().into_account().into(),
            number - balance,
        )
        .await?;
    }

    let mut pairs_iter = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_pair_iter(
        &keystore,
        PairAuthority::Normal,
    )
    .unwrap();
    let bob = pairs_iter.next().unwrap();
    let charlie = pairs_iter.next().unwrap();
    let dave = pairs_iter.next().unwrap();
    let eve = pairs_iter.next().unwrap();
    let ferdie = pairs_iter.next().unwrap();

    for id in [0, currency_id] {
        for pair in [bob, charlie, dave, eve, ferdie] {
            let balance = XPredictLogic::balance_of(&client, id, &pair.public().into()).await?;
            let num = if id == 0 {
                if balance > 10u128.pow(12) {
                    continue;
                }
                10 * 10u128.pow(12)
            } else if pair.public() == bob.public() {
                if balance > bob_number {
                    continue;
                }
                bob_number
            } else if pair.public() == charlie.public() {
                if balance > charlie_number {
                    continue;
                }
                charlie_number
            } else {
                continue;
            };
            XPredictLogic::transfer_token(&client, &admin_signer, id, &pair.public().into(), num)
                .await?;
        }
    }

    println!("create an account for uploading results...");
    let autonomy_minimal_stake_number =
        XPredictLogic::autonomy_minimal_stake_number(&client).await?;
    for account in [dave, eve, ferdie] {
        let balance = XPredictLogic::balance_of(&client, 1, &account.public().into()).await?;
        if balance < autonomy_minimal_stake_number {
            XPredictLogic::transfer_token(
                &client,
                &admin_signer,
                1,
                &account.public().into(),
                autonomy_minimal_stake_number - balance,
            )
            .await?;
        }
        let _signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_signer(account);
        let staked_number =
            XPredictLogic::autonomy_stake_account(&client, &account.public().into()).await?;
        if staked_number != autonomy_minimal_stake_number {
            XPredictLogic::autonomy_stake(&client, &_signer).await?;
        }

        if XPredictLogic::autonomy_account(&client, &account.public().into())
            .await
            .is_err()
        {
            XPredictLogic::autonomy_tag(&client, &admin_signer, &account.public().into()).await?;
        }
    }

    let bob_signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_signer(bob);
    let proposal_id =
        XPredictLogic::make_proposal(&client, &bob_signer, currency_id, bob_number).await?;
    println!("make proposal with id : {:?}", proposal_id);

    XPredictLogic::quick_to_formal(&client, &admin_signer, proposal_id).await?;

    let charlie_signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_signer(bob);
    let (yes, no) = XPredictLogic::proposal_pairs(&client, proposal_id).await?;
    let actual_number =
        XPredictLogic::proposal_buy(&client, &charlie_signer, proposal_id, yes, charlie_number)
            .await?;
    println!(
        "actual: {:?}, fee: {:?}",
        actual_number,
        charlie_number - actual_number
    );

    let balance = XPredictLogic::balance_of(&client, yes, &bob.public().into()).await?;

    println!("bob yes balance: {:?}", balance);

    let (yes_ops, no_ops) = XPredictLogic::proposal_optional(&client, proposal_id).await?;
    println!("yes: {:?}, no: {:?}", yes_ops, no_ops);

    println!("waiting for proposal status...");
    let close_time = XPredictLogic::proposal_close_time(&client, proposal_id).await?;
    loop {
        let state = XPredictLogic::proposal_status(&client, proposal_id).await?;
        println!(
            "current proposal status: {:?}, current time: {}, close time: {}",
            state,
            XPredictLogic::now_format()?,
            XPredictLogic::ts_format(close_time)?
        );
        if state == ProposalStatus::WaitingForResults {
            break;
        }
        async_std::task::sleep(Duration::from_secs(10)).await;
    }

    println!("waiting for upload result...");
    let dave_signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_signer(dave);
    XPredictLogic::autonomy_upload(&client, &dave_signer, proposal_id, yes).await?;

    let eve_signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_signer(eve);
    XPredictLogic::autonomy_upload(&client, &eve_signer, proposal_id, yes).await?;

    let ferdie_signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_signer(ferdie);
    XPredictLogic::autonomy_upload(&client, &ferdie_signer, proposal_id, no).await?;

    Ok(())
}
