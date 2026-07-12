use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathSegmentV0 {
    Field(&'static str),
    Index(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathV0 {
    segments: Vec<PathSegmentV0>,
}

impl PathV0 {
    pub fn root_body() -> Self {
        Self {
            segments: vec![PathSegmentV0::Field("body")],
        }
    }

    pub fn field(&self, field: &'static str) -> Self {
        let mut next = self.clone();
        next.segments.push(PathSegmentV0::Field(field));
        next
    }

    pub fn index(&self, index: usize) -> Self {
        let mut next = self.clone();
        next.segments.push(PathSegmentV0::Index(index));
        next
    }

    pub fn segments(&self) -> &[PathSegmentV0] {
        &self.segments
    }
}

impl fmt::Display for PathV0 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("$")?;
        for segment in &self.segments {
            match segment {
                PathSegmentV0::Field(field) => write!(formatter, ".{field}")?,
                PathSegmentV0::Index(index) => write!(formatter, "[{index}]")?,
            }
        }
        Ok(())
    }
}
