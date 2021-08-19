use std::marker::PhantomData;

use codec::{Decode, Encode};
use subxt::{module, system::System, Call, Event, Store};

use super::RulerModule;

#[module]
pub trait Ruler: System {}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct RulerAddressStore<T: Ruler> {
	#[store(returns = Option<T::AccountId>)]
	pub ruler_module: RulerModule,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingRulerAddressStore<T: Ruler> {
	#[store(returns = Option<T::AccountId>)]
	pub ruler_module: RulerModule,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct PendingRulerAddressEvent<T: Ruler> {
	pub ruler_module: RulerModule,
	pub from: T::AccountId,
	pub to: T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AcceptRulerAddressEvent<T: Ruler> {
	pub ruler_module: RulerModule,
	pub account: T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferRulerAddressCall<'a, T: Ruler> {
	pub module: RulerModule,
	pub address: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct AcceptRulerAddressCall<T: Ruler> {
	pub module: RulerModule,
	pub _runtime: PhantomData<T>,
}
