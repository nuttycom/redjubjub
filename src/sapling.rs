//! Signature types for the Sapling protocol.

use super::SigType;

/// A type variable corresponding to Zcash's Sapling `SpendAuthSig`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SpendAuth {}
impl SigType for SpendAuth {}
impl super::SpendAuth for SpendAuth {}

/// A type variable corresponding to Zcash's Sapling `BindingSig`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Binding {}
impl SigType for Binding {}
impl super::Binding for Binding {}
