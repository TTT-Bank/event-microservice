use std::{fmt::Display, str::FromStr};

use time::PrimitiveDateTime;

use crate::error::DomainError;
use super::utils::{FilterOp, OrderOp};

pub type UserId = u64;

#[derive(Debug, Clone)]
pub struct UserCredentials {
        pub login: String,
        pub password: String
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum UserRole {
        #[default]
        User,
        Organizer,
        Admin
}

impl FromStr for UserRole {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.trim() {
                        "User" => Ok(Self::User),
                        "Organizer" => Ok(Self::Organizer),
                        "Admin" => Ok(Self::Admin),
                        _ => Err(DomainError::Parse(s.to_string()))
                }
        }
}

impl Display for UserRole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = match self {
                        Self::User => "User",
                        Self::Organizer => "Organizer",
                        Self::Admin => "Admin"
                };

                f.write_str(string)
        }
}

#[derive(Debug, Clone)]
pub struct UserModel {
        pub id: UserId,
        pub login: String,
        pub role: UserRole,
        pub created_at: PrimitiveDateTime,
        pub updated_at: PrimitiveDateTime
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserUpdate {
        Role(UserRole),
        Password(String),
        Login(String)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserFilter {
        Role(FilterOp<UserRole>),
        Login(FilterOp<String>)
}

impl FromStr for UserFilter {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.trim();
                if let Some((field, op)) = s.split_once(' ') {
                        match field.trim() {
                                "role" => Ok(Self::Role(op.parse()?)),
                                "login" => Ok(Self::Login(op.parse()?)),
                                _ => Err(DomainError::Filter(s.to_string()))
                        }
                } else {
                        Err(DomainError::Filter(s.to_string()))
                }
        }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserOrder {
        Id(OrderOp),
        Role(OrderOp),
        CreatedAt(OrderOp),
        UpdatedAt(OrderOp)
}

impl FromStr for UserOrder {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some((field, op)) = s.split_once(' ') {
                        match field.trim() {
                                "id" => Ok(Self::Id(op.parse()?)),
                                "role" => Ok(Self::Role(op.parse()?)),
                                "created_at" => Ok(Self::CreatedAt(op.parse()?)),
                                "updated_at" => Ok(Self::UpdatedAt(op.parse()?)),
                                _ => Err(DomainError::Order(s.to_string()))
                        }
                } else {
                        match s.trim() {
                                "id" => Ok(Self::Id(OrderOp::default())),
                                "role" => Ok(Self::Role(OrderOp::default())),
                                "created_at" => Ok(Self::CreatedAt(OrderOp::default())),
                                "updated_at" => Ok(Self::UpdatedAt(OrderOp::default())),
                                _ => Err(DomainError::Order(s.to_string()))
                        }
                }
        }
}

impl Default for UserOrder {
        fn default() -> Self {
                Self::Id(OrderOp::default())
        }
}
