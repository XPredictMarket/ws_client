use std::{collections::HashMap, fmt::Debug, slice::Iter};

use sp_core::Pair;
use sp_keyring::AccountKeyring;
use sp_runtime::traits::{IdentifyAccount, Verify};
use subxt::{sp_core::sr25519::Pair as Sr25519Pair, PairSigner, Runtime};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PairAuthority {
	Admin,
	Normal,
}

pub trait XPredictPairs<T>
where
	T: Runtime,
	T::Signature: From<<Sr25519Pair as Pair>::Signature>,
	<T::Signature as Verify>::Signer:
		From<<Sr25519Pair as Pair>::Public> + IdentifyAccount<AccountId = T::AccountId>,
{
	fn get_pair_iter(&self, authority: PairAuthority) -> Option<Iter<Sr25519Pair>>;
	fn get_pair_signer(
		&self,
		authority: PairAuthority,
		index: usize,
	) -> Option<PairSigner<T, Sr25519Pair>>;
	fn get_default_signer(&self, authority: PairAuthority) -> Option<PairSigner<T, Sr25519Pair>>;
	fn get_public(
		&self,
		authority: PairAuthority,
		index: usize,
	) -> Option<<Sr25519Pair as Pair>::Public>;
	fn get_signer(pair: &Sr25519Pair) -> PairSigner<T, Sr25519Pair>;
}

pub struct XPredictKeystore {
	pairs_map: HashMap<PairAuthority, Vec<Sr25519Pair>>,
}

impl XPredictKeystore {
	pub fn new() -> XPredictKeystore {
		let mut pairs_map = HashMap::new();
		pairs_map.insert(PairAuthority::Admin, vec![AccountKeyring::Alice.pair()]);
		pairs_map.insert(
			PairAuthority::Normal,
			vec![
				AccountKeyring::Bob.pair(),
				AccountKeyring::Charlie.pair(),
				AccountKeyring::Dave.pair(),
				AccountKeyring::Eve.pair(),
				AccountKeyring::Ferdie.pair(),
				AccountKeyring::One.pair(),
				AccountKeyring::Two.pair(),
			],
		);
		XPredictKeystore { pairs_map }
	}
}

impl<T> XPredictPairs<T> for XPredictKeystore
where
	T: Runtime,
	T::Signature: From<<Sr25519Pair as Pair>::Signature>,
	<T::Signature as Verify>::Signer:
		From<<Sr25519Pair as Pair>::Public> + IdentifyAccount<AccountId = T::AccountId>,
{
	fn get_pair_iter(&self, authority: PairAuthority) -> Option<Iter<Sr25519Pair>> {
		self.pairs_map.get(&authority).map(|pairs| pairs.iter())
	}

	fn get_pair_signer(
		&self,
		authority: PairAuthority,
		index: usize,
	) -> Option<PairSigner<T, Sr25519Pair>> {
		if let Some(pairs) = self.pairs_map.get(&authority) {
			pairs.get(index).map(|pair| Self::get_signer(pair))
		} else {
			None
		}
	}

	fn get_default_signer(&self, authority: PairAuthority) -> Option<PairSigner<T, Sr25519Pair>> {
		self.get_pair_signer(authority, 0)
	}

	fn get_public(
		&self,
		authority: PairAuthority,
		index: usize,
	) -> Option<<Sr25519Pair as Pair>::Public> {
		if let Some(pairs) = self.pairs_map.get(&authority) {
			pairs.get(index).map(|pair| pair.public())
		} else {
			None
		}
	}

	fn get_signer(pair: &Sr25519Pair) -> PairSigner<T, Sr25519Pair> {
		PairSigner::new(pair.clone())
	}
}
