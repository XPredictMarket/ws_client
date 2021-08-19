use std::{
	marker::PhantomData,
	time::{Duration, SystemTime},
};

use subxt::{sp_core::sr25519::Pair as Sr25519Pair, sudo::*, system::*, Client, PairSigner};

use crate::{
	pallets::{couple::*, proposals::*, tokens::*, *},
	runtime::{Moment, XPredictRuntime},
};

pub struct XPredictLogic;

impl XPredictLogic {
	pub async fn new_asset(
		client: &Client<XPredictRuntime>,
		signer: &PairSigner<XPredictRuntime, Sr25519Pair>,
		decimals: u8,
	) -> Result<CurrencyId, Box<dyn std::error::Error>> {
		let call = client
			.encode(NewAssetCall {
				name: "foo".as_bytes().to_vec(),
				symbol: "bar".as_bytes().to_vec(),
				decimals,
				_runtime: PhantomData,
			})
			.unwrap();
		let result = client.sudo_and_watch(signer, &call).await?;

		let currency_id = result.new_asset()?.unwrap().currency_id;
		Ok(currency_id)
	}

	pub async fn mint_token(
		client: &Client<XPredictRuntime>,
		signer: &PairSigner<XPredictRuntime, Sr25519Pair>,
		currency_id: CurrencyId,
		to: <XPredictRuntime as System>::AccountId,
		number: Balance,
	) -> Result<(), Box<dyn std::error::Error>> {
		let call = client
			.encode(MintCall {
				currency_id,
				to,
				number,
			})
			.unwrap();
		let result = client.sudo_and_watch(signer, &call).await?;
		if result.mint()?.is_some() {
			Ok(())
		} else {
			Err("unknown error".into())
		}
	}

	pub async fn transfer_token(
		client: &Client<XPredictRuntime>,
		signer: &PairSigner<XPredictRuntime, Sr25519Pair>,
		currency_id: CurrencyId,
		to: <XPredictRuntime as System>::AccountId,
		number: Balance,
	) -> Result<(), Box<dyn std::error::Error>> {
		let result = client
			.transfer_and_watch(signer, currency_id, to, number)
			.await?;
		if result.transfer()?.is_some() {
			Ok(())
		} else {
			Err("unknown error".into())
		}
	}

	pub async fn make_proposal(
		client: &Client<XPredictRuntime>,
		signer: &PairSigner<XPredictRuntime, Sr25519Pair>,
		currency_id: CurrencyId,
		number: Balance,
	) -> Result<ProposalId, Box<dyn std::error::Error>> {
		let close_time = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)?
			.checked_add(Duration::from_secs(15 * 60)) // 15 min
			.ok_or("unknown error")?
			.as_millis() as Moment;
		let result = client
			.new_proposal_and_watch(
				signer,
				"test".as_bytes().to_vec(),
				["a".as_bytes().to_vec(), "b".as_bytes().to_vec()],
				close_time,
				1,
				currency_id,
				number,
				200,
				"".as_bytes().to_vec(),
			)
			.await?;
		let proposal_id = result.new_proposal()?.ok_or("unknown error")?.proposal_id;
		Ok(proposal_id)
	}

	pub async fn quick_to_formal(
		client: &Client<XPredictRuntime>,
		signer: &PairSigner<XPredictRuntime, Sr25519Pair>,
		proposal_id: ProposalId,
	) -> Result<(), Box<dyn std::error::Error>> {
		let call = client
			.encode(SetStatusCall {
				proposal_id,
				new_status: ProposalStatus::FormalPrediction,
				_runtime: PhantomData,
			})
			.unwrap();
		let result = client.sudo_and_watch(signer, &call).await?;
		if result.proposal_status_changed()?.is_some() {
			Ok(())
		} else {
			Err("unknown error".into())
		}
	}

	pub async fn balance_of(
		client: &Client<XPredictRuntime>,
		currency_id: CurrencyId,
		account: &<XPredictRuntime as System>::AccountId,
	) -> Result<Balance, Box<dyn std::error::Error>> {
		let balance = if currency_id == 0 {
			let info = client.account(account, None).await?;
			info.data.free
		} else {
			client
				.free_balance_of(account, currency_id, None)
				.await?
				.unwrap_or(0)
		};
		Ok(balance)
	}

	pub async fn currencies(
		client: &Client<XPredictRuntime>,
		currency_id: CurrencyId,
	) -> Result<PRC20, Box<dyn std::error::Error>> {
		let result = client.currencies(currency_id, None).await?;
		let token_info =
			result.ok_or_else(|| Into::<Box<dyn std::error::Error>>::into("no such currency"))?;
		Ok(token_info)
	}

	pub async fn number_of_currency(
		client: &Client<XPredictRuntime>,
	) -> Result<CurrencyId, Box<dyn std::error::Error>> {
		Ok(client.current_currency_id(None).await?.unwrap_or(0))
	}
}
