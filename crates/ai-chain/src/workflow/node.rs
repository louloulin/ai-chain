
use ai_chain_types::serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use async_trait::async_trait;
use ai_chain_types::errors::internal::BoxedError;
use ai_chain_types::node::OpIdentifier;
use ai_chain_types::types::Schema;

pub type PortHandle = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "ai_chain_types::serde")]
pub enum OutputPortType {
    Stateless,
    StatefulWithPrimaryKeyLookup,
}

impl Display for OutputPortType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputPortType::Stateless => f.write_str("Stateless"),
            OutputPortType::StatefulWithPrimaryKeyLookup { .. } => {
                f.write_str("StatefulWithPrimaryKeyLookup")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputPortDef {
    pub handle: PortHandle,
    pub typ: OutputPortType,
}

impl OutputPortDef {
    pub fn new(handle: PortHandle, typ: OutputPortType) -> Self {
        Self { handle, typ }
    }
}


