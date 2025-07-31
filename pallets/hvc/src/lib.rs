// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file.
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// Submodules import.
mod config;
mod events;
mod errors;
mod storage;
mod dispatchables;
mod types;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::Hash;
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Re-exports types from the `types` module.
    pub use super::types::{Proposal, Decision};

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        
        /// Maximum required limit for the BoundedVec type corresponding to Consuls.
        #[pallet::constant]
        type MaxConsuls: Get<u32>;

        /// Maximum required limit for the BoundedVec type corresponding to Patricians.
        #[pallet::constant]
        type MaxPatricians: Get<u32>;

        /// Maximum required limit for the BoundedVec type corresponding to Terms.
        #[pallet::constant]
        type MaxTerms: Get<u32>;

        /// Lifetime of proposals in days, after which they expire.
        #[pallet::constant]
        type ProposalDuration: Get<u32>;

        /// Maximum limit required for the BoundedVec type corresponding to Proposals.
        #[pallet::constant]
        type MaxProposals: Get<u32>;

        /// Maximum limit required for the BoundedVec type corresponding to Assignments.
        #[pallet::constant]
        type MaxAssignments: Get<u32>;

        /// Maximum limit required for the BoundedVec type corresponding to Assignments.
        #[pallet::constant]
        type MaxReasonLength: Get<u32>;
    }

    /// List of all system validators, known as consuls. They are responsible for validating new blocks.
    #[pallet::storage]
    #[pallet::getter(fn consuls)]
    pub type Consuls<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxConsuls>, ValueQuery>;

    /// List of all game masters in the system, known as patricians. They are responsible for proposing new blocks.
    #[pallet::storage]
    #[pallet::getter(fn patricians)]
    pub type Patricians<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxPatricians>, ValueQuery>;

    /// List of elite members along with their terms of office (pair of AccountId and deadline).
    #[pallet::storage]
    #[pallet::getter(fn terms)]
    pub type Terms<T: Config> = StorageValue<_, BoundedVec<(T::AccountId, T::Moment), T::MaxTerms>, ValueQuery>;

    /// List of proposals to add new consuls, patricians, blocks... to the system.
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageValue<_, BoundedVec<(T::AccountId, Proposal<T>), T::MaxProposals>, ValueQuery>;

    /// List of proposal validation assignments to consuls.
    #[pallet::storage]
    #[pallet::getter(fn assignments)]
    pub type Assignments<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, BoundedVec<T::AccountId, T::MaxAssignments>, ValueQuery>;

    /// List of decisions taken on the previous proposals indexed by proposal hash.
    #[pallet::storage]
    #[pallet::getter(fn decisions)]
    pub type Decisions<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, BoundedVec<(T::AccountId, Decision), T::MaxAssignments>, ValueQuery>;
    
    /// Events that functions in this pallet can emit.
    ///
    /// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
    /// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
    /// documentation for each event field and its parameters is added to a node's metadata so it
    /// can be used by external interfaces or tools.
    ///
    ///    The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emitted when a new consul is proposed to be added to the system.
        ConsulNominated { who: T::AccountId },
        /// Emitted when a new consul is added to the system.
        ConsulAdded { who: T::AccountId },
        /// Emitted when a consul is removed from the system.
        ConsulRemoved { who: T::AccountId },
        /// Emitted when a consul is assigned for the validation of a proposal.
        ConsulAssigned { who: T::AccountId, proposal: T::Hash },
        /// Emitted when a new patrician is proposed to be added to the system.
        PatricianNominated { who: T::AccountId },
        /// Emitted when a new patrician is added to the system.
        PatricianAdded { who: T::AccountId },
        /// Emitted when a patrician is removed from the system.
        PatricianRemoved { who: T::AccountId },
        /// Emitted when a validator begins the process of reviewing a block (after the block appears or
        /// is reassigned due to a timeout). There may be multiple validators analyzing the same block, 
        /// in which case this event is emitted for each of them.
        BlockProposed { who: T::AccountId, number: BlockNumberFor<T> },
        /// Emitted when the validator rejects the block, including a reason.
        BlockRejected { who: T::AccountId, number: BlockNumberFor<T>, reason: Option<BoundedVec<u8, T::MaxReasonLength>> },
        /// Emitted when the validator marks the block as potentially conflicting, including a reason and
        /// a request for more information.
        BlockNeedsInfo { who: T::AccountId, number: BlockNumberFor<T>, reason: Option<BoundedVec<u8, T::MaxReasonLength>> },
        /// Emitted when a consul approves a block after the validation process.
        BlockPreApproved { who: T::AccountId, number: BlockNumberFor<T> },
        /// Emitted when all validators assigned to a block have approved it.
        BlockApproved { who: T::AccountId, number: BlockNumberFor<T> },
        /// Emitted when the term of an elite member is renewed.
        TermRenewed { who: T::AccountId, term: T::Moment },
        /// Emitted when an elite member ceases to be one due to the expiration of his term.
        TermExpired { who: T::AccountId, term: T::Moment },
        /// **Emitted if a consul does not respond before the established deadline, allowing the reassignment 
        /// of a proposal or another decision.
        DeadlinePassed { who: T::AccountId },
        /// Emitted when a consul makes a decision on one of the pending proposals assigned to them.
        NewDecision { who: T::AccountId, proposal: T::Hash, decision: Decision },
    }

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        /// The value retrieved was `None` as no value was previously set.
        NoneValue,
        /// There was an attempt to increment the value in storage over `u32::MAX`.
        StorageOverflow,
        /// The account attempting to approve/reject a proposal is not a consul assigned to this proposal.
        NotAssignedConsul,
        /// The account attempting to act as a consul is not a valid consul. This error takes precedence over NotAssignedConsul.
        InvalidConsul,
        /// The account intended to be promoted to consul is already a consul.
        AlreadyConsul,
        /// The maximum number of consuls has been reached, and an attempt was made to add another.
        TooManyConsuls,
        /// The account attempting to propose a block is not a valid patrician.
        InvalidPatrician,
        /// The maximum number of patricians has been reached, and an attempt was made to add another.
        TooManyPatricians,
        /// At least one of the accounts proposed as a candidate is not valid (for example, it does not exist).
        InvalidCandidate,
        /// The block is either invalid or missing.
        InvalidBlock,
        /// The block is not in a valid state to be approved/rejected (for example, attempting to approve a block that has already been approved).
        InvalidBlockState,
        /// The proposal is either invalid or missing.
        InvalidProposal,
        /// The maximum number of proposals has been reached, and an attempt was made to add another.
        TooManyProposals,
        /// The consul did not respond before the established deadline.
        DeadlinePassed,
        /// The required signature is invalid or missing.
        InvalidSignature,
        /// The given string exceeds the maximum number of characters.
        StringTooLong,
    }

    /// The pallet's dispatchable functions ([`Call`]s).
    ///
    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// They must always return a `DispatchResult` and be annotated with a weight and call index.
    ///
    /// The [`call_index`] macro is used to explicitly
    /// define an index for calls in the [`Call`] enum. This is useful for pallets that may
    /// introduce new dispatchables over time. If the order of a dispatchable changes, its index
    /// will also change which will break backwards compatibility.
    ///
    /// The [`weight`] macro is used to assign a weight to each call.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Proposes the nomination of an account from the patrician group as a consul.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::propose_consul())]
        pub fn propose_consul(
            origin: OriginFor<T>,
            candidate: T::AccountId,
        ) -> DispatchResult {
            // Sanity checks.
            let proposer = ensure_signed(origin)?;
            ensure!(Consuls::<T>::get().contains(&proposer), Error::<T>::InvalidConsul);
            ensure!(Patricians::<T>::get().contains(&candidate), Error::<T>::InvalidPatrician);
            ensure!(!Consuls::<T>::get().contains(&candidate), Error::<T>::AlreadyConsul);

            // Calculates the deadline.
            let current_time = <pallet_timestamp::Pallet<T>>::get();
            let expiry = current_time + T::ProposalDuration::get().into();

            // Creates proposal and calculates hash.
            let proposal_without_hash = Proposal::<T>::ConsulNomination(Default::default(), candidate.clone(), expiry);
            let proposal_hash = T::Hashing::hash_of(&(proposer.clone(), &proposal_without_hash));

            // Gets the current proposals and checks the limit.
            let mut proposals = Proposals::<T>::get();
            ensure!(
                proposals.len() < T::MaxProposals::get() as usize,
                Error::<T>::TooManyProposals
            );

            // Adds the new proposal to the BoundedVec.
            let proposal = Proposal::<T>::ConsulNomination(proposal_hash, candidate.clone(), expiry);
            proposals
                .try_push((proposer.clone(), proposal))
                .map_err(|_| Error::<T>::TooManyProposals)?;

            // Updates the storage with the modified BoundedVec.
            Proposals::<T>::put(proposals);

            Self::deposit_event(Event::ConsulNominated { who: candidate });

            Ok(())
        }
    }
}
