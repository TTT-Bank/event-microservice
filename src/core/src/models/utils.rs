use std::{fmt::Display, str::FromStr};

use crate::error::DomainError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Offset {
        pub page: u32,
        pub limit: u32
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FilterOp<T>
{
        Eq(T),
        Ne(T),
        Gt(T),
        Lt(T),
        Gte(T),
        Lte(T)
}

impl<T: FromStr> FromStr for FilterOp<T>
where DomainError: std::convert::From<<T as FromStr>::Err>
{
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some((op, value)) = s.split_once(' ') {
                        match op.trim() {
                                "eq" => Ok(Self::Eq(value.parse()?)),
                                "ne" => Ok(Self::Ne(value.parse()?)),
                                "gt" => Ok(Self::Gt(value.parse()?)),
                                "lt" => Ok(Self::Lt(value.parse()?)),
                                "gte" => Ok(Self::Gte(value.parse()?)),
                                "lte" => Ok(Self::Lte(value.parse()?)),
                                _ => Err(DomainError::FilterOp(s.to_string()))
                        }
                } else {
                        Err(DomainError::Filter(s.to_string()))
                }
        }
}

impl<T> FilterOp<T> {
        pub fn operation(&self) -> String {
                match self {
                        Self::Eq(_) => String::from("="),
                        Self::Ne(_) => String::from("!="),
                        Self::Gt(_) => String::from(">"),
                        Self::Lt(_) => String::from("<"),
                        Self::Gte(_) => String::from(">="),
                        Self::Lte(_) => String::from("<=")
                }
        }

        pub fn value(&self) -> &T {
                match self {
                        Self::Eq(value) |
                        Self::Ne(value) |
                        Self::Gt(value) |
                        Self::Lt(value) |
                        Self::Gte(value) |
                        Self::Lte(value) => value
                }
        }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum OrderOp
{
        #[default]
        Asc,
        Desc
}

impl FromStr for OrderOp {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.trim() {
                        "asc" => Ok(Self::Asc),
                        "desc" => Ok(Self::Desc),
                        _ => Err(DomainError::OrderOp(s.to_string()))
                }
        }
}

impl Display for OrderOp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                        Self::Asc => f.write_str("ASC"),
                        Self::Desc => f.write_str("DESC")
                }
        }
}
