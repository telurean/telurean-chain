use super::super::*;
use crate::mock::{new_test_ext, RuntimeOrigin, System, Test, Uniques};
use frame_support::assert_ok;

#[test]
fn adds_assets() {
    new_test_ext().execute_with(|| {
        let collection = 0u32;
        let owner_item = 1u32;
        let asset_item = 2u32;
        let owner = 1u64;

        // Create collection and NFT with pallet_uniques.
        assert_ok!(Uniques::create(
            RuntimeOrigin::signed(owner),
            collection,
            owner
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(owner),
            collection,
            owner_item,
            owner
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(owner),
            collection,
            asset_item,
            owner
        ));

        // Verify relationship creation.
        let count = AssetCount::<Test>::get((collection, owner_item));
        assert_ok!(Pallet::<Test>::create_ownership(
            RuntimeOrigin::signed(owner),
            collection,
            owner_item,
            asset_item
        ));
        assert_eq!(AssetCount::<Test>::get((collection, owner_item)), count + 1);
        assert_eq!(
            OwnerAssets::<Test>::get((collection, owner_item, count)),
            Some(asset_item)
        );

        // Verify event.
        System::assert_last_event(
            Event::<Test>::OwnershipAdded {
                owner: (collection, owner_item),
                asset: asset_item,
                who: owner,
            }
            .into(),
        );
    });
}