use super::super::*;
use crate::mock::{new_test_ext, RuntimeOrigin, System, Test, Uniques};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn works() {
    new_test_ext().execute_with(|| {
        let collection = 0u32;
        let owner_item = 1u32;
        let owner = 1u64;

        // Create collection and NFTs with pallet_uniques.
        assert_ok!(Uniques::create(
            RuntimeOrigin::signed(owner),
            collection,
            owner
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(owner),
            collection,
            owner_item, // 1 -> 1 asset.
            owner
        ));

        for i in 2..=12 {
            assert_ok!(Uniques::mint(
                RuntimeOrigin::signed(owner),
                collection,
                i, // 2 3 4 5 -> 4 assets.
                owner
            ));
            assert_ok!(Pallet::<Test>::create_ownership(
                RuntimeOrigin::signed(owner),
                collection,
                owner_item,
                i,
            ));
        }

        // Retrieve the first 4 assets.
        assert_ok!(Pallet::<Test>::get_owned_assets(
            RuntimeOrigin::signed(owner),
            collection,
            owner_item,
            0,
            4,
        ));

        let expected_assets =
            BoundedVec::try_from((2..=5).map(|i| i).collect::<Vec<u32>>()).unwrap();
        System::assert_last_event(
            Event::<Test>::AssetsRetrieved {
                owner: (collection, owner_item),
                assets: expected_assets,
            }
            .into(),
        );
    });
}

#[test]
fn supports_maximum_exceeded() {
    new_test_ext().execute_with(|| {
        let collection = 0u32;
        let owner_item = 1u32;
        let owner = 1u64;

        // Create collection and NFTs with pallet_uniques.
        assert_ok!(Uniques::create(
            RuntimeOrigin::signed(owner),
            collection,
            owner
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(owner),
            collection,
            owner_item, // 1 -> 1 asset.
            owner
        ));

        for i in 2..=12 {
            assert_ok!(Uniques::mint(
                RuntimeOrigin::signed(owner),
                collection,
                i, // 2 3 4 5 6 7 8 9 10 11 12 -> 11 assets.
                owner
            ));
            assert_ok!(Pallet::<Test>::create_ownership(
                RuntimeOrigin::signed(owner),
                collection,
                owner_item,
                i,
            ));
        }

        // Test for exceeded limit.
        assert_noop!(
            Pallet::<Test>::get_owned_assets(
                RuntimeOrigin::signed(owner),
                collection,
                owner_item,
                0,
                11, // Excedes ExceededMaxAssetsPerQuery (10)
            ),
            Error::<Test>::ExceededMaxAssetsPerQuery
        );
    });
}