#![feature(negate_unsigned)]

use std::ops::{ Div, Not, Shl, Shr };
use std::path::{ Path, PathBuf };

/// Path use div.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PathDiv {
    inner: PathBuf
}

impl PathDiv {
    pub fn new<P: AsRef<Path>>(p: P) -> PathDiv {
        PathDiv { inner: p.as_ref().to_path_buf() }
    }

    /// Get sub PathDiv.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(negate_unsigned)]
    /// use pathlib::PathDiv;
    ///
    /// let home = PathDiv::new("/home/quininer");
    /// assert_eq!(home.get(1, true), Some(PathDiv::new("home")));
    /// assert_eq!(home.get(0, false), Some(PathDiv::new("quininer")));
    /// ```
    pub fn get(&self, i: usize, rl: bool) -> Option<PathDiv> {
        let it = self.path().iter();
        it.clone()
            .nth(if rl { i } else { it.count() - i -1 })
            .map(PathDiv::new)
    }

    pub fn path(&self) -> &Path {
        self.inner.as_path()
    }

    pub fn to_str(&self) -> Option<&str> {
        self.path().to_str()
    }
}

impl AsRef<str> for PathDiv {
    fn as_ref(&self) -> &str {
        self.to_str().unwrap_or("")
    }
}

impl AsRef<Path> for PathDiv {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

/// Join PathDiv.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use pathlib::PathDiv;
///
/// let root = PathDiv::new("/");
/// let home = root / "home" / "quininer";
/// assert_eq!(home.path(), Path::new("/home/quininer"));
/// ```
impl<T> Div<T> for PathDiv where T: AsRef<Path> {
    type Output = PathDiv;

    fn div(self, rhs: T) -> PathDiv {
        PathDiv::new(self.inner.join(rhs))
    }
}

/// Returns the parent path, and if does not exist,
/// and now the path is returned.
///
/// # Examples
///
/// ```
/// use pathlib::PathDiv;
///
/// let home = PathDiv::new("/home/quininer");
/// assert_eq!(!home.clone(), PathDiv::new("/home"));
/// assert_eq!(!!!home.clone(), PathDiv::new("/"));
/// ```
impl Not for PathDiv {
    type Output = PathDiv;

    fn not(self) -> PathDiv {
        match self.path().parent() {
            Some(path) => PathDiv::new(path),
            None => self.clone()
        }
    }
}

/// Get the N-th PathDiv. From left to right.
///
/// # Examples
///
/// ```
/// use pathlib::PathDiv;
///
/// let home = PathDiv::new("/home/quininer");
/// assert_eq!(home >> 1, PathDiv::new("home"));
/// ```
impl Shr<usize> for PathDiv {
    type Output = PathDiv;

    fn shr(self, i: usize) -> PathDiv {
        self.get(i, true).unwrap_or(PathDiv::new(""))
    }
}

/// Get the N-th PathDiv. From right to left.
///
/// # Examples
///
/// ```
/// use pathlib::PathDiv;
///
/// let home = PathDiv::new("/home/quininer");
/// assert_eq!(home << 0, PathDiv::new("quininer"));
/// ```
impl Shl<usize> for PathDiv {
    type Output = PathDiv;

    fn shl(self, i: usize) -> PathDiv {
        self.get(i, false).unwrap_or(PathDiv::new(""))
    }
}
