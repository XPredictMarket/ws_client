pub mod autonomy;
pub mod couple;
pub mod proposals;
pub mod tokens;

use codec::{Decode, Encode};

pub type CurrencyId = u32;
pub type Balance = u128;
pub type ProposalId = u32;
pub type VersionId = u32;
pub type CategoryId = u32;
pub type ChainId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum RulerModule {
	PlatformDividend,
	CrossChainBurn,
	NotUsed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum ProposalStatus {
	FormalPrediction,
	OriginalPrediction,
	WaitingForResults,
	ResultAnnouncement,
	Inlitigation,
	End,
}
