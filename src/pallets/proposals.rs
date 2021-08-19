use std::marker::PhantomData;

use codec::{Decode, Encode};
use subxt::{module, system::System, Call, Event, Store};

use super::{ProposalId, ProposalStatus};

#[module]
pub trait Proposals: System {}

#[derive(Call, Encode)]
pub struct SetStatusCall<T: Proposals> {
	pub proposal_id: ProposalId,
	pub new_status: ProposalStatus,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProposalStatusChangedEvent<T: Proposals> {
	pub proposal_id: ProposalId,
	pub state: ProposalStatus,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct CurrentProposalIdStore<T: Proposals> {
	#[store(returns = Option<ProposalId>)]
	pub _runtime: PhantomData<T>,
}
