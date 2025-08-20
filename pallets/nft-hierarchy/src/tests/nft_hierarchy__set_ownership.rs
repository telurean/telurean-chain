#![allow(non_snake_case)]

use super::super::*;
use crate::mock::{new_test_ext, RuntimeOrigin, System, Test};
use frame_support::{assert_ok, BoundedVec};

#[test]
fn works() {
    new_test_ext().execute_with(|| {
        let collec_id = 1u32;
        let owner_id = 1u128;
        let asset_id = 2u128;
        let who = 1u64;

        // Create NFT for the owner.
        let tags: BoundedVec<BoundedVec<u8, <Test as pallet::Config>::StringLimit>, <Test as pallet::Config>::TypeLimit> = 
            BoundedVec::try_from(vec![
                BoundedVec::try_from(b"entity".to_vec()).unwrap(),
                BoundedVec::try_from(b"owner".to_vec()).unwrap(),
                BoundedVec::try_from(b"character".to_vec()).unwrap(),
        ]).unwrap();
        assert_ok!(Pallet::<Test>::register_asset(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            tags
        ));

        // Create NFT for the asset.
        let tags: BoundedVec<BoundedVec<u8, <Test as pallet::Config>::StringLimit>, <Test as pallet::Config>::TypeLimit> = 
            BoundedVec::try_from(
                vec![BoundedVec::try_from(b"entity".to_vec()).unwrap()]
            ).unwrap();
        assert_ok!(Pallet::<Test>::register_asset(
            RuntimeOrigin::signed(who),
            collec_id,
            asset_id,
            tags
        ));

        // Verify relationship creation.
        assert_ok!(Pallet::<Test>::set_ownership(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            asset_id
        ));
        assert_eq!(AssetCount::<Test>::get((collec_id, owner_id)), 1);
        assert_eq!(
            OwnerAssets::<Test>::get((collec_id, owner_id, 0)),
            Some(asset_id)
        );

        // Verify event.
        System::assert_last_event(
            Event::<Test>::OwnershipAdded {
                owner: (collec_id, owner_id),
                asset: asset_id,
                who,
            }
            .into(),
        );
    });
}