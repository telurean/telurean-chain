use crate::Config;
use sp_std::str;
use scale_info::TypeInfo;
use sp_runtime::{ BoundedVec, traits::Get };
use frame_support::pallet_prelude::{ Encode, Decode, MaxEncodedLen };

/// This structure represents the information corresponding to a single NFT: the collection it belongs to, 
/// its owner if it has one, and the tags that define its type.
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct NftInfo<T: Config> {
    pub collec_id: Option<T::CollectionId>,
    pub item_id: Option<T::ItemId>,
    pub owner_id: Option<T::ItemId>,
    pub tags: BoundedVec<BoundedVec<u8, T::StringLimit>, T::TypeLimit>,
}
impl<T: Config> NftInfo<T> {
    pub fn is_type(&self, tag: &str) -> bool {
        self.tags.iter().any(|bounded_vec| {
            str::from_utf8(&bounded_vec)
                .map(|s| s == tag)
                .unwrap_or(false)
        })
    }
}
impl<T: Config> MaxEncodedLen for NftInfo<T> {
    fn max_encoded_len() -> usize {
        let collection_len = T::CollectionId::max_encoded_len();
        let owner_len = T::ItemId::max_encoded_len();
        // In SCALE encoding, a BoundedVec<u8, N> is encoded as follows: A length prefix (up to 4 bytes for a u32,
        // indicating how many bytes the vector contains). The vector's data (up to N bytes, where N is T::StringLimit).
        // Therefore, the maximum size of a BoundedVec<u8, T::StringLimit> is 4 + T::StringLimit::get() bytes.
        let tag_len = 4 + T::TypeLimit::get() as usize * (4 + T::StringLimit::get() as usize);
        
        collection_len + owner_len + tag_len
    }
}
impl<T: Config> Default for NftInfo<T> {
    fn default() -> Self {
        NftInfo {
            collec_id: None,
            item_id: None,
            owner_id: None,
            tags: BoundedVec::default(),
        }
    }
}