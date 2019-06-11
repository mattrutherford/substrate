use srml_slashing::{OnSlashing, Misconduct};
use crate::{Module, Trait};

use rstd::marker::PhantomData;
use rstd::cmp;

/// OnSlashing implementation for `Staking`
pub struct StakingSlasher<T>(PhantomData<T>);

impl<T: Trait, M: Misconduct<T::AccountId>> OnSlashing<T::AccountId, M> for StakingSlasher<T> {

	fn on_slash(who: &T::AccountId, severity: M::Severity) {
		let balance = <Module<T>>::slashable_balance(who);
		let slash = balance / severity.into();

		<Module<T>>::slash_validator(who, slash);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Unresponsive {
	severity: u64,
}

impl Default for Unresponsive {
	fn default() -> Self {
		Self { severity: 100_000}
	}
}

impl<T> Misconduct<T> for Unresponsive {
	type Severity = u64;

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
