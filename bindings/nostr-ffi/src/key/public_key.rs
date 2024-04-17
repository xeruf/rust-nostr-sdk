// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;

use nostr::nips::nip19::{FromBech32, ToBech32};
use nostr::nips::nip21::NostrURI;
use uniffi::Object;

use crate::error::Result;

#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct PublicKey {
    inner: nostr::PublicKey,
}

impl Deref for PublicKey {
    type Target = nostr::PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nostr::PublicKey> for PublicKey {
    fn from(inner: nostr::PublicKey) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl PublicKey {
    /// Try to parse public key from `hex`, `bech32` or [NIP21](https://github.com/nostr-protocol/nips/blob/master/21.md) uri
    #[inline]
    #[uniffi::constructor]
    pub fn parse(public_key: &str) -> Result<Self> {
        Ok(Self {
            inner: nostr::PublicKey::parse(public_key)?,
        })
    }

    #[inline]
    #[uniffi::constructor]
    pub fn from_hex(hex: &str) -> Result<Self> {
        Ok(Self {
            inner: nostr::PublicKey::from_hex(hex)?,
        })
    }

    #[inline]
    #[uniffi::constructor]
    pub fn from_bech32(bech32: &str) -> Result<Self> {
        Ok(Self {
            inner: nostr::PublicKey::from_bech32(bech32)?,
        })
    }

    #[inline]
    #[uniffi::constructor]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            inner: nostr::PublicKey::from_slice(bytes)?,
        })
    }

    #[inline]
    #[uniffi::constructor]
    pub fn from_nostr_uri(uri: &str) -> Result<Self> {
        Ok(Self {
            inner: nostr::PublicKey::from_nostr_uri(uri)?,
        })
    }

    #[inline]
    pub fn to_hex(&self) -> String {
        self.inner.to_string()
    }

    #[inline]
    pub fn to_bech32(&self) -> Result<String> {
        Ok(self.inner.to_bech32()?)
    }

    #[inline]
    pub fn to_nostr_uri(&self) -> Result<String> {
        Ok(self.inner.to_nostr_uri()?)
    }
}
