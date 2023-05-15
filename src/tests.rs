use crate::mock::*;
use frame_support::{assert_ok, bounded_vec};

pub fn test_create_class() {
    assert_ok!(Asset::do_create_class(&1, &1, 2000, bounded_vec![0]));
}

#[test]
fn test_verify_class_id_successful() {
    new_test_ext().execute_with(|| {
        test_create_class();
        assert_eq!(Asset::verify_class_id_(2000), true);
    })
}

#[test]
fn test_verify_class_id_not_initialized() {
    new_test_ext().execute_with(|| {
        assert_eq!(Asset::verify_class_id_(2000), false);
    })
}

#[test]
fn test_verify_class_id_failed() {
    new_test_ext().execute_with(|| {
        assert_eq!(Asset::verify_class_id_(2000), false);
    })
}

// #[test]
// fn mint_bundle_fails() {
//     new_test_ext().execute_with(|| {
//         assert_err!(
//             Bundle::do_mint_bundles(&2, &2, &2, bundle_id, 10),
//             Error::<Test>::InsufficientBalance
//         );

//         assert_eq!(
//             vec![9, 200, 300, 400, 500,],
//             Asset::balance_of_single_owner_batch(&2, 4000, asset_ids.to_vec()).unwrap()
//         );
//     })
// }
