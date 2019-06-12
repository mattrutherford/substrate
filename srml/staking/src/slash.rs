use srml_slashing::{OnSlashing, Misconduct};
use crate::{BalanceOf, Module, Trait};
use srml_support::traits::Currency;

use rstd::marker::PhantomData;
use rstd::cmp;

use srml_support::impl_outer_origin;
use substrate_primitives::H256;
use primitives::{
	traits::{BlakeTwo256, IdentityLookup},
	testing::{Digest, DigestItem, Header}
};

/// OnSlashing implementation for `Staking`
pub struct StakingSlasher<T, M> {
	t: PhantomData<T>,
	m: PhantomData<M>,
}

impl<T: Trait, M: Misconduct> OnSlashing<T::AccountId, M::Severity> for StakingSlasher<T, M> {
	fn on_slash(who: &T::AccountId, m: M::Severity) {
		let balance = <Module<T>>::slashable_balance(who);
		// don't work, need magic to cast slashing::Trait::Balance -> crate::Trait::Balance!
		let s: BalanceOf<T> = m.into();
		let slash = balance / balance;
		<Module<T>>::slash_validator(who, slash);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Unresponsive {
	severity: u64,
}

impl_outer_origin! {
	pub enum Origin for Unresponsive {}
}

impl Default for Unresponsive {
	fn default() -> Self {
		Self { severity: 100_000}
	}
}

impl Misconduct for Unresponsive {
	type Severity = u64;
	type Currency = balances::Module<Unresponsive>;

	fn on_misconduct(&mut self) {
		self.severity = cmp::max(1, self.severity / 2);
	}

	fn on_signal(&mut self) {
		self.severity = cmp::min(100_000, self.severity * 2);
	}

	fn severity(&self) -> Self::Severity {
		self.severity
	}
}


impl system::Trait for Unresponsive {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Digest = Digest;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type Log = DigestItem;
}

impl balances::Trait for Unresponsive {
	type Balance = u64;
	type OnFreeBalanceZero = ();
	type OnNewAccount = ();
	type Event = ();
	type TransactionPayment = ();
	type TransferPayment = ();
	type DustRemoval = ();
}
