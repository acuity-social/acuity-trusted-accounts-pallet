#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen)]
    pub struct Account<AccountId> {
        pub account_id: Option<AccountId>,
    }

	impl<AccountId> Default for Account<AccountId> {
	    fn default() -> Account<AccountId> {
			Self {
				account_id: None
			}
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn account_trusted_account_list_count)]
	// Mapping of account to count of accounts that it trusts.
	pub type AccountTrustedAccountListCount<T: Config> = StorageMap<_,
		Blake2_128Concat, T::AccountId,
		u32,
		ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_trusted_account_list)]
	// Mapping of account to array of trusted accounts.
	pub type AccountTrustedAccountList<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, u32,
		Account<T::AccountId>,
		ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_trusted_account_index)]
	// Mapping of account1 to mapping of account2 to index + 1 in accountTrustedAccountList.
	pub type AccountTrustedAccountIndex<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, T::AccountId,
		u32,
		ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// It is not possible to trust self
		TrustSelf,
		/// The account is already trusted.
		AlreadyTrusted,
		/// The account is not trusted.
		NotTrusted,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn trust_account(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			// Check that the sender is not attempting to trust themselves.
			if sender == account {
				Err(Error::<T>::TrustSelf)?;
			}
			// Check that the account is not already trusted.
			if <AccountTrustedAccountIndex<T>>::get(&sender, &account) != 0 {
				Err(Error::<T>::AlreadyTrusted)?;
			}
			let list_length = <AccountTrustedAccountListCount<T>>::get(&sender);

			//----------------------------------------

			<AccountTrustedAccountList<T>>::insert(&sender, list_length, Account::<T::AccountId> {
				account_id: Some(account.clone())
			});
			<AccountTrustedAccountListCount<T>>::insert(&sender, list_length + 1);
			<AccountTrustedAccountIndex<T>>::insert(&sender, &account, list_length + 1);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn untrust_account(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			// Get the index + 1 of the account to be removed
			let i = <AccountTrustedAccountIndex<T>>::get(&sender, &account);
			// Check the account is trusted.
			if i == 0 {
				Err(Error::<T>::NotTrusted)?;
			}

			//----------------------------------------

			// Delete the index from state.
			<AccountTrustedAccountIndex<T>>::remove(&sender, &account);
			// Get the list length.
			let list_length = <AccountTrustedAccountListCount<T>>::get(&sender);
			// Check if this is not the last account.
			if i == list_length {
				let moving_account = <AccountTrustedAccountList<T>>::get(&sender, list_length);
				<AccountTrustedAccountList<T>>::insert(&sender, i - 1, moving_account);
				<AccountTrustedAccountIndex<T>>::insert(&sender, &account, i);
			}
			// Remove the last account.
			<AccountTrustedAccountList<T>>::remove(&sender, &list_length - 1);
			<AccountTrustedAccountListCount<T>>::insert(&sender, &list_length);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}
}
