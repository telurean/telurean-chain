#![allow(non_snake_case)]

use super::super::*;
use crate::{mock::{new_test_ext, RuntimeOrigin, System, Test}};
use frame_support::{assert_ok, BoundedVec};

#[test]
fn works() {
    new_test_ext().execute_with(|| {
        let collec_id = 1u32;
        let asset_id = 2u128;
        let who = 1u64;

        // Create NFT for the owner.
        let tags: BoundedVec<BoundedVec<u8, <Test as pallet::Config>::StringLimit>, <Test as pallet::Config>::TypeLimit> = 
            BoundedVec::try_from(vec![
                BoundedVec::try_from(b"entity".to_vec()).unwrap(),
        ]).unwrap();
        assert_ok!(Pallet::<Test>::register_asset(
            RuntimeOrigin::signed(who),
            collec_id,
            asset_id,
            tags.clone()
        ));

        // Verify the NFT has been registered.
        let info = NftInfos::<Test>::get(asset_id);
        assert_eq!(info.collec_id, Some(collec_id));
        assert_eq!(info.owner_id, None);
        assert_eq!(info.tags, tags);
        
        // Verify event.
        System::assert_last_event(
            Event::<Test>::NftRegistered {
                collection: collec_id,
                asset: asset_id,
                who,
            }
            .into(),
        );
    });
}