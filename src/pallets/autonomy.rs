use std::{collections::BTreeMap, marker::PhantomData};

use codec::{Decode, Encode};
use sp_runtime::{traits::Verify, MultiSignature};
use subxt::{module, system::System, Call, Event, Store};

use super::{Balance, CurrencyId, Moment, ProposalId};

#[module]
pub trait Autonomy: System {}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq)]
pub struct Payload<Public> {
	pub proposal_id: u32,
	pub result: u32,
	pub public: Public,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct StakedAccountStore<'a, T: Autonomy> {
	#[store(returns = Option<Balance>)]
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AutonomyAccountStore<'a, T: Autonomy> {
	#[store(returns = Option<()>)]
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct TemporaryResultsStore<'a, T: Autonomy> {
	#[store(returns = Option<CurrencyId>)]
	pub proposal_id: ProposalId,
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalAnnouncementStore<T: Autonomy> {
	#[store(returns = Option<Moment>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct StatisticalResultsStore<T: Autonomy> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct MinimalStakeNumberStore<T: Autonomy> {
	#[store(returns = Option<Balance>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct MinimalReportNumberStore<T: Autonomy> {
	#[store(returns = Option<Balance>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ReportStakedNumberStore<'a, T: Autonomy> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ReportAccountStore<'a, T: Autonomy> {
	#[store(returns = Option<BTreeMap<T::AccountId, (bool, Balance)>>)]
	pub proposal_id: ProposalId,
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct StatisticalReportStore<T: Autonomy> {
	#[store(returns = Option<Balance>)]
	pub proposal_id: ProposalId,
	pub approve: bool,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalReportTimeStore<T: Autonomy> {
	#[store(returns = Option<Moment>)]
	pub proposal_id: ProposalId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PublicityIntervalStore<T: Autonomy> {
	#[store(returns = Option<Moment>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ReportIntervalStore<T: Autonomy> {
	#[store(returns = Option<Moment>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct StakeEvent<T: Autonomy> {
	pub from: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UnStakeEvent<T: Autonomy> {
	pub from: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SlashEvent<T: Autonomy> {
	pub who: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TaggingEvent<T: Autonomy> {
	pub who: T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UnTaggingEvent<T: Autonomy> {
	pub who: T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UploadResultEvent<T: Autonomy> {
	pub who: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SetMinimalNumberEvent<T: Autonomy> {
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SetPublicityIntervalEvent<T: Autonomy> {
	pub time: Moment,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ReportEvent<T: Autonomy> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SecondedReportEvent<T: Autonomy> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub to: T::AccountId,
	pub approve: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TakeOutEvent<T: Autonomy> {
	pub from: T::AccountId,
	pub proposal_id: ProposalId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct StakeCall<T: Autonomy> {
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct UnStakeCall<T: Autonomy> {
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct Slashcall<'a, T: Autonomy> {
	pub who: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct UploadResultCall<T: Autonomy> {
	pub payload: Payload<<MultiSignature as Verify>::Signer>,
	pub signature: MultiSignature,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct ReportCall<'a, T: Autonomy> {
	pub proposal_id: ProposalId,
	pub target: &'a T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SecondedReportCall<'a, T: Autonomy> {
	pub proposal_id: ProposalId,
	pub target: &'a T::AccountId,
	pub number: Balance,
	pub support: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TakeOutCall<'a, T: Autonomy> {
	pub proposal_id: ProposalId,
	pub target: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TaggingCall<'a, T: Autonomy> {
	pub target: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct UntaggingCall<'a, T: Autonomy> {
	pub account: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetMinimalNumberCall<T: Autonomy> {
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetPublicityInterval<T: Autonomy> {
	pub interval: Moment,
	pub _runtime: PhantomData<T>,
}
