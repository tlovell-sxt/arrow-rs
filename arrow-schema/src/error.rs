// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Defines `ArrowError` for representing failures in various Arrow operations.
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Many different operations in the `arrow` crate return this error type.
#[derive(Debug)]
pub enum ArrowError {
    /// Returned when functionality is not yet available.
    NotYetImplemented(String),
    ExternalError(Box<dyn Error + Send + Sync>),
    CastError(String),
    MemoryError(String),
    ParseError(String),
    SchemaError(String),
    ComputeError(String),
    DivideByZero,
    ArithmeticOverflow(String),
    CsvError(String),
    JsonError(String),
    IpcError(String),
    InvalidArgumentError(String),
    ParquetError(String),
    /// Error during import or export to/from the C Data Interface
    CDataInterface(String),
    DictionaryKeyOverflowError,
    RunEndIndexOverflowError,
}

impl ArrowError {
    /// Wraps an external error in an `ArrowError`.
    pub fn from_external_error(error: Box<dyn Error + Send + Sync>) -> Self {
        Self::ExternalError(error)
    }
}

impl From<core::str::Utf8Error> for ArrowError {
    fn from(error: core::str::Utf8Error) -> Self {
        ArrowError::ParseError(error.to_string())
    }
}

impl From<alloc::string::FromUtf8Error> for ArrowError {
    fn from(error: alloc::string::FromUtf8Error) -> Self {
        ArrowError::ParseError(error.to_string())
    }
}

impl Display for ArrowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ArrowError::NotYetImplemented(source) => {
                write!(f, "Not yet implemented: {}", &source)
            }
            ArrowError::ExternalError(source) => write!(f, "External error: {}", &source),
            ArrowError::CastError(desc) => write!(f, "Cast error: {desc}"),
            ArrowError::MemoryError(desc) => write!(f, "Memory error: {desc}"),
            ArrowError::ParseError(desc) => write!(f, "Parser error: {desc}"),
            ArrowError::SchemaError(desc) => write!(f, "Schema error: {desc}"),
            ArrowError::ComputeError(desc) => write!(f, "Compute error: {desc}"),
            ArrowError::ArithmeticOverflow(desc) => write!(f, "Arithmetic overflow: {desc}"),
            ArrowError::DivideByZero => write!(f, "Divide by zero error"),
            ArrowError::CsvError(desc) => write!(f, "Csv error: {desc}"),
            ArrowError::JsonError(desc) => write!(f, "Json error: {desc}"),
            ArrowError::IpcError(desc) => write!(f, "Ipc error: {desc}"),
            ArrowError::InvalidArgumentError(desc) => {
                write!(f, "Invalid argument error: {desc}")
            }
            ArrowError::ParquetError(desc) => {
                write!(f, "Parquet argument error: {desc}")
            }
            ArrowError::CDataInterface(desc) => {
                write!(f, "C Data interface error: {desc}")
            }
            ArrowError::DictionaryKeyOverflowError => {
                write!(f, "Dictionary key bigger than the key type")
            }
            ArrowError::RunEndIndexOverflowError => {
                write!(f, "Run end encoded array index overflow error")
            }
        }
    }
}

impl Error for ArrowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ArrowError::ExternalError(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error_source() {
        let e1 = ArrowError::DivideByZero;
        assert!(e1.source().is_none());

        // one level of wrapping
        let e2 = ArrowError::ExternalError(Box::new(e1));
        let source = e2.source().unwrap().downcast_ref::<ArrowError>().unwrap();
        assert!(matches!(source, ArrowError::DivideByZero));

        // two levels of wrapping
        let e3 = ArrowError::ExternalError(Box::new(e2));
        let source = e3
            .source()
            .unwrap()
            .downcast_ref::<ArrowError>()
            .unwrap()
            .source()
            .unwrap()
            .downcast_ref::<ArrowError>()
            .unwrap();

        assert!(matches!(source, ArrowError::DivideByZero));
    }
}
