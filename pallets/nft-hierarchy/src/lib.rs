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

        /// Define the batch of NFTs retrieved per transaction.
        type MaxRelationshipsPerQuery: Get<u32>;
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

    /// TODO.
    #[pallet::storage]
    pub type OwnerAssets<T: Config> = StorageNMap<
		Key = (
			Key<Twox64Concat, T::CollectionId>,
			Key<Twox64Concat, T::ItemId>,
			Key<Twox64Concat, u64>, // Index.
		),
		Value = Option<(T::CollectionId, T::ItemId)>,
		QueryKind = ValueQuery,
    >;

    /// Counter of assets for each owner, which will serve as a paginator.
    #[pallet::storage]
    pub type AssetCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (T::CollectionId, T::ItemId),
        u64,
        ValueQuery,
    >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        OwnershipAdded {
            owner: (T::CollectionId, T::ItemId),
            asset: (T::CollectionId, T::ItemId),
            who: T::AccountId,
        },
        OwnershipRemoved {
            owner: (T::CollectionId, T::ItemId),
            asset: (T::CollectionId, T::ItemId),
            who: T::AccountId,
        },
    }

	#[pallet::error]
    pub enum Error<T> {
        TokenNotFound,
        NotOwner,
        OwnershipNotFound,
    }

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_ownership())]
        pub fn create_ownership(
            origin: OriginFor<T>,
            owner_collection: T::CollectionId,
            owner_item: T::ItemId,
            asset_collection: T::CollectionId,
            asset_item: T::ItemId,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;
            ensure!(
                uniques::Pallet::<T>::owner(owner_collection.clone(), owner_item).is_some(),
                Error::<T>::TokenNotFound
            );

            // Verify that the asset exists and belongs to the caller.
            ensure!(
                uniques::Pallet::<T>::owner(asset_collection.clone(), asset_item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            let index = AssetCount::<T>::get((owner_collection.clone(), owner_item));
            OwnerAssets::<T>::insert((owner_collection.clone(), owner_item, index), None::<(T::CollectionId, T::ItemId)>);
            AssetCount::<T>::mutate((owner_collection.clone(), owner_item), |count| *count += 1);

            Self::deposit_event(Event::OwnershipAdded {
                owner: (owner_collection, owner_item),
                asset: (asset_collection, asset_item),
                who,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::remove_ownership())]
        pub fn remove_ownership(
            origin: OriginFor<T>,
            owner_collection: T::CollectionId,
            owner_item: T::ItemId,
            asset_collection: T::CollectionId,
            asset_item: T::ItemId,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;
            ensure!(
                uniques::Pallet::<T>::owner(asset_collection.clone(), asset_item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            // Search for the ownership relationship.
            let count = AssetCount::<T>::get((owner_collection.clone(), owner_item.clone()));
            let mut found_index = None;
            for index in 0..count {
                if OwnerAssets::<T>::get((owner_collection.clone(), owner_item.clone(), index)).is_some() {
                    found_index = Some(index);
                    break;
                }
            }
            let index = found_index.ok_or(Error::<T>::OwnershipNotFound)?;

            // Move the last relationship to the deleted index.
            let last_index = count - 1;
            if index < last_index {
                let last_child = OwnerAssets::<T>::take((owner_collection.clone(), owner_item, last_index));
                OwnerAssets::<T>::insert((owner_collection.clone(), owner_item, index), last_child);
            }
            AssetCount::<T>::mutate((owner_collection.clone(), owner_item), |count| *count -= 1);
            OwnerAssets::<T>::remove((owner_collection.clone(), owner_item, last_index));

            Self::deposit_event(Event::OwnershipRemoved {
                owner: (owner_collection, owner_item),
                asset: (asset_collection, asset_item),
                who,
            });

            Ok(())
        }
    }
}
