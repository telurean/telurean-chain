use super::*;
use crate::mock::{RuntimeOrigin, new_test_ext, Test, System, Uniques};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_relationship_works() {
    new_test_ext().execute_with(|| {
        let collection_id = 0u32;
        let parent_id = 1u32;
        let child_id = 2u32;
        let owner = 1u64;
        let relationship = BoundedVec::try_from(b"wielder-weapon".to_vec()).unwrap();
        let sword_type = BoundedVec::try_from(b"sword".to_vec()).unwrap();
        let human_type = BoundedVec::try_from(b"human".to_vec()).unwrap();

        // Create collection and NFTs with pallet_uniques.
        assert_ok!(Uniques::create(
            RuntimeOrigin::signed(owner),
            collection_id,
            owner
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(owner),
            collection_id,
            parent_id,
            owner
        ));
        assert_ok!(Uniques::mint(
            RuntimeOrigin::signed(owner),
            collection_id,
            child_id,
            owner
        ));

        // Verify NFT types creation.
        NftTypes::<Test>::insert((collection_id, parent_id), sword_type.clone());
        NftTypes::<Test>::insert((collection_id, child_id), human_type.clone());
        assert_eq!(NftTypes::<Test>::get((collection_id, parent_id)), sword_type);
        assert_eq!(NftTypes::<Test>::get((collection_id, child_id)), human_type);

        // Verify relationship creation.
        assert_ok!(Pallet::<Test>::create_relationship(
            RuntimeOrigin::signed(owner),
            collection_id,
            parent_id,
            collection_id,
            child_id,
            relationship.clone()
        ));
        assert_eq!(
            Hierarchy::<Test>::get((collection_id, parent_id, (collection_id, child_id))),
            relationship
        );

        // Verify event.
        System::assert_last_event(Event::<Test>::RelationshipAdded {
            parent: (collection_id, parent_id),
            child: (collection_id, child_id),
            relationship,
            who: owner,
        }.into());
    });
}