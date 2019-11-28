//!  prototype module for bridging in ethereum poa blockcahin

#![recursion_limit = "128"]
#![cfg_attr(not(feature = "std"), no_std)]

// use blake2::Blake2b;
use codec::{Decode, Encode};
use rstd::vec::Vec;
use sr_eth_primitives::{
	header::EthHeader, BestBlock, BlockNumber as EthBlockNumber, H160, H256, H64, U128, U256, U512,
};
use support::{decl_event, decl_module, decl_storage, dispatch::Result, traits::Currency};
use system::ensure_signed;

use sr_primitives::RuntimeDebug;

use rlp::{decode, encode};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	//	type Hash: rstd::hash::Hash;
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct Proof {
	pub nodes: Vec<Vec<u8>>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct ActionRecord {
	pub index: u64,
	pub proof: Vec<u8>,
	pub header_hash: H256,
}

decl_storage! {
	trait Store for Module<T: Trait> as EthBridge {
		/// Anchor block that works as genesis block
		pub BeginHeader get(begin_header): Option<EthHeader>;
		/// Info of the best block header for now
		pub BestHeader get(best_header): BestBlock;
		///
		pub BlockList get(block_list): map EthBlockNumber => EthHeader;

		pub HeaderOf get(header_of): map H256 => Option<EthHeader>;

//		pub BestHashOf get(best_hash_of): map u64 => Option<H256>;
//		pub HashsOf get(hashs_of): map u64 => Vec<H256>;

		/// Block delay for verify transaction
		pub FinalizeNumber get(finalize_number): Option<u64>;

		pub ActionOf get(action_of): map T::Hash => Option<ActionRecord>;

		pub HeaderForIndex get(header_for_index): map H256 => Vec<(u64, T::Hash)>;
	}
	add_extra_genesis {
		config(header): Option<Vec<u8>>;
		build(|config| {
			if let Some(h) = &config.header {
				let header: EthHeader = rlp::decode(&h).expect("can't deserialize the header");
				BeginHeader::put(header.clone());

				<Module<T>>::genesis_header(header);
			} else {
//				BeginNumber::put(config.number);
			}
		});
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call
	where
		origin: T::Origin
	{
		fn deposit_event() = default;

		pub fn store_block_header(origin, header: EthHeader) {
			let _relayer = ensure_signed(origin)?;
			let _ = Self::verify(&header)?;

			<Module<T>>::deposit_event(RawEvent::NewHeader(header));
		}

		pub fn relay_receipt(origin, proof: ActionRecord) {
			// confirm that the block hash is right
			// get the receipt MPT trie root from the block header
			// Using receipt MPT trie root to verify the proof and index etc.
		}

		pub fn submit_header(origin, header: EthHeader) {
			// if header confirmed then return
			// if header in unverified header then challenge
		}
	}
}

decl_event! {
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId
	{
		NewHeader(EthHeader),
		TODO(AccountId),
	}
}

impl<T: Trait> Module<T> {
	pub fn genesis_header(header: EthHeader) {
		unimplemented!()
	}

	pub fn adjust_deposit_value() {
		unimplemented!()
	}

	/// 1. if exists?
	/// 2. verify (difficulty + prev_hash + nonce + re-org)
	/// 3. challenge
	fn verify(header: &EthHeader) -> Result {
		let number = header.number();
		Ok(())
	}

	fn _punish(_who: &T::AccountId) -> Result {
		unimplemented!()
	}
}
