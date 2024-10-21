// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! A crate for a generic Rust PLONK implementation using arkworks as a backend.

#![cfg_attr(not(any(test)), no_std)]
#![forbid(rustdoc::broken_intra_doc_links)]
#![forbid(missing_docs)]

#[doc(inline)]
pub use plonk_core::*;

#[doc(inline)]
pub use plonk_hashing as hashing;
