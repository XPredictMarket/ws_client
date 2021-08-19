#![allow(clippy::too_many_arguments)]
use std::marker::PhantomData;

use codec::{Decode, Encode};
use subxt::{module, system::System, Call, Event};

use crate::runtime::Moment;

use super::{Balance, CategoryId, CurrencyId, ProposalId};

#[module]
pub trait Couple: System {}

#[derive(Call, Encode)]
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

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewProposalEvent<T: Couple> {
	pub who: T::AccountId,
	pub proposal_id: ProposalId,
	pub currency_id: CurrencyId,
}
