use std::marker::PhantomData;

use codec::{Decode, Encode};
use subxt::{balances::Balances, module, system::System, Call, Event, Store};

use super::{Balance, CurrencyId};

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
pub struct PRC20 {
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
}

#[module]
pub trait Tokens: System + Balances {}

#[derive(Call, Encode)]
pub struct NewAssetCall<T: Tokens> {
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewAssetEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub _runtime: PhantomData<T>,
}

#[derive(Call, Encode)]
pub struct MintCall<T: Tokens> {
	pub currency_id: CurrencyId,
	pub to: T::AccountId,
	pub number: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MintEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub to: T::AccountId,
	pub number: T::Balance,
}

#[derive(Call, Encode)]
pub struct TransferCall<T: Tokens> {
	pub currency_id: CurrencyId,
	pub to: T::AccountId,
	pub number: Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferEvent<T: Tokens> {
	pub currency_id: CurrencyId,
	pub from: T::AccountId,
	pub to: T::AccountId,
	pub number: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct FreeBalanceOfStore<'a, T: System> {
	#[store(returns = Option<Balance>)]
	pub account_id: &'a T::AccountId,
	pub currency_id: CurrencyId,
}

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
