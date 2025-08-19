#![allow(non_snake_case)]

use super::super::*;
use crate::mock::{new_test_ext, RuntimeOrigin, System, Test, Uniques};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn works() {
    new_test_ext().execute_with(|| {
        let collec_id = 0u32;
        let owner_id = 1u128;
        let who = 1u64;

        // Create collection and NFTs with pallet_uniques.
        assert_ok!(Uniques::create(
            RuntimeOrigin::signed(who),
            collec_id,
            who
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id, // 1 -> 1 asset.
            who
        ));

        for i in 2..=12 {
            assert_ok!(Uniques::mint(
                RuntimeOrigin::signed(who),
                collec_id,
                i, // 2 3 4 5 -> 4 assets.
                who
            ));
            assert_ok!(Pallet::<Test>::set_ownership(
                RuntimeOrigin::signed(who),
                collec_id,
                owner_id,
                i,
            ));
        }

        // Retrieve the first 4 assets.
        assert_ok!(Pallet::<Test>::get_owned_assets(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id,
            0,
            4,
        ));

        let expected_assets =
            BoundedVec::try_from((2..=5).map(|i| i).collect::<Vec<u128>>()).unwrap();
        System::assert_last_event(
            Event::<Test>::AssetsRetrieved {
                owner: (collec_id, owner_id),
                assets: expected_assets,
            }
            .into(),
        );
    });
}

#[test]
fn supports_maximum_exceeded() {
    new_test_ext().execute_with(|| {
        let collec_id = 0u32;
        let owner_id = 1u128;
        let who = 1u64;

        // Create collection and NFTs with pallet_uniques.
        assert_ok!(Uniques::create(
            RuntimeOrigin::signed(who),
            collec_id,
            who
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(who),
            collec_id,
            owner_id, // 1 -> 1 asset.
            who
        ));

        for i in 2..=12 {
            assert_ok!(Uniques::mint(
                RuntimeOrigin::signed(who),
                collec_id,
                i, // 2 3 4 5 6 7 8 9 10 11 12 -> 11 assets.
                who
            ));
            assert_ok!(Pallet::<Test>::set_ownership(
                RuntimeOrigin::signed(who),
                collec_id,
                owner_id,
                i,
            ));
        }

        // Test for exceeded limit.
        assert_noop!(
            Pallet::<Test>::get_owned_assets(
                RuntimeOrigin::signed(who),
                collec_id,
                owner_id,
                0,
                11, // Excedes ExceededMaxAssetsPerQuery (10)
            ),
            Error::<Test>::ExceededMaxAssetsPerQuery
        );
    });
}