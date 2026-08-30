//! Shared source/physical arity contract for instance constructors.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InstanceConstructorAbiV1 {
    source_arity: usize,
    physical_arity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InstanceConstructorAbiErrorV1 {
    PhysicalArityOverflow,
    SourceArityMismatch { expected: usize, actual: usize },
    PhysicalArityMismatch { expected: usize, actual: usize },
}

impl InstanceConstructorAbiV1 {
    pub(crate) fn issue(source_arity: usize) -> Result<Self, InstanceConstructorAbiErrorV1> {
        let physical_arity = source_arity
            .checked_add(1)
            .ok_or(InstanceConstructorAbiErrorV1::PhysicalArityOverflow)?;
        Ok(Self {
            source_arity,
            physical_arity,
        })
    }

    pub(crate) const fn source_arity(self) -> usize {
        self.source_arity
    }

    pub(crate) const fn physical_arity(self) -> usize {
        self.physical_arity
    }

    pub(crate) fn validate(
        self,
        source_arity: usize,
        physical_arity: usize,
    ) -> Result<(), InstanceConstructorAbiErrorV1> {
        if source_arity != self.source_arity {
            return Err(InstanceConstructorAbiErrorV1::SourceArityMismatch {
                expected: self.source_arity,
                actual: source_arity,
            });
        }
        if physical_arity != self.physical_arity {
            return Err(InstanceConstructorAbiErrorV1::PhysicalArityMismatch {
                expected: self.physical_arity,
                actual: physical_arity,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_constructor_abi_checks_n_plus_one() {
        let abi = InstanceConstructorAbiV1::issue(2).expect("N+1 ABI");
        assert_eq!(abi.source_arity(), 2);
        assert_eq!(abi.physical_arity(), 3);
        abi.validate(2, 3).expect("matching arities");
        assert!(matches!(
            abi.validate(1, 3),
            Err(InstanceConstructorAbiErrorV1::SourceArityMismatch { .. })
        ));
        assert!(matches!(
            abi.validate(2, 2),
            Err(InstanceConstructorAbiErrorV1::PhysicalArityMismatch { .. })
        ));
    }
}
