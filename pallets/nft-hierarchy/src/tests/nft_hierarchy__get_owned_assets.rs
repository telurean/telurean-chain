#![allow(non_snake_case)]

use super::super::*;
use crate::mock::{new_test_ext, RuntimeOrigin, System, Test};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn works() {
    new_test_ext().execute_with(|| {
        let collec_id = 0u32;
        let owner_id = 1u128;
        let who = 1u64;

        // Create NFTs for the owner.
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

        // Create NFTs for the asset.
        let tags: BoundedVec<BoundedVec<u8, <Test as pallet::Config>::StringLimit>, <Test as pallet::Config>::TypeLimit> = 
            BoundedVec::try_from(
                vec![BoundedVec::try_from(b"entity".to_vec()).unwrap()]
            ).unwrap();
        for i in 2..=12 {
            let _ = Pallet::<Test>::register_asset(
                RuntimeOrigin::signed(who),
                collec_id,
                i,
                tags.clone()
            );
            let _ = Pallet::<Test>::set_ownership(
                RuntimeOrigin::signed(who),
                collec_id,
                owner_id,
                i,
            );
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