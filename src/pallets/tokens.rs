use std::{collections::BTreeMap, marker::PhantomData};

use codec::{Decode, Encode};
use subxt::{module, system::System, Call, Event, Store};

use super::{Balance, CurrencyId};

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
pub struct PRC20 {
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
}

#[module]
pub trait Tokens: System {}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct CurrentCurrencyIdStore<T: System> {
	#[store(returns = Option<CurrencyId>)]
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct CurrenciesStore<T: System> {
	#[store(returns = Option<PRC20>)]
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct TotalSupplyStore<T: System> {
	#[store(returns = Option<Balance>)]
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct FreeBalanceOfStore<'a, T: System> {
	#[store(returns = Option<Balance>)]
	pub account_id: &'a T::AccountId,
	pub currency_id: CurrencyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ReserveOfStore<'a, T: System> {
	#[store(returns = Option<Balance>)]
	pub account_id: &'a T::AccountId,
	pub currency_id: CurrencyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AllowanceStore<'a, T: System> {
	#[store(returns = Option<BTreeMap<T::AccountId, Balance>>)]
	pub account_id: &'a T::AccountId,
	pub currency_id: CurrencyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewAssetEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MintEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct BurnEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct BurnFromEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub from: T::AccountId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub from: T::AccountId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferFromEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub who: T::AccountId,
	pub from: T::AccountId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ApprovalEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub from: T::AccountId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct NewAssetCall<T: Tokens> {
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct MintCall<'a, T: Tokens> {
	pub currency_id: CurrencyId,
	pub to: &'a T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct BurnCall<T: Tokens> {
	pub currency_id: CurrencyId,
	pub number: Balance,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct BurnFromCall<'a, T: Tokens> {
	pub currency_id: CurrencyId,
	pub from: &'a T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferCall<'a, T: Tokens> {
	pub currency_id: CurrencyId,
	pub to: &'a T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferFromCall<'a, T: Tokens> {
	pub currency_id: CurrencyId,
	pub from: &'a T::AccountId,
	pub to: &'a T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct ApproveCall<'a, T: Tokens> {
	pub currency_id: CurrencyId,
	pub spender: &'a T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct AddApproveCall<'a, T: Tokens> {
	pub currency_id: CurrencyId,
	pub spender: &'a T::AccountId,
	pub number: Balance,
}
