use super::*;

#[allow(unused)]
use crate::Pallet as Attributes;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_attributes() {
        // TODO
	}

	#[benchmark]
	fn clear_attributes() {
        // TODO
	}

	impl_benchmark_test_suite!(Attributes, crate::mock::new_test_ext(), crate::mock::Test);
}
