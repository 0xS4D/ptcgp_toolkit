#![allow(dead_code)]
pub mod extractor;
mod circular;
pub mod field;
pub mod map;
pub mod message;
pub mod one_of;
pub mod package;
pub mod proto_enum;
pub mod schema;
pub mod service;
pub mod writer;

use crate::proto::message::ProtoMessage;
use crate::proto::proto_enum::ProtoEnum;
use crate::proto::service::ProtoService;

#[derive(PartialEq)]
pub enum ProtoType {
    Enum(ProtoEnum),
    Message(ProtoMessage),
    Service(ProtoService),
}
