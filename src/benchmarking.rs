#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::Pallet as Assets;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_support::bounded_vec;
use frame_support::sp_runtime::SaturatedConversion;
use frame_system::RawOrigin;

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

benchmarks! {

  create_class {
    let class_id: u32 = 100;
    let caller: T::AccountId = whitelisted_caller();
    let cost_in_u128: u128 = 1000000000000000000000;
    let cost: BalanceOf<T> = cost_in_u128.saturated_into::<BalanceOf<T>>();
    T::Currency::make_free_balance_be(&caller, cost);

  }: _(RawOrigin::Signed(caller.clone()), caller.clone(), class_id.into(), bounded_vec![0])
  verify {
        assert_event::<T>(Event::ClassCreated { class_id: class_id.into(), who: caller }.into());
    }

  impl_benchmark_test_suite!(Assets, crate::mock::new_test_ext(), crate::mock::Test)
}
