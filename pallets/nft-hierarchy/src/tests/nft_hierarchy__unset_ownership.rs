#![allow(non_snake_case)]

use super::super::*;
use crate::mock::{new_test_ext, RuntimeOrigin, System, Test, Uniques};
use frame_support::assert_ok;

#[test]
fn works() {
    new_test_ext().execute_with(|| {
        let collec_id = 0u32;
        let owner_id = 1u128;
        let asset_id = 2u128;
        let who = 1u64;

        // Create collection and NFT with pallet_uniques.
        assert_ok!(Uniques::create(
            RuntimeOrigin::signed(who),
            collec_id,
            who
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            who
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(who),
            collec_id,
            asset_id,
            who
        ));

        // Verify relationship creation.
        let count = AssetCount::<Test>::get((collec_id, owner_id));
        assert_ok!(Pallet::<Test>::set_ownership(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            asset_id
        ));
        assert_eq!(
            OwnerAssets::<Test>::get((collec_id, owner_id, count)),
            Some(asset_id)
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