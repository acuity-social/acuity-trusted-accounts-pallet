use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

/*
#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}
*/

#[test]
fn trust_account_cant_trust_self_control() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::trust_account(Origin::signed(1), 2));
	});
}

#[test]
fn trust_account_cant_trust_self() {
	new_test_ext().execute_with(|| {
		assert_noop!(TemplateModule::trust_account(Origin::signed(1), 1), Error::<Test>::TrustSelf);
	});
}

#[test]
fn trust_account_already_trusted_control() {
	new_test_ext().execute_with(|| {
		TemplateModule::trust_account(Origin::signed(1), 2);
		assert_ok!(TemplateModule::trust_account(Origin::signed(1), 3));
	});
}

#[test]
fn trust_account_already_trusted() {
	new_test_ext().execute_with(|| {
		TemplateModule::trust_account(Origin::signed(1), 2);
		assert_noop!(TemplateModule::trust_account(Origin::signed(1), 2), Error::<Test>::AlreadyTrusted);
	});
}
