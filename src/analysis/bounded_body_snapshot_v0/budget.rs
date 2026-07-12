use super::SnapshotLimitsV0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetLimitV0 {
    Depth,
    NodeCount,
    ChildrenPerBody,
    Arguments,
    LiteralBytes,
    AtomBytes,
    TotalTextBytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedBodyBudgetV0 {
    limits: SnapshotLimitsV0,
    node_count: usize,
    total_text_bytes: usize,
    max_depth_observed: usize,
}

impl Default for BoundedBodyBudgetV0 {
    fn default() -> Self {
        Self::new(SnapshotLimitsV0::SCHEMA)
    }
}

impl BoundedBodyBudgetV0 {
    pub fn new(limits: SnapshotLimitsV0) -> Self {
        Self {
            limits,
            node_count: 0,
            total_text_bytes: 0,
            max_depth_observed: 0,
        }
    }

    pub fn observe_node(&mut self, depth: usize) -> Result<(), BudgetLimitV0> {
        if depth > self.limits.max_depth {
            return Err(BudgetLimitV0::Depth);
        }
        if self.node_count >= self.limits.max_node_count {
            return Err(BudgetLimitV0::NodeCount);
        }
        self.node_count += 1;
        self.max_depth_observed = self.max_depth_observed.max(depth);
        Ok(())
    }

    pub fn observe_body_children(&self, count: usize) -> Result<(), BudgetLimitV0> {
        (count <= self.limits.max_children_per_body)
            .then_some(())
            .ok_or(BudgetLimitV0::ChildrenPerBody)
    }

    pub fn observe_arguments(&self, count: usize) -> Result<(), BudgetLimitV0> {
        (count <= self.limits.max_arguments)
            .then_some(())
            .ok_or(BudgetLimitV0::Arguments)
    }

    pub fn observe_literal(&mut self, value: &str) -> Result<(), BudgetLimitV0> {
        let bytes = value.len();
        if bytes > self.limits.max_literal_bytes {
            return Err(BudgetLimitV0::LiteralBytes);
        }
        self.observe_atom_bytes(bytes)
    }

    pub fn observe_atom(&mut self, value: &str) -> Result<(), BudgetLimitV0> {
        let bytes = value.len();
        if bytes > self.limits.max_atom_bytes {
            return Err(BudgetLimitV0::AtomBytes);
        }
        self.observe_atom_bytes(bytes)
    }

    fn observe_atom_bytes(&mut self, bytes: usize) -> Result<(), BudgetLimitV0> {
        let next = self
            .total_text_bytes
            .checked_add(bytes)
            .ok_or(BudgetLimitV0::TotalTextBytes)?;
        if next > self.limits.max_total_text_bytes {
            return Err(BudgetLimitV0::TotalTextBytes);
        }
        self.total_text_bytes = next;
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn max_depth_observed(&self) -> usize {
        self.max_depth_observed
    }
}
