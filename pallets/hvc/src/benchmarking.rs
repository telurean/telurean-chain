use super::*;

#[allow(unused)]
use crate::Pallet as PalletHvc;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn propose_consul() {
		// let value = 100u32;
		// let caller: T::AccountId = whitelisted_caller();
		// #[extrinsic_call]
		// propose_consul(RawOrigin::Signed(caller), ...);

		// assert_eq!(Something::<T>::get(), Some(value));
	}

	impl_benchmark_test_suite!(PalletHvc, crate::mock::new_test_ext(), crate::mock::Test);
}
