use super::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use core::fmt::Debug;

/// Different types of proposals made by consuls and patricians. Each proposal is identified 
/// by a unique T::Hash derived from its content.
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, MaxEncodedLen, Debug)]
#[scale_info(skip_type_params(T))]
pub enum Proposal<T: Config> {
    ConsulNomination(T::Hash, T::AccountId, T::Moment),
    PatricianNomination(T::Hash, T::AccountId, T::Moment),
    NewBlock(T::Hash, BlockNumberFor<T>),
}

/// All possible decisions to be made by the consuls regarding the submitted proposals.
#[derive(Encode, Decode, DecodeWithMemTracking, TypeInfo, Clone, PartialEq, MaxEncodedLen, Debug)]
pub enum Decision {
    Approved,
    Rejected,
    NeedsInfo,
}