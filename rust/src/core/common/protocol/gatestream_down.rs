use crate::common::types::{ArbCmd, Cycles, Gate, QubitRef, SequenceNumber};
use serde::{Deserialize, Serialize};

/// Gatestream requests/downstream messages.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GatestreamDown {
    /// A pipelined gatestream request.
    ///
    /// By pipelined we mean that the response to the request is NOT
    /// immediately waited upon. In fact, not every request necessarily even
    /// needs a response.
    ///
    /// To associate requests with responses a sequence number system is used.
    /// The sequence numbers are generated by a counter on the upstream side
    /// such that they are monotonously increasing. The sequence number is
    /// echoed in the response, such that when the contents of a response are
    /// needed, it can be waited upon. Other than that, the downstream plugin
    /// should not do anything with the sequence number.
    Pipelined(SequenceNumber, PipelinedGatestreamDown),

    /// Requests execution of the given `ArbCmd` by the plugin.
    ///
    /// The valid responses to this message are:
    ///
    ///  - success: `GatestreamUp::ArbSuccess`
    ///  - failure: `GatestreamUp::ArbFailure`
    ArbRequest(ArbCmd),
}

/// Enumeration of pipelined gatestream messages.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PipelinedGatestreamDown {
    /// Requests allocation of a number of qubits.
    ///
    /// The `ArbCmd` vector allows commands to be associated with a qubit
    /// register. This can for instance be used to customize error parameters
    /// for a specific qubit or qubit register.
    Allocate(usize, Vec<ArbCmd>),

    /// Requests that one or more qubits be freed.
    Free(Vec<QubitRef>),

    /// Requests execution of a gate.
    Gate(Gate),

    /// Advances the simulation by the specified number of cycles.
    Advance(Cycles),
}
