#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "moby")]
pub mod moby;

#[cfg(feature = "wordnet")]
pub mod wordnet;