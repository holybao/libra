// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::Executor;
use anyhow::Result;
use executor_types::ChunkExecutor;
use libra_crypto::HashValue;
use libra_state_view::StateView;
use libra_types::{
    account_address::AccountAddress,
    account_state_blob::{AccountStateBlob, AccountStateWithProof},
    contract_event::ContractEvent,
    epoch_change::EpochChangeProof,
    event::EventKey,
    ledger_info::LedgerInfoWithSignatures,
    proof::{AccumulatorConsistencyProof, SparseMerkleProof},
    transaction::{
        Transaction, TransactionListWithProof, TransactionOutput, TransactionToCommit,
        TransactionWithProof, Version,
    },
    vm_status::VMStatus,
};
use libra_vm::VMExecutor;
use proptest::{
    arbitrary::Arbitrary,
    strategy::{Strategy, ValueTree},
    test_runner::{self, TestRunner},
};
use storage_interface::{DbReader, DbReaderWriter, DbWriter, Order, StartupInfo, TreeState};

pub fn fuzz(data: &[u8]) {
    // setup fake db
    let fake_db = FakeDb {};
    let db_reader_writer = DbReaderWriter::new(fake_db);
    let mut executor = Executor::<FakeVM>::new(db_reader_writer);

    // construct arguments
    let passthrough_rng =
        test_runner::TestRng::from_seed(test_runner::RngAlgorithm::PassThrough, data);
    let config = test_runner::Config::default();
    let mut runner = TestRunner::new_with_rng(config, passthrough_rng);

    // txn_list_with_proof
    let strategy = TransactionListWithProof::arbitrary();
    let strategy_tree = match strategy.new_tree(&mut runner) {
        Ok(x) => x,
        Err(_) => return,
    };
    let txn_list_with_proof = strategy_tree.current();

    // verified_target_li
    let strategy = LedgerInfoWithSignatures::arbitrary();
    let strategy_tree = match strategy.new_tree(&mut runner) {
        Ok(x) => x,
        Err(_) => return,
    };
    let verified_target_li = strategy_tree.current();

    //
    let _events = executor.execute_and_commit_chunk(txn_list_with_proof, verified_target_li, None);
}

/// A fake VM implementing VMExecutor
pub struct FakeVM;

impl VMExecutor for FakeVM {
    fn execute_block(
        _transactions: Vec<Transaction>,
        _state_view: &dyn StateView,
    ) -> Result<Vec<TransactionOutput>, VMStatus> {
        Ok(Vec::new())
    }
}

/// A fake database implementing DbReader and DbWriter
pub struct FakeDb;

impl DbReader for FakeDb {
    fn get_block_timestamp(&self, _version: u64) -> Result<u64> {
        unimplemented!();
    }

    fn get_epoch_ending_ledger_infos(
        &self,
        _start_epoch: u64,
        _end_epoch: u64,
    ) -> Result<EpochChangeProof> {
        unimplemented!();
    }

    fn get_transactions(
        &self,
        _start_version: Version,
        _batch_size: u64,
        _ledger_version: Version,
        _fetch_events: bool,
    ) -> Result<TransactionListWithProof> {
        unimplemented!();
    }

    fn get_events(
        &self,
        _event_key: &EventKey,
        _start: u64,
        _order: Order,
        _limit: u64,
    ) -> Result<Vec<(u64, ContractEvent)>> {
        unimplemented!();
    }

    fn get_latest_account_state(
        &self,
        _address: AccountAddress,
    ) -> Result<Option<AccountStateBlob>> {
        unimplemented!();
    }

    fn get_latest_ledger_info(&self) -> Result<LedgerInfoWithSignatures> {
        unimplemented!();
    }

    fn get_latest_version(&self) -> Result<Version> {
        Ok(self.get_latest_ledger_info()?.ledger_info().version())
    }

    fn get_latest_commit_metadata(&self) -> Result<(Version, u64)> {
        let ledger_info_with_sig = self.get_latest_ledger_info()?;
        let ledger_info = ledger_info_with_sig.ledger_info();
        Ok((ledger_info.version(), ledger_info.timestamp_usecs()))
    }

    fn get_startup_info(&self) -> Result<Option<StartupInfo>> {
        Ok(Some(StartupInfo::new_for_testing()))
    }

    fn get_txn_by_account(
        &self,
        _address: AccountAddress,
        _seq_num: u64,
        _ledger_version: Version,
        _fetch_events: bool,
    ) -> Result<Option<TransactionWithProof>> {
        unimplemented!();
    }

    fn get_state_proof_with_ledger_info(
        &self,
        _known_version: u64,
        _ledger_info: LedgerInfoWithSignatures,
    ) -> Result<(EpochChangeProof, AccumulatorConsistencyProof)> {
        unimplemented!();
    }

    fn get_state_proof(
        &self,
        _known_version: u64,
    ) -> Result<(
        LedgerInfoWithSignatures,
        EpochChangeProof,
        AccumulatorConsistencyProof,
    )> {
        unimplemented!();
    }

    fn get_account_state_with_proof(
        &self,
        _address: AccountAddress,
        _version: Version,
        _ledger_version: Version,
    ) -> Result<AccountStateWithProof> {
        unimplemented!();
    }

    fn get_account_state_with_proof_by_version(
        &self,
        _address: AccountAddress,
        _version: Version,
    ) -> Result<(Option<AccountStateBlob>, SparseMerkleProof)> {
        unimplemented!();
    }

    fn get_latest_state_root(&self) -> Result<(Version, HashValue)> {
        unimplemented!();
    }

    fn get_latest_tree_state(&self) -> Result<TreeState> {
        unimplemented!();
    }

    fn get_epoch_ending_ledger_info(
        &self,
        _known_version: u64,
    ) -> Result<LedgerInfoWithSignatures> {
        unimplemented!();
    }
}

impl DbWriter for FakeDb {
    fn save_transactions(
        &self,
        _txns_to_commit: &[TransactionToCommit],
        _first_version: Version,
        _ledger_info_with_sigs: Option<&LedgerInfoWithSignatures>,
    ) -> Result<()> {
        Ok(())
    }
}
