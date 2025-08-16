use crate as pallet_nft_hierarchy;
use frame_support::{derive_impl, parameter_types, traits::{VariantCountOf, ConstU32}};
use frame_system::{EnsureRoot, EnsureSigned};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

#[frame_support::runtime]
mod runtime {
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type Uniques = pallet_uniques::Pallet<Test>;

    #[runtime::pallet_index(3)]
    pub type NftHierarchy = pallet_nft_hierarchy::Pallet<Test>;
}

// frame_system
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type AccountData = pallet_balances::AccountData<u128>;
}

// pallet_balances
parameter_types! {
    pub const ExistentialDeposit: u128 = 1_000_000_000_000;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type DoneSlashHandler = ();
}

// pallet_uniques
parameter_types! {
    pub const CollectionDeposit: u128 = 1_000_000_000_000;
    pub const ItemDeposit: u128 = 100_000_000_000;
    pub const MetadataDepositBase: u128 = 100_000_000_000;
    pub const AttributeDepositBase: u128 = 10_000_000_000;
    pub const DepositPerByte: u128 = 1_000_000_000;
    pub const UniquesStringLimit: u32 = 128;
    pub const KeyLimit: u32 = 64;
    pub const ValueLimit: u32 = 256;
}

impl pallet_uniques::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CollectionId = u32;
    type ItemId = u32;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<Self::AccountId>;
    type CreateOrigin = EnsureSigned<Self::AccountId>;
    type Locker = ();
    type CollectionDeposit = CollectionDeposit;
    type ItemDeposit = ItemDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type AttributeDepositBase = AttributeDepositBase;
    type DepositPerByte = DepositPerByte;
    type StringLimit = UniquesStringLimit;
    type KeyLimit = KeyLimit;
    type ValueLimit = ValueLimit;
    type WeightInfo = pallet_uniques::weights::SubstrateWeight<Test>;
}

// pallet_nft_hierarchy
parameter_types! {
    pub const StringLimit: u32 = 128;
    pub const MaxRelationshipsPerQuery: u32 = 10;
}

impl pallet_nft_hierarchy::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_nft_hierarchy::weights::SubstrateWeight<Test>;
    type StringLimit = StringLimit;
    type MaxRelationshipsPerQuery = MaxRelationshipsPerQuery;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    // Añadir saldo inicial a una cuenta para pruebas
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 1_000_000_000_000_000)], // Cuenta 1 con saldo suficiente
        dev_accounts: Some((1, 1_000_000_000_000_000, None)),
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
