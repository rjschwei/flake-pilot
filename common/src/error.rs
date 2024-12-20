//
// Copyright (c) 2023 Elektrobit Automotive GmbH
// Copyright (c) 2023 Marcus Schäfer
//
// This file is part of flake-pilot
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
use std::process::{ExitCode, Output, Termination};
use crate::command::{CommandError, ProcessError};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum FlakeError {
    /// The pilot tried to run a sub command and failed
    #[error("Failed to run {}", .0)]
    CommandError(#[from] CommandError),

    #[error("IOError: {kind:?} {message:?}")]
    IOError {
        kind: String,
        message: String
    },

    /// IO operation pass through
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// MalformedJson pass through
    #[cfg(feature = "json")]
    #[error(transparent)]
    MalformedJson(#[from] serde_json::Error),

    /// Unknown command for resume mode
    #[error("Unknown command for resume mode, specify a target_app_path")]
    UnknownCommand,

    /// This flake is already running
    #[error("Instance in use by another instance, consider @NAME argument")]
    AlreadyRunning,

    #[error("Datasync failed, for details recall with PILOT_DEBUG=1")]
    SyncFailed,

    /// OperationError pass through
    #[error("{}", .0)]
    OperationError(#[from] OperationError)
}

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Max retries exceeded, for details recall with PILOT_DEBUG=1")]
    MaxTriesExceeded
}

impl Termination for FlakeError {
    /// A failed sub command will forward its error code
    ///
    /// All other errors are represented as Failure
    fn report(self) -> std::process::ExitCode {
        match self {
            FlakeError::CommandError(
                CommandError {
                    base: ProcessError::ExecutionError(Output { status, .. }),
                    ..
                }
            ) => match status.code() {
                Some(code) => (code as u8).into(),
                None => ExitCode::FAILURE,
            },
            _ => ExitCode::FAILURE,
        }
    }
}

