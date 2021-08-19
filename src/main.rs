use crate::pairs::{PairAuthority, XPredictKeystore, XPredictPairs};
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

    let (currency_id, decimals) = if max_currency_id == 1 {
        let currency_id = XPredictLogic::new_asset(&client, &admin_signer, 8).await?;
        (currency_id, 8)
    } else {
        let token_info = XPredictLogic::currencies(&client, 1).await?;
        (1, token_info.decimals)
    };

    let bob_number = 100 * 10u128.pow(decimals as u32);
    let charlie_number = 255 * 10u128.pow((decimals - 1) as u32);
    let number = bob_number + charlie_number;

    let balance = XPredictLogic::balance_of(
        &client,
        currency_id,
        &admin_signer.signer().public().into_account().into(),
    )
    .await?;

    if balance < number {
        XPredictLogic::mint_token(
            &client,
            &admin_signer,
            currency_id,
            admin_signer.signer().public().into_account().into(),
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

    for id in [0, currency_id] {
        for public in [bob.public(), charlie.public()] {
            let balance = XPredictLogic::balance_of(&client, id, &public.into()).await?;
            let num = if id == 0 {
                if balance > 10u128.pow(12) {
                    continue;
                }
                10 * 10u128.pow(12)
            } else if public == bob.public() {
                if balance > bob_number {
                    continue;
                }
                bob_number
            } else if public == charlie.public() {
                if balance > charlie_number {
                    continue;
                }
                charlie_number
            } else {
                continue;
            };
            XPredictLogic::transfer_token(&client, &admin_signer, id, public.into(), num).await?;
        }
    }

    let bob_signer = <XPredictKeystore as XPredictPairs<XPredictRuntime>>::get_signer(bob);
    let proposal_id =
        XPredictLogic::make_proposal(&client, &bob_signer, currency_id, bob_number).await?;
    println!("make proposal with id : {:?}", proposal_id);

    XPredictLogic::quick_to_formal(&client, &admin_signer, proposal_id).await?;
    Ok(())
}
