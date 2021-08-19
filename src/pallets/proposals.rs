use std::marker::PhantomData;

use codec::{Decode, Encode};
use subxt::{module, system::System, Call, Event, Store};

use super::{Balance, CurrencyId, Moment, ProposalId, ProposalStatus, VersionId};

#[module]
pub trait Proposals: System {}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct CurrentProposalIdStore<T: Proposals> {
	#[store(returns = Option<ProposalId>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalLiquidateVersionIdStore<T: Proposals> {
	#[store(returns = Option<VersionId>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalStatusStore<T: Proposals> {
	#[store(returns = Option<ProposalStatus>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalOwnerStore<T: Proposals> {
	#[store(returns = Option<T::AccountId>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalUsedCurrencyIdStore<T: Proposals> {
	#[store(returns = Option<bool>)]
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalCloseTimeStore<T: Proposals> {
	#[store(returns = Option<Moment>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalCreateTimeStore<T: Proposals> {
	#[store(returns = Option<Moment>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalAnnouncementTimeStore<T: Proposals> {
	#[store(returns = Option<Moment>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalAutomaticExpirationTimeStore<T: Proposals> {
	#[store(returns = Option<Moment>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalMinimumIntervalTimeStore<T: Proposals> {
	#[store(returns = Option<Moment>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalVoteStakeStore<'a, T: Proposals> {
	#[store(returns = Option<(Balance, bool)>)]
	pub proposal_id: ProposalId,
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalCountVoteStore<T: Proposals> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub approve: bool,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct MinimumVoteStore<T: Proposals> {
	#[store(returns = Option<Balance>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct DefaultRewardStore<T: Proposals> {
	#[store(returns = Option<Balance>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalRewardStore<T: Proposals> {
	#[store(returns = Option<Balance>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProposalStatusChangedEvent<T: Proposals> {
	pub proposal_id: ProposalId,
	pub state: ProposalStatus,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct StakeToEvent<T: Proposals> {
	pub who: T::AccountId,
	pub proposal_id: ProposalId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UnStakeFromEvent<T: Proposals> {
	pub who: T::AccountId,
	pub proposal_id: ProposalId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DepositRewardEvent<T: Proposals> {
	pub from: T::AccountId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ReclaimRewardEvent<T: Proposals> {
	pub from: T::AccountId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct WithdrawalRewardEvent<T: Proposals> {
	pub who: T::AccountId,
	pub proposal_id: ProposalId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetStatusCall<T: Proposals> {
	pub proposal_id: ProposalId,
	pub new_status: ProposalStatus,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct StakeToCall<T: Proposals> {
	pub proposal_id: ProposalId,
	pub number: Balance,
	pub opinion: bool,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct UnstakeFromCall<T: Proposals> {
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct WithdrawalRewardCall<T: Proposals> {
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct DepositRewardCall<T: Proposals> {
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct ReclaimRewardCall<'a, T: Proposals> {
	pub to: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetProposalMinimumIntervalTimeCall<T: Proposals> {
	pub time: Moment,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetDefaultRewardCall<T: Proposals> {
	pub value: Balance,
	pub _runtime: PhantomData<T>,
}
