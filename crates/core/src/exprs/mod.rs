/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

pub mod filter;

use anyhow::{anyhow, Error};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub use filter::*;

/// An operator that represents a comparison operation used in a partition filter expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HudiOperator {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
}

impl Display for HudiOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            // Binary Operators
            HudiOperator::Eq => write!(f, "="),
            HudiOperator::Ne => write!(f, "!="),
            HudiOperator::Lt => write!(f, "<"),
            HudiOperator::Lte => write!(f, "<="),
            HudiOperator::Gt => write!(f, ">"),
            HudiOperator::Gte => write!(f, ">="),
        }
    }
}

// TODO: Add more operators
impl HudiOperator {
    pub const TOKEN_OP_PAIRS: [(&'static str, HudiOperator); 6] = [
        ("=", HudiOperator::Eq),
        ("!=", HudiOperator::Ne),
        ("<", HudiOperator::Lt),
        ("<=", HudiOperator::Lte),
        (">", HudiOperator::Gt),
        (">=", HudiOperator::Gte),
    ];
}

impl FromStr for HudiOperator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HudiOperator::TOKEN_OP_PAIRS
            .iter()
            .find_map(|&(token, op)| {
                if token.eq_ignore_ascii_case(s) {
                    Some(op)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("Unsupported operator: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_from_str() {
        assert_eq!(HudiOperator::from_str("=").unwrap(), HudiOperator::Eq);
        assert_eq!(HudiOperator::from_str("!=").unwrap(), HudiOperator::Ne);
        assert_eq!(HudiOperator::from_str("<").unwrap(), HudiOperator::Lt);
        assert_eq!(HudiOperator::from_str("<=").unwrap(), HudiOperator::Lte);
        assert_eq!(HudiOperator::from_str(">").unwrap(), HudiOperator::Gt);
        assert_eq!(HudiOperator::from_str(">=").unwrap(), HudiOperator::Gte);
        assert!(HudiOperator::from_str("??").is_err());
    }
}