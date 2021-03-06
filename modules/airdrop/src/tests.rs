//! Unit tests for the airdrop module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Airdrop, ExtBuilder, Origin, System, TestEvent, ACA, ALICE, BOB, KAR};
use sp_runtime::traits::BadOrigin;

#[test]
fn airdrop_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(Airdrop::airdrop(Origin::signed(BOB), ALICE, KAR, 10000), BadOrigin,);
		assert_ok!(Airdrop::airdrop(Origin::ROOT, ALICE, KAR, 10000));
		let airdrop_event = TestEvent::airdrop(RawEvent::Airdrop(ALICE, KAR, 10000));
		assert!(System::events().iter().any(|record| record.event == airdrop_event));
		assert_eq!(Airdrop::airdrops(ALICE, KAR), 10000);
	});
}

#[test]
fn update_airdrop_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Airdrop::airdrop(Origin::ROOT, ALICE, ACA, 10000));
		assert_ok!(Airdrop::airdrop(Origin::ROOT, ALICE, ACA, 10000));
		assert_eq!(Airdrop::airdrops(ALICE, ACA), 20000);
		assert_noop!(Airdrop::update_airdrop(Origin::signed(BOB), ALICE, ACA, 0), BadOrigin,);
		assert_ok!(Airdrop::update_airdrop(Origin::ROOT, ALICE, ACA, 0));
		let update_airdrop_event = TestEvent::airdrop(RawEvent::UpdateAirdrop(ALICE, ACA, 0));
		assert!(System::events()
			.iter()
			.any(|record| record.event == update_airdrop_event));
		assert_eq!(Airdrop::airdrops(ALICE, ACA), 0);
	});
}
