#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use frame_support::storage::Key;
    use pallet_uniques::{self as uniques};

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

        /// Limit for strings provided from outside.
        type StringLimit: Get<u32>;
	}

	/// Map where each identified NFT corresponds to its type.
    #[pallet::storage]
    pub type NftTypes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (T::CollectionId, T::ItemId),
        // The types of entities will be defined by a string provided from outside.
        BoundedVec<u8, <T as pallet::Config>::StringLimit>,
        ValueQuery
    >;

    /// Map a parent NFT (collection and ID) to its children (collection and ID). Use a boolean to 
    /// indicate the existence of the relationship.
    #[pallet::storage]
    pub type Hierarchy<T: Config> = StorageNMap<
		Key = (
			Key<Twox64Concat, T::CollectionId>,
			Key<Twox64Concat, T::ItemId>,
			Key<Twox64Concat, (T::CollectionId, T::ItemId)>,
		),
        // The types of entities will be defined by a string provided from outside.
		Value = BoundedVec<u8, <T as pallet::Config>::StringLimit>, 
		QueryKind = ValueQuery,
    >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RelationshipAdded {
            parent: (T::CollectionId, T::ItemId),
            child: (T::CollectionId, T::ItemId),
            relationship: BoundedVec<u8, <T as pallet::Config>::StringLimit>,
            who: T::AccountId,
        },
        RelationshipRemoved {
            parent: (T::CollectionId, T::ItemId),
            child: (T::CollectionId, T::ItemId),
            who: T::AccountId,
        },
    }

	#[pallet::error]
    pub enum Error<T> {
        TokenNotFound,
        NotOwner,
        InvalidRelationship,
        StringTooLong,
    }

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_child())]
        pub fn create_relationship(
            origin: OriginFor<T>,
            parent_collection: T::CollectionId,
            parent_item: T::ItemId,
            child_collection: T::CollectionId,
            child_item: T::ItemId,
            relationship: BoundedVec<u8, <T as pallet::Config>::StringLimit>,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            // Verify that the parent exists.
            ensure!(
                uniques::Pallet::<T>::owner(parent_collection.clone(), parent_item).is_some(),
                Error::<T>::TokenNotFound
            );

            // Verify that the child exists and belongs to the caller.
            ensure!(
                uniques::Pallet::<T>::owner(child_collection.clone(), child_item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            // Verify that the relationship is not empty.
            ensure!(!relationship.is_empty(), Error::<T>::InvalidRelationship);

            // Create relationship.
            Hierarchy::<T>::insert((parent_collection.clone(), parent_item, (child_collection.clone(), child_item)), relationship.clone());
            Self::deposit_event(Event::RelationshipAdded {
                parent: (parent_collection, parent_item),
                child: (child_collection, child_item),
                relationship,
                who,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::remove_child())]
        pub fn remove_relationship(
            origin: OriginFor<T>,
            parent_collection: T::CollectionId,
            parent_item: T::ItemId,
            child_collection: T::CollectionId,
            child_item: T::ItemId,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            // Verify that the child exists and belongs to the caller.
            ensure!(
                uniques::Pallet::<T>::owner(child_collection.clone(), child_item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            // Verify that the relationship exists.
            ensure!(
                Hierarchy::<T>::contains_key((parent_collection.clone(), parent_item, (child_collection.clone(), child_item))),
                Error::<T>::InvalidRelationship
            );

            // Remove the relationship.
            Hierarchy::<T>::remove((parent_collection.clone(), parent_item, (child_collection.clone(), child_item)));
            Self::deposit_event(Event::RelationshipRemoved {
                parent: (parent_collection, parent_item),
                child: (child_collection, child_item),
                who,
            });

            Ok(())
        }
	}
}
