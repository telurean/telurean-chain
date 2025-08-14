#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use pallet_uniques::{self as uniques};

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config + uniques::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;

        type KeyLimit: Get<u32>;
        type ValueLimit: Get<u32>;
        type AttributeLimit: Get<u32>;
    }

	/// Storage for attributes as key-value pairs.
    #[pallet::storage]
    pub type Attributes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        Blake2_128Concat,
        T::ItemId,
        BoundedVec<(BoundedVec<u8, <T as pallet::Config>::KeyLimit>, BoundedVec<u8, <T as pallet::Config>::ValueLimit>), T::AttributeLimit>,
        ValueQuery
    >;

	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AttributesSet {
            collection: T::CollectionId,
            item: T::ItemId,
            who: T::AccountId,
        },
        AttributesCleared {
            collection: T::CollectionId,
            item: T::ItemId,
            who: T::AccountId,
        },
    }

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
    pub enum Error<T> {
        TokenNotFound,
        NotOwner,
        AttributeLimitReached,
        KeyTooLong,
        ValueTooLong,
    }

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_attributes())]
        pub fn set_attributes(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            attributes: BoundedVec<(BoundedVec<u8, <T as pallet::Config>::KeyLimit>, BoundedVec<u8, <T as pallet::Config>::ValueLimit>), T::AttributeLimit>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                uniques::Pallet::<T>::owner(collection.clone(), item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            Attributes::<T>::insert(collection.clone(), item, attributes);
            Self::deposit_event(Event::AttributesSet { collection, item, who });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::clear_attributes())]
        pub fn clear_attributes(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                uniques::Pallet::<T>::owner(collection.clone(), item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            Attributes::<T>::remove(collection.clone(), item);
            Self::deposit_event(Event::AttributesCleared { collection, item, who });
            Ok(())
        }
	}
}
