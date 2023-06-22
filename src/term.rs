use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum Term {
    /// ```rust
    /// assert_eq!(
    ///     erlpack::Term::from_bytes(&[131, 97, 14]).unwrap(),
    ///     erlpack::Term::Integer(num_bigint::BigInt::from(14))
    /// );
    /// ```
    Integer(BigInt),

    /// ```rust
    /// assert_eq!(
    ///     erlpack::Term::from_bytes(&[131, 70, 64, 43, 5, 30, 184, 81, 235, 133]).unwrap(),
    ///     erlpack::Term::Float(13.51)
    /// );
    /// ```
    Float(f64),

    /// An atom is a constant with its value equal to its name
    /// Erlang booleans are the atoms `true` and `false
    /// ```rust
    /// assert_eq!(
    ///     erlpack::Term::from_bytes(&[131, 100, 0, 4, 116, 114, 117, 101]).unwrap(),
    ///     erlpack::Term::Atom("true".to_string())
    /// );
    /// ```
    Atom(String),

    /// ```rust
    /// assert_eq!(
    ///     erlpack::Term::from_bytes(&[131, 104, 2, 97, 1, 97, 2]).unwrap(),
    ///     erlpack::Term::Tuple(vec![
    ///         erlpack::Term::Integer(num_bigint::BigInt::from(1)),
    ///         erlpack::Term::Integer(num_bigint::BigInt::from(2))
    ///     ])
    /// );
    /// ```
    Tuple(Vec<Term>),

    /// ```rust
    /// # use erlpack::Term;
    /// assert_eq!(
    ///     Term::from_bytes(&[131, 116, 0, 0, 0, 2, 100, 0, 1, 97, 100, 0, 1, 65, 100, 0, 1, 98, 100, 0, 1, 66]).unwrap(),
    ///     Term::Map(vec![
    ///         (Term::Atom("a".to_string()), Term::Atom("A".to_string())),
    ///         (Term::Atom("b".to_string()), Term::Atom("B".to_string()))
    ///     ])
    /// );
    /// ```
    Map(Vec<(Term, Term)>),

    /// Shorthand for an empty list
    /// ```rust
    /// assert_eq!(
    ///     erlpack::Term::from_bytes(&[131, 106]).unwrap(),
    ///     erlpack::Term::Nil,
    /// );
    /// ```
    Nil,

    /// ```rust
    /// assert_eq!(
    ///     erlpack::Term::from_bytes(&[131, 108, 0, 0, 0, 1, 100, 0, 1, 97, 106]).unwrap(),
    ///     erlpack::Term::List(
    ///         vec![erlpack::Term::Atom("a".to_string())],
    ///         Box::new(erlpack::Term::Nil)
    ///     )
    /// );
    /// ```
    List(Vec<Term>, Box<Term>),

    /// ```rust
    /// assert_eq!(
    ///     erlpack::Term::from_bytes(&[131, 109, 0, 0, 0, 20, 115, 116, 114, 105, 110, 103, 115, 32, 97, 114, 101, 32, 98, 105, 110, 97, 114, 105, 101, 115]).unwrap(),
    ///     erlpack::Term::Binary(b"strings are binaries".to_vec())
    /// );
    /// ```
    Binary(Vec<u8>),
}

impl Term {
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        crate::decoder::Reader::new(bytes).read()
    }

    #[must_use]
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        crate::encoder::Encoder::new().encode(self)
    }
}
