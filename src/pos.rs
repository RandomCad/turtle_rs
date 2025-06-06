use std::{fmt::Display, num::ParseIntError, str::FromStr};

/// A position in a file.
///
/// Used when an error is found while compiling to tell the developer where to fix his code
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FilePos {
    pub line: usize,
    pub column: usize,
}

impl FilePos {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn is_empty(&self) -> bool {
        self.column == 0 && self.line == 0
    }
}

impl Display for FilePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FilePosParseErr {
    #[error("no delimiter")]
    NoDelimiter,
    #[error("{0}")]
    ParseError(#[from] ParseIntError),
}

impl FromStr for FilePos {
    type Err = FilePosParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, column) = s
            .split_once(|c: char| !c.is_ascii_digit())
            .ok_or(FilePosParseErr::NoDelimiter)?;
        Ok(Self::new(line.parse()?, column.parse()?))
    }
}

/// Attach [`FilePos`] to any type `T`, mostly tokens
///
/// Implements [`Deref`](std::ops::Deref) to access inner value
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos<T> {
    pos: FilePos,
    token: T,
}

impl<T> Pos<T> {
    /// Create new [`Pos`] wrapper.
    pub fn new(token: T, pos: FilePos) -> Self {
        Self { pos, token }
    }

    /// Get attached [`FilePos`]
    pub fn get_pos(&self) -> FilePos {
        self.pos
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Pos<U> {
        let Self { pos, token } = self;
        Pos {
            pos,
            token: f(token),
        }
    }

    pub fn into_inner(self) -> T {
        self.token
    }
}

impl<T> std::ops::Deref for Pos<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl<T> std::ops::DerefMut for Pos<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token
    }
}

pub trait Positionable: Sized {
    fn attach_pos(self, pos: FilePos) -> Pos<Self>;
}

impl<T> Positionable for T {
    fn attach_pos(self, pos: FilePos) -> Pos<Self> {
        Pos::new(self, pos)
    }
}
