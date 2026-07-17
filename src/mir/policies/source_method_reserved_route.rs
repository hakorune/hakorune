//! Pure reserved-route policy for source `MethodCall` receivers.
//!
//! The caller supplies an explicit context. This module never reads Builder
//! state, lexical/import/catalog facts, or target/runtime metadata.

use crate::ast::{ASTNode, LiteralValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceMethodReservedRouteContextV1 {
    Ordinary,
    FastMemBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MirDebugMethodV1 {
    Log,
    Mark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReplIntrinsicMethodV1 {
    Get,
    Set,
}

impl ReplIntrinsicMethodV1 {
    pub(crate) const fn spelling(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Set => "set",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceMethodReservedRouteFailureV1 {
    MirDebugLabelRequired,
    UnsupportedReplMethod,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceMethodReservedRouteDecisionV1 {
    Ordinary,
    FastMem,
    MirDebug {
        method: MirDebugMethodV1,
        label: Box<str>,
    },
    ReplIntrinsic {
        method: ReplIntrinsicMethodV1,
    },
    ReservedFail(SourceMethodReservedRouteFailureV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceMethodReservedRouteDispositionV1 {
    Ordinary,
    FastMem,
    MirDebug,
    ReplIntrinsic,
    ReservedFail(SourceMethodReservedRouteFailureV1),
}

impl SourceMethodReservedRouteDecisionV1 {
    pub(crate) const fn disposition(&self) -> SourceMethodReservedRouteDispositionV1 {
        match self {
            Self::Ordinary => SourceMethodReservedRouteDispositionV1::Ordinary,
            Self::FastMem => SourceMethodReservedRouteDispositionV1::FastMem,
            Self::MirDebug { .. } => SourceMethodReservedRouteDispositionV1::MirDebug,
            Self::ReplIntrinsic { .. } => SourceMethodReservedRouteDispositionV1::ReplIntrinsic,
            Self::ReservedFail(reason) => {
                SourceMethodReservedRouteDispositionV1::ReservedFail(*reason)
            }
        }
    }
}

pub(crate) fn classify_source_method_reserved_route_v1(
    context: SourceMethodReservedRouteContextV1,
    receiver: &ASTNode,
    method: &str,
    arguments: &[ASTNode],
) -> SourceMethodReservedRouteDecisionV1 {
    let ASTNode::Variable { name: receiver, .. } = receiver else {
        return SourceMethodReservedRouteDecisionV1::Ordinary;
    };

    if context == SourceMethodReservedRouteContextV1::FastMemBody && receiver == "mem" {
        return SourceMethodReservedRouteDecisionV1::FastMem;
    }

    if receiver == "__mir__" {
        let method = match method {
            "log" => MirDebugMethodV1::Log,
            "mark" => MirDebugMethodV1::Mark,
            _ => return SourceMethodReservedRouteDecisionV1::Ordinary,
        };
        let Some(first) = arguments.first() else {
            return SourceMethodReservedRouteDecisionV1::ReservedFail(
                SourceMethodReservedRouteFailureV1::MirDebugLabelRequired,
            );
        };
        let ASTNode::Literal {
            value: LiteralValue::String(label),
            ..
        } = first
        else {
            return SourceMethodReservedRouteDecisionV1::Ordinary;
        };
        return SourceMethodReservedRouteDecisionV1::MirDebug {
            method,
            label: label.clone().into_boxed_str(),
        };
    }

    if receiver == "__repl" {
        let method = match method {
            "get" => ReplIntrinsicMethodV1::Get,
            "set" => ReplIntrinsicMethodV1::Set,
            _ => {
                return SourceMethodReservedRouteDecisionV1::ReservedFail(
                    SourceMethodReservedRouteFailureV1::UnsupportedReplMethod,
                )
            }
        };
        return SourceMethodReservedRouteDecisionV1::ReplIntrinsic { method };
    }

    SourceMethodReservedRouteDecisionV1::Ordinary
}
