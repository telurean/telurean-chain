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
    use frame_support::storage::Key;
    use frame_system::pallet_prelude::*;
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

        /// Limit on the number of elements in item groups.
        type ItemLimit: Get<u32>;

        /// Define the batch of NFTs retrieved per transaction.
        type MaxAssetsPerTransaction: Get<u32>;
    }

    /// Map where each identified NFT corresponds to its type.
    #[pallet::storage]
    pub type NftTypes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (T::CollectionId, T::ItemId),
        // The types of entities will be defined by a string provided from outside.
        BoundedVec<u8, <T as pallet::Config>::StringLimit>,
        ValueQuery,
    >;

    /// The following list of storage elements represents the relationships between
    /// different types of entities in Telurean Chain. In addition to specific relationships,
    /// there are two properties for user-defined relationships: one paginated to
    /// storage an indeterminate number of relationships and another limited.
    /// The purpose of storage segmentation is to minimize the gas impact of searches.

    /// Ownership relationship. This relationship is paginated by an asset counter for each owner.
    /// Only the ItemId is stored since the CollectionId matches that of the owner.
    #[pallet::storage]
    pub type OwnerAssets<T: Config> = StorageNMap<
        Key = (
            Key<Twox64Concat, T::CollectionId>,
            Key<Twox64Concat, T::ItemId>,
            Key<Twox64Concat, u64>, // Asset counter that acts as an index in pagination.
        ),
        Value = Option<T::ItemId>,
        QueryKind = ValueQuery,
    >;

    /// Counter of assets for each owner, which will serve as a paginator.
    #[pallet::storage]
    pub type AssetCount<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::CollectionId, T::ItemId), u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        OwnershipAdded {
            owner: (T::CollectionId, T::ItemId),
            asset: T::ItemId,
            who: T::AccountId,
        },
        OwnershipRemoved {
            owner: (T::CollectionId, T::ItemId),
            asset: T::ItemId,
            who: T::AccountId,
        },
        AssetsRetrieved {
            owner: (T::CollectionId, T::ItemId),
            assets: BoundedVec<T::ItemId, T::MaxAssetsPerTransaction>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        TokenNotFound,
        NotOwner,
        OwnershipNotFound,
        ExceededMaxAssetsPerQuery,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_ownership())]
        pub fn create_ownership(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            owner_item: T::ItemId,
            asset_item: T::ItemId,
        ) -> DispatchResult {
            // Verify that the owner and asset exist and belongs to the caller.
            let who = ensure_signed(origin)?;
            ensure!(
                uniques::Pallet::<T>::owner(collection.clone(), owner_item) == Some(who.clone()),
                Error::<T>::TokenNotFound
            );
            ensure!(
                uniques::Pallet::<T>::owner(collection.clone(), asset_item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            // Add new ownership relationship.
            let index = AssetCount::<T>::get((collection.clone(), owner_item));
            OwnerAssets::<T>::insert((collection.clone(), owner_item, index), Some(asset_item));
            AssetCount::<T>::mutate((collection.clone(), owner_item), |count| *count += 1);

            Self::deposit_event(Event::OwnershipAdded {
                owner: (collection.clone(), owner_item),
                asset: asset_item,
                who,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::remove_ownership())]
        pub fn remove_ownership(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            owner_item: T::ItemId,
            asset_item: T::ItemId,
        ) -> DispatchResult {
            // Verify that the owner and asset exist and belongs to the caller.
            let who = ensure_signed(origin)?;
            ensure!(
                uniques::Pallet::<T>::owner(collection.clone(), owner_item) == Some(who.clone()),
                Error::<T>::TokenNotFound
            );
            ensure!(
                uniques::Pallet::<T>::owner(collection.clone(), asset_item) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            // Search for the ownership relationship.
            let count = AssetCount::<T>::get((collection.clone(), owner_item));
            let mut found_index = None;
            for index in 0..count {
                if OwnerAssets::<T>::get((collection.clone(), owner_item, index)).is_some() {
                    found_index = Some(index);
                    break;
                }
            }
            let index = found_index.ok_or(Error::<T>::OwnershipNotFound)?;

            // Move the last relationship to the deleted index.
            let last_index = count - 1;
            if index < last_index {
                let last_child =
                    OwnerAssets::<T>::take((collection.clone(), owner_item, last_index));
                OwnerAssets::<T>::insert((collection.clone(), owner_item, index), last_child);
            }
            AssetCount::<T>::mutate((collection.clone(), owner_item), |count| *count -= 1);
            OwnerAssets::<T>::remove((collection.clone(), owner_item, last_index));

            Self::deposit_event(Event::OwnershipRemoved {
                owner: (collection.clone(), owner_item),
                asset: asset_item,
                who,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::get_owned_assets())]
        pub fn get_owned_assets(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            owner_item: T::ItemId,
            start_index: u64,
            num_assets: u32,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            let max_assets_per_query = T::MaxAssetsPerTransaction::get();
            ensure!(
                num_assets <= max_assets_per_query,
                Error::<T>::ExceededMaxAssetsPerQuery
            );

            // Ensure that the limit does not exceed MaxAssetsPerTransaction to avoid excessive reads.
            let num_assets_owned = AssetCount::<T>::get((collection.clone(), owner_item));
            let end_index = start_index
                .saturating_add(num_assets as u64)
                .min(num_assets_owned);
            let mut owned_nfts = BoundedVec::new();
            for i in start_index..end_index {
                if let Some(asset) = OwnerAssets::<T>::get((collection.clone(), owner_item, i)) {
                    owned_nfts
                        .try_push(asset)
                        .map_err(|_| Error::<T>::ExceededMaxAssetsPerQuery)?;
                }
            }

            // Emit an event with the retrieved assets.
            Self::deposit_event(Event::AssetsRetrieved {
                owner: (collection, owner_item),
                assets: owned_nfts,
            });

            Ok(())
        }
    }
}
