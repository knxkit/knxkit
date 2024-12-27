// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

/// Represents the various errors that can occur in the KNXKit project.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Represents a general project error with a static string message.
    #[error("Project error: {0}")]
    ProjectError(&'static str),

    /// Represents a parsing error with a string message.
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Represents an error that occurs during ZIP operations.
    #[error("ZIP Error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    /// Represents an error that occurs during XML parsing.
    #[error("XML Error: {0}")]
    XmlError(#[from] roxmltree::Error),

    /// Represents an I/O error.
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}
