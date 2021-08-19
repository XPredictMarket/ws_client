#![allow(clippy::too_many_arguments)]
use std::marker::PhantomData;

use codec::{Decode, Encode};
use subxt::{module, system::System, Call, Event, Store};

use super::{Balance, CategoryId, CurrencyId, Moment, ProposalId};

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
pub struct Proposal {
	pub title: Vec<u8>,
	pub category_id: CategoryId,
	pub detail: Vec<u8>,
}

#[module]
pub trait Couple: System {}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct StakedAccountStore<T: Couple> {
	#[store(returns = Option<Proposal>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PoolPairsStore<T: Couple> {
	#[store(returns = Option<(CurrencyId, CurrencyId)>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalCurrencyIdStore<T: Couple> {
	#[store(returns = Option<CurrencyId>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalTotalVolumeStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalLiquidateCurrencyIdStore<T: Couple> {
	#[store(returns = Option<CurrencyId>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalTotalEarnTradingFeeStore<T: Couple> {
	#[store(returns = Option<u32>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalResultStore<T: Couple> {
	#[store(returns = Option<CurrencyId>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalAccountInfoStore<'a, T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalTotalMarketStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalTotalOptionalMarketStore<T: Couple> {
	#[store(returns = Option<(Balance, Balance)>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalFinallyTotalOptionalMarketStore<T: Couple> {
	#[store(returns = Option<(Balance, Balance)>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalTotalMarketFeeStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalFinallyMarketFeeStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalTotalMarketLiquidStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalFinallyMarketLiquidStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalOwnerAlreadyWithdrawnFeeStore<'a, T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalLiquidityProviderFeeRateStore<T: Couple> {
	#[store(returns = Option<u32>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalWithdrawalFeeRateStore<T: Couple> {
	#[store(returns = Option<u32>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalTotalAutonomyRewardStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalCurrentAutonomyRewardStore<T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalAccountRewardStartStore<'a, T: Couple> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AddLiquidityEvent<T: Couple> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RemoveLiquiditEvent<T: Couple> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct BuyEvent<T: Couple> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SellEvent<T: Couple> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RetrievalEvent<T: Couple> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SetResultEvent<T: Couple> {
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewProposalEvent<T: Couple> {
	pub who: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct WithdrawalRewardEvent<T: Couple> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct NewProposalCall<T: Couple> {
	pub title: Vec<u8>,
	pub optional: [Vec<u8>; 2],
	pub close_time: Moment,
	pub category_id: CategoryId,
	pub currency_id: CurrencyId,
	pub number: Balance,
	pub earn_fee: u32,
	pub detail: Vec<u8>,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct AddLiquidityCall<T: Couple> {
	pub proposal_id: ProposalId,
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RemoveLiquidityCall<T: Couple> {
	pub proposal_id: ProposalId,
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct BuyCall<T: Couple> {
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SellCall<T: Couple> {
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RetrievalCall<T: Couple> {
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct WithdrawalRewardCall<T: Couple> {
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetResultCall<T: Couple> {
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}
