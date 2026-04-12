use super::{MirFunction, MirModule};
use std::fmt;

impl fmt::Display for MirFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "function {}({}) -> {} {{",
            self.signature.name,
            self.signature
                .params
                .iter()
                .enumerate()
                .map(|(i, ty)| format!("%{}: {:?}", i, ty))
                .collect::<Vec<_>>()
                .join(", "),
            format!("{:?}", self.signature.return_type)
        )?;

        // Show effects if not pure
        if !self.signature.effects.is_pure() {
            writeln!(f, "  ; effects: {}", self.signature.effects)?;
        }

        // Show blocks in order
        let mut block_ids: Vec<_> = self.blocks.keys().copied().collect();
        block_ids.sort();

        for block_id in block_ids {
            if let Some(block) = self.blocks.get(&block_id) {
                write!(f, "{}", block)?;
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for MirModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module {} {{", self.name)?;

        // Show globals
        if !self.globals.is_empty() {
            writeln!(f, "  ; globals:")?;
            for (name, value) in &self.globals {
                writeln!(f, "  global {} = {}", name, value)?;
            }
            writeln!(f)?;
        }

        // Show functions
        for function in self.functions.values() {
            writeln!(f, "{}", function)?;
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}
