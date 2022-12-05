use std::{ops::{Deref, DerefMut}, str::FromStr, fmt::Display};

use actix_web::{web};
use chrono::{Utc, DateTime, NaiveDateTime};
use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use sha2::{Sha256, Digest};

use super::errors::OrderServiceError;

const SERIALIZE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f %Z";

// Types

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Order {
    pub o_id: String,
    pub c_id: String,
    pub r_id: String,
    pub cust_addr: String,
    pub rest_addr: String,
}

#[derive(Debug, Default, Clone)]
pub struct OrderBuilder {
    pub o_id: Option<String>,
    pub c_id: Option<String>,
    pub r_id: Option<String>,
    pub cust_addr: Option<String>,
    pub rest_addr: Option<String>,
}

// Impls
impl Order {
    pub fn build(builder: OrderBuilder) -> Option<Self> {
        Some(Self {
            o_id: builder.o_id?,
            c_id: builder.c_id?,
            r_id: builder.r_id?,
            cust_addr: builder.cust_addr?,
            rest_addr: builder.rest_addr?,
        })
    }

    pub fn to_json_string(&self) -> Result<String, OrderServiceError> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => Err(OrderServiceError::from(e)),
        }
    }
}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold((0,1),|(acc,mul),&bit|(acc+(mul*(1&bit as u32)),mul.wrapping_add(mul))).0
}