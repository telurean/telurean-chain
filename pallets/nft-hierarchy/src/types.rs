use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen, Default)]
#[scale_info(skip_type_params(T))]
pub enum NftType {
    #[default]
    Entity,
    Character,
}