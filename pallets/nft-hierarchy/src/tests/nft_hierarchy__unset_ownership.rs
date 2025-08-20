#![allow(non_snake_case)]

use super::super::*;
use crate::mock::{new_test_ext, RuntimeOrigin, System, Test};
use frame_support::{assert_ok, BoundedVec};

#[test]
fn works() {
    new_test_ext().execute_with(|| {
        let collec_id = 0u32;
        let owner_id = 1u128;
        let asset_id = 2u128;
        let who = 1u64;

        let tags: BoundedVec<BoundedVec<u8, <Test as pallet::Config>::StringLimit>, <Test as pallet::Config>::TypeLimit> = 
            BoundedVec::try_from(vec![
                BoundedVec::try_from(b"entity".to_vec()).unwrap(),
                BoundedVec::try_from(b"owner".to_vec()).unwrap(),
                BoundedVec::try_from(b"character".to_vec()).unwrap(),
        ]).unwrap();
        let _ = Pallet::<Test>::register_asset(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            tags
        );

        // Create NFT for the asset.
        let tags: BoundedVec<BoundedVec<u8, <Test as pallet::Config>::StringLimit>, <Test as pallet::Config>::TypeLimit> = 
            BoundedVec::try_from(
                vec![BoundedVec::try_from(b"entity".to_vec()).unwrap()]
            ).unwrap();
        let _ = Pallet::<Test>::register_asset(
            RuntimeOrigin::signed(who),
            collec_id,
            asset_id,
            tags
        );

        // Relationship creation.
        let _ = Pallet::<Test>::set_ownership(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            asset_id
        );

        // Verify relationship removal.
        assert_ok!(Pallet::<Test>::unset_ownership(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            asset_id
        ));

        // Verify event.
        System::assert_last_event(
            Event::<Test>::OwnershipRemoved {
                owner: (collec_id, owner_id),
                asset: asset_id,
                who,
            }
            .into(),
        );
    });
}