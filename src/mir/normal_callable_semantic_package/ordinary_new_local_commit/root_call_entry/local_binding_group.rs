//! The existing source-ordered binding group retains its original physical proof.
//! Sharing a prefix between exits shares the packet, never its affine row.
use super::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct RootLocalCallBindingGroupV1 {
    site: OwnedExprSiteV1,
    bindings: Vec<(BasicBlockId, MirInstruction)>,
    lexical: Option<Rc<EmittedLexicalCallProjectionV1>>,
}

impl RootLocalCallBindingGroupV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn new(
        site: OwnedExprSiteV1,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        lexical: Option<Rc<EmittedLexicalCallProjectionV1>>,
    ) -> Result<Self, String> {
        if bindings.is_empty() {
            return Err(freeze("local-call-bindings-empty"));
        }
        if let Some(packet) = &lexical {
            if packet.call_site() != &site {
                return Err(freeze("local-call-proof-site"));
            }
            packet.validate_recorded(&bindings)?;
        }
        Ok(Self {
            site,
            bindings,
            lexical,
        })
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        &self.bindings
    }
    pub(in crate::mir) fn lexical(&self) -> Option<&EmittedLexicalCallProjectionV1> {
        self.lexical.as_deref()
    }

    // Only flat bindings are rebound. The shared original packet remains
    // immutable even when several exits cover the same source prefix.
    pub(in crate::mir::normal_callable_semantic_package) fn with_bindings(
        &self,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Self {
        Self {
            site: self.site.clone(),
            bindings,
            lexical: self.lexical.clone(),
        }
    }
}
