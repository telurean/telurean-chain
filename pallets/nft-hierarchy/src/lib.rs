#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Common functionality.
mod common;

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
    use common::NftInfo;
    use frame_support::pallet_prelude::*;
    use frame_support::storage::Key;
    use sp_runtime::traits::StaticLookup;
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
    pub trait Config: frame_system::Config + uniques::Config + TypeInfo {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;

        /// Limit on the number of assignable tags, which make up the type, that define an NFT.
        type TypeLimit: Get<u32>;

        /// Define the batch of NFTs retrieved per transaction.
        type MaxAssetsPerTransaction: Get<u32>;
    }

    /// The following list of storage elements represents the relationships between different types
    /// of entities in Telurean Chain. In addition to specific relationships, there are two properties
    /// for user-defined relationships: one paginated to storage an indeterminate number of relationships
    /// and another limited. The purpose of storage segmentation is to minimize the gas impact of searches.

    /// Map where each NFT, identified by its ID, corresponds to a structure that stores the information
    /// about it. The key type is u128, so the system’s NFT limit is 2¹²⁸. Essentially infinite.
    #[pallet::storage]
    pub type NftInfos<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u128,
        NftInfo<T>,
        ValueQuery,
    >;

    // NFT identifiers are global to the blockchain and do not depend on the collection. They are incremental,
    // starting with 1 (not 0), and the next NFT in the list is always the current ID + 1.
    #[pallet::storage]
    pub type LastNftId<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Ownership relationship. This relationship is paginated by an asset counter for each owner.
    /// Only the ItemId is stored since the CollectionId matches that of the owner.
    #[pallet::storage]
    pub type OwnerAssets<T: Config> = StorageNMap<
        Key = (
            Key<Twox64Concat, T::CollectionId>,
            Key<Twox64Concat, T::ItemId>,
            Key<Twox64Concat, u128>, // Asset counter that acts as an index in pagination.
        ),
        Value = Option<T::ItemId>,
        QueryKind = ValueQuery,
    >;

    /// Counter of assets for each owner, which will serve as a paginator.
    #[pallet::storage]
    pub type AssetCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (T::CollectionId, T::ItemId),
        u128,
        ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NftRegistered {
            who: T::AccountId,
            collection: T::CollectionId,
            asset: T::ItemId,
        },
        OwnershipAdded {
            who: T::AccountId,
            owner: (T::CollectionId, T::ItemId),
            asset: T::ItemId,
        },
        OwnershipRemoved {
            who: T::AccountId,
            owner: (T::CollectionId, T::ItemId),
            asset: T::ItemId,
        },
        AssetsRetrieved {
            owner: (T::CollectionId, T::ItemId),
            assets: BoundedVec<T::ItemId, T::MaxAssetsPerTransaction>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        TokenNotFound,
        UnknownCollection,
        AlreadyExists,
        NotOwner,
        OwnershipNotFound,
        ExceededTypeLimit,
        ExceededMaxAssetsPerQuery,
        WrongNft,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> 
    where
        T::CollectionId: From<u32> + Into<u32> + Copy,
        T::ItemId: From<u128> + Into<u128>,
    {
        /// Register a new asset in the system, understanding an asset as an NFT that is not
        /// a collection. The specified collection must have been previously registered in the 
        /// Uniques pallet, or register_asset will produce an error.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::register_nft())]
        pub fn register_asset(
            origin: OriginFor<T>,
            collec_id: T::CollectionId,
            tags: BoundedVec<BoundedVec<u8, T::StringLimit>, T::TypeLimit>,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            // Verify that the collection exists and the NFT is not already created.
            ensure!(
                uniques::Collection::<T>::get(collec_id).is_some(),
                Error::<T>::UnknownCollection
            );

            let mut nft_id = LastNftId::<T>::get();
            if nft_id == 0 { nft_id = 1 } else { nft_id += 1 }
            let item_id: T::ItemId = nft_id.into();
            ensure!(
                uniques::Item::<T>::get(collec_id, item_id).is_none(),
                Error::<T>::AlreadyExists
            );

            // Verify that the type limit is not exceeded.
            ensure!(
                tags.len() <= T::TypeLimit::get() as usize,
                Error::<T>::ExceededTypeLimit
            );

            NftInfos::<T>::insert(nft_id, NftInfo { 
                collec_id: Some(collec_id),
                item_id: Some(item_id),
                owner_id: None,
                tags: tags
            });
            LastNftId::<T>::mutate(|id| *id = nft_id);

            // Mint the new NFT for the given collection and owner.
            uniques::Pallet::<T>::mint(
                T::RuntimeOrigin::signed(who.clone()),
                collec_id,
                item_id,
                <T::Lookup as StaticLookup>::unlookup(who.clone()))?;
            
            Self::deposit_event(Event::NftRegistered {
                who: who,
                collection: collec_id,
                asset: item_id,
            });

            Ok(())
        }

        /// The ownership relationship between NFTs (one NFT being the owner of another NFT) 
        /// is one of the basic relationships in Telurean Chain.
        #[pallet::call_index(1)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_ownership())]
        pub fn set_ownership(
            origin: OriginFor<T>,
            collec_id: T::CollectionId,
            owner_id: T::ItemId,
            asset_id: T::ItemId,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            // Verify that the owner and asset exist, belong to the caller.
            ensure!(
                uniques::Pallet::<T>::owner(collec_id, owner_id) == Some(who.clone()),
                Error::<T>::TokenNotFound
            );
            ensure!(
                uniques::Pallet::<T>::owner(collec_id, asset_id) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            let owner = NftInfos::<T>::get(owner_id.into());
            ensure!(
                owner.collec_id.is_some() && owner.is_type("owner"),
                Error::<T>::WrongNft
            );

            // Add new ownership relationship.
            let index = AssetCount::<T>::get((collec_id, owner_id));
            if index == 0 {
                AssetCount::<T>::insert((collec_id, owner_id), 0);
            }
            OwnerAssets::<T>::insert((collec_id, owner_id, index), Some(asset_id));
            AssetCount::<T>::mutate((collec_id, owner_id), |count| *count += 1);

            Self::deposit_event(Event::OwnershipAdded {
                owner: (collec_id, owner_id),
                asset: asset_id,
                who,
            });

            Ok(())
        }

        /// Operations on the ownership relationship in Telurean Chain are handled separately, 
        /// with unset_ownership being the symmetric function to set_ownership.
        #[pallet::call_index(2)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::remove_ownership())]
        pub fn unset_ownership(
            origin: OriginFor<T>,
            collec_id: T::CollectionId,
            owner_id: T::ItemId,
            asset_id: T::ItemId,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            // Verify that the owner and asset exist and belongs to the caller.
            ensure!(
                uniques::Pallet::<T>::owner(collec_id, owner_id) == Some(who.clone()),
                Error::<T>::TokenNotFound
            );
            ensure!(
                uniques::Pallet::<T>::owner(collec_id, asset_id) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            // Search for the ownership relationship.
            let count = AssetCount::<T>::get((collec_id, owner_id));
            let mut found_index = None;
            for index in 0..count {
                if OwnerAssets::<T>::get((collec_id, owner_id, index)).is_some() {
                    found_index = Some(index);
                    break;
                }
            }
            let index = found_index.ok_or(Error::<T>::OwnershipNotFound)?;

            // Move the last relationship to the deleted index.
            let last_index = count - 1;
            if index < last_index {
                let last_child = OwnerAssets::<T>::take((collec_id, owner_id, last_index));
                OwnerAssets::<T>::insert((collec_id, owner_id, index), last_child);
            }
            AssetCount::<T>::mutate((collec_id, owner_id), |count| *count -= 1);

            Self::deposit_event(Event::OwnershipRemoved {
                owner: (collec_id, owner_id),
                asset: asset_id,
                who,
            });

            Ok(())
        }

        /// Retrieve all NFTs that belong to another NFT through an ownership relationship. You must
        /// specify the number of assets to return, as there is a limit of 10 per query, and the index
        /// of the first asset, where this index refers to the order number of the owned asset. For 
        /// example: if El Cid owns his sword Tizona (1), his armor (2), his horse Babieca (3), and 
        /// 20 properties on the Valencia coast (4..23), and you want to retrieve the last 5 properties,
        /// start = 19 and num_assets = 5.
        #[pallet::call_index(3)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::get_owned_assets())]
        pub fn get_owned_assets(
            origin: OriginFor<T>,
            collec_id: T::CollectionId,
            owner_id: T::ItemId,
            start: u128,
            num_assets: u32,
        ) -> DispatchResult {

            let _ = ensure_signed(origin)?;

            let max_assets_per_query = T::MaxAssetsPerTransaction::get();
            ensure!(
                num_assets <= max_assets_per_query,
                Error::<T>::ExceededMaxAssetsPerQuery
            );

            // Ensure that the limit does not exceed MaxAssetsPerTransaction to avoid excessive reads.
            let num_assets_owned = AssetCount::<T>::get((collec_id, owner_id));
            let end = start
                .saturating_add(num_assets as u128)
                .min(num_assets_owned);
            let mut owned_nfts = BoundedVec::new();
            for i in start..end {
                if let Some(asset) = OwnerAssets::<T>::get((collec_id, owner_id, i)) {
                    owned_nfts
                        .try_push(asset)
                        .map_err(|_| Error::<T>::ExceededMaxAssetsPerQuery)?;
                }
            }

            // Emit an event with the retrieved assets.
            Self::deposit_event(Event::AssetsRetrieved {
                owner: (collec_id, owner_id),
                assets: owned_nfts,
            });

            Ok(())
        }
    }
}
