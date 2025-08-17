use sp_std::str;
use crate::{Config, NftTypes};

pub fn nft_is_type<T: Config>(
    collection: T::CollectionId,
    owner_item: T::ItemId,
    nft_type: &str,
) -> bool {

    let types = NftTypes::<T>::get((collection.clone(), owner_item));
    types.iter().any(|bounded_vec| {
        str::from_utf8(&bounded_vec)
            .map(|s| s == nft_type)
            .unwrap_or(false)
    })
}