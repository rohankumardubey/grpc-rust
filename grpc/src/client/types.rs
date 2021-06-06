use crate::client::req_sink::ClientRequestSinkUntyped;
use crate::common::types::Types;

pub(crate) struct ClientTypes;

impl Types for ClientTypes {
    type SinkUntyped = ClientRequestSinkUntyped;
}

unsafe impl Sync for ClientTypes {}
