#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::Get};
use frame_system::ensure_root;
use sp_std::prelude::*;
use pallet_balances;
use pallet_nft::{Token, Params, Socket, Rarity, TokenId};
use pallet_nft as NFT;
use sp_runtime::traits::StaticLookup;
// use std::collections::HashSet;
use codec::{Decode, Encode, EncodeLike};

// extern crate clap;
// #[macro_use]
// extern crate prettytable;
// extern crate reqwest;
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config + pallet_nft::Config + pallet_balances::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}


decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		TokenMinted(AccountId, TokenId),
		TokenTransferred(TokenId, AccountId),
        TokenBreeded(TokenId),
        // TokensTransferred(TokenId, AccountId, TokenId, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///
		TokenExist,
		///
		NotTokenOwner,
		///
		NonExistentToken,
		///
		NotNftMaster
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
        		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

        #[weight = 10_000]
        pub fn mint_nft(origin, target_account: <T as frame_system::Config>::AccountId, token_id: pallet_nft::TokenId) -> dispatch::DispatchResult {
           let minted_token = pallet_nft::Module::<T>::mint_basic_nft(&target_account, token_id);
           Ok(())
        }

        // #[weight = 10_000]
        // pub fn breed_nft(origin, target_account: <T as frame_system::Config>::AccountId) -> dispatch::DispatchResult {
        //     let token_id = 3;
        //     let rarity = NFT::Rarity::Legendary;
        //     let socket = NFT::Socket::Head;
        //     let params = NFT::Params{strength: 2,
        //         agility: 2,
        //         intelligence: 9 };
        //     NFT::Module::<T>::burn(origin.clone(), 1);
        //     NFT::Module::<T>::burn(origin.clone(), 2);
        //     return NFT::Module::<T>::mint(origin, target_account, token_id, rarity, socket, params);
        // }

        #[weight = 10_000]
        pub fn transfer_nft(origin, dest_account: T::AccountId, token_id: pallet_nft::TokenId) {
            let json_nft = 123;
            return NFT::Module::<T>::transfer_basic_nft(&dest_account, token_id);
        }


        #[weight = 10_000]
        pub fn transfer(origin, dest: <T::Lookup as StaticLookup>::Source, value: <T as pallet_balances::Config>::Balance) -> dispatch::DispatchResultWithPostInfo {
            return pallet_balances::Pallet::<T>::transfer(origin, dest, value);
		}
    }
}


// fn get_from_json() -> Token {
//     let client = reqwest::Client::new();
//     let mut request = client.get(URL);

//     let mut resp = request.send().unwrap();

//     assert!(resp.status().is_success());
//     return resp;
// }