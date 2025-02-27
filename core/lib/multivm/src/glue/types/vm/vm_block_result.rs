use crate::glue::{GlueFrom, GlueInto};
use crate::interface::{
    CurrentExecutionState, ExecutionResult, Refunds, VmExecutionResultAndLogs,
    VmExecutionStatistics,
};
use zksync_types::tx::tx_execution_info::VmExecutionLogs;

// Note: In version after vm VmVirtualBlocks the bootloader memory knowledge is encapsulated into the VM.
// But before it was a part of a public API.
// Bootloader memory required only for producing witnesses,
// and server doesn't need to generate witnesses for old blocks

impl GlueFrom<crate::vm_m5::vm::VmBlockResult> for crate::interface::FinishedL1Batch {
    fn glue_from(value: crate::vm_m5::vm::VmBlockResult) -> Self {
        crate::interface::FinishedL1Batch {
            block_tip_execution_result: VmExecutionResultAndLogs {
                result: value.block_tip_result.revert_reason.glue_into(),
                logs: VmExecutionLogs {
                    events: value.block_tip_result.logs.events.clone(),
                    l2_to_l1_logs: value.block_tip_result.logs.l2_to_l1_logs.clone(),
                    storage_logs: value.block_tip_result.logs.storage_logs.clone(),
                    total_log_queries_count: value.block_tip_result.logs.total_log_queries_count,
                },
                statistics: VmExecutionStatistics {
                    contracts_used: value.block_tip_result.contracts_used,
                    cycles_used: value.block_tip_result.cycles_used,
                    total_log_queries: value.block_tip_result.logs.total_log_queries_count,
                    computational_gas_used: value.full_result.gas_used,
                    gas_used: value.full_result.gas_used,
                },
                refunds: Refunds::default(),
            },
            final_execution_state: CurrentExecutionState {
                events: value.full_result.events,
                storage_log_queries: value.full_result.storage_log_queries,
                used_contract_hashes: value.full_result.used_contract_hashes,
                l2_to_l1_logs: value.full_result.l2_to_l1_logs,
                total_log_queries: value.full_result.total_log_queries,
                cycles_used: value.full_result.cycles_used,
                storage_refunds: Vec::new(),
            },
            final_bootloader_memory: None,
        }
    }
}

impl GlueFrom<crate::vm_m6::vm::VmBlockResult> for crate::interface::FinishedL1Batch {
    fn glue_from(value: crate::vm_m6::vm::VmBlockResult) -> Self {
        crate::interface::FinishedL1Batch {
            block_tip_execution_result: VmExecutionResultAndLogs {
                result: value.block_tip_result.revert_reason.glue_into(),
                logs: VmExecutionLogs {
                    events: value.block_tip_result.logs.events,
                    l2_to_l1_logs: value.block_tip_result.logs.l2_to_l1_logs,
                    storage_logs: value.block_tip_result.logs.storage_logs,
                    total_log_queries_count: value.block_tip_result.logs.total_log_queries_count,
                },
                statistics: VmExecutionStatistics {
                    contracts_used: value.block_tip_result.contracts_used,
                    cycles_used: value.block_tip_result.cycles_used,
                    total_log_queries: value.block_tip_result.logs.total_log_queries_count,
                    computational_gas_used: value.full_result.computational_gas_used,
                    gas_used: value.full_result.gas_used,
                },
                refunds: Refunds::default(),
            },
            final_execution_state: CurrentExecutionState {
                events: value.full_result.events,
                storage_log_queries: value.full_result.storage_log_queries,
                used_contract_hashes: value.full_result.used_contract_hashes,
                l2_to_l1_logs: value.full_result.l2_to_l1_logs,
                total_log_queries: value.full_result.total_log_queries,
                cycles_used: value.full_result.cycles_used,
                storage_refunds: Vec::new(),
            },
            final_bootloader_memory: None,
        }
    }
}

impl GlueFrom<crate::vm_1_3_2::vm::VmBlockResult> for crate::interface::FinishedL1Batch {
    fn glue_from(value: crate::vm_1_3_2::vm::VmBlockResult) -> Self {
        crate::interface::FinishedL1Batch {
            block_tip_execution_result: VmExecutionResultAndLogs {
                result: value.block_tip_result.revert_reason.glue_into(),
                logs: VmExecutionLogs {
                    events: value.block_tip_result.logs.events,
                    l2_to_l1_logs: value.block_tip_result.logs.l2_to_l1_logs,
                    storage_logs: value.block_tip_result.logs.storage_logs,
                    total_log_queries_count: value.block_tip_result.logs.total_log_queries_count,
                },
                statistics: VmExecutionStatistics {
                    contracts_used: value.block_tip_result.contracts_used,
                    cycles_used: value.block_tip_result.cycles_used,
                    total_log_queries: value.block_tip_result.logs.total_log_queries_count,
                    computational_gas_used: value.full_result.computational_gas_used,
                    gas_used: value.full_result.gas_used,
                },
                refunds: Refunds::default(),
            },
            final_execution_state: CurrentExecutionState {
                events: value.full_result.events,
                storage_log_queries: value.full_result.storage_log_queries,
                used_contract_hashes: value.full_result.used_contract_hashes,
                l2_to_l1_logs: value.full_result.l2_to_l1_logs,
                total_log_queries: value.full_result.total_log_queries,
                cycles_used: value.full_result.cycles_used,
                storage_refunds: Vec::new(),
            },
            final_bootloader_memory: None,
        }
    }
}

impl GlueFrom<crate::vm_1_3_2::vm::VmBlockResult> for crate::interface::VmExecutionResultAndLogs {
    fn glue_from(value: crate::vm_1_3_2::vm::VmBlockResult) -> Self {
        let mut result = value
            .full_result
            .revert_reason
            .map(|a| a.revert_reason)
            .glue_into();

        if let ExecutionResult::Success { output } = &mut result {
            *output = value.full_result.return_data;
        }

        VmExecutionResultAndLogs {
            result,
            logs: VmExecutionLogs {
                events: value.full_result.events,
                l2_to_l1_logs: value.full_result.l2_to_l1_logs,
                storage_logs: value.full_result.storage_log_queries,
                total_log_queries_count: value.full_result.total_log_queries,
            },
            statistics: VmExecutionStatistics {
                contracts_used: value.full_result.contracts_used,
                cycles_used: value.full_result.cycles_used,
                total_log_queries: value.full_result.total_log_queries,
                computational_gas_used: value.full_result.computational_gas_used,
                gas_used: value.full_result.gas_used,
            },
            refunds: Refunds::default(),
        }
    }
}

impl GlueFrom<crate::vm_m5::vm::VmBlockResult> for crate::interface::VmExecutionResultAndLogs {
    fn glue_from(value: crate::vm_m5::vm::VmBlockResult) -> Self {
        let mut result = value
            .full_result
            .revert_reason
            .map(|a| a.revert_reason)
            .glue_into();

        if let ExecutionResult::Success { output } = &mut result {
            *output = value.full_result.return_data;
        }

        VmExecutionResultAndLogs {
            result,
            logs: VmExecutionLogs {
                events: value.full_result.events,
                l2_to_l1_logs: value.full_result.l2_to_l1_logs,
                storage_logs: value.full_result.storage_log_queries,
                total_log_queries_count: value.full_result.total_log_queries,
            },
            statistics: VmExecutionStatistics {
                contracts_used: value.full_result.contracts_used,
                cycles_used: value.full_result.cycles_used,
                total_log_queries: value.full_result.total_log_queries,
                computational_gas_used: 0,
                gas_used: value.full_result.gas_used,
            },
            refunds: Refunds::default(),
        }
    }
}

impl GlueFrom<crate::vm_m6::vm::VmBlockResult> for crate::interface::VmExecutionResultAndLogs {
    fn glue_from(value: crate::vm_m6::vm::VmBlockResult) -> Self {
        let mut result = value
            .full_result
            .revert_reason
            .map(|a| a.revert_reason)
            .glue_into();

        if let ExecutionResult::Success { output } = &mut result {
            *output = value.full_result.return_data;
        }

        VmExecutionResultAndLogs {
            result,
            logs: VmExecutionLogs {
                events: value.full_result.events,
                l2_to_l1_logs: value.full_result.l2_to_l1_logs,
                storage_logs: value.full_result.storage_log_queries,
                total_log_queries_count: value.full_result.total_log_queries,
            },
            statistics: VmExecutionStatistics {
                contracts_used: value.full_result.contracts_used,
                cycles_used: value.full_result.cycles_used,
                total_log_queries: value.full_result.total_log_queries,
                computational_gas_used: value.full_result.computational_gas_used,
                gas_used: value.full_result.gas_used,
            },
            refunds: Refunds::default(),
        }
    }
}
