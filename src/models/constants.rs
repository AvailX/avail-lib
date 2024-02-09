#![allow(dead_code)]
use snarkvm::prelude::{const_assert, hrp2};

pub const PRIVATE_KEY: &str = "avl-p";
pub const VIEW_KEY: &str = "avl-v";
pub const TX_PREFIX: u16 = hrp2!("at");
pub const TRANSITION_PREFIX: u16 = hrp2!("au");

/* TESTING */
pub const WEAK_PASSWORD: &str = "password";
pub const STRONG_PASSWORD: &str = "ywZ9&377DQd5";
pub const TESTNET_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

pub const TESTNET_ADDRESS: &str = "aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px";

pub const TESTNET3_PRIVATE_KEY: &str =
    "APrivateKey1zkp64gV6fJFCLXi8saX8QbTsq2MXQ7Cz1Br326C9TjGfdiC";
pub const TESTNET3_VIEW_KEY: &str = "AViewKey1r7dvuCHS8ZbVGoaN2E6r1Q9iEp2TT58GNPN3m4h8fCiC";
pub const TESTNET3_ADDRESS: &str =
    "aleo10ttmj7sx2gfmk2r6t2eudkhhv0el593x9p3m5xyz4mahn3ae8qrqjvtlwf";

pub const TESTNET3_ADDRESS_2: &str =
    "aleo1hxwmpdem56a3z35z65kckn0cn0xpmzrjglhnlqzkepcdlx4xwsfqu40t35";
pub const TESTNET3_VIEW_KEY2: &str = "AViewKey1cjLTLFqkNwZE3DZrTaEUxNzFz3BxbE3R9N7Q6J5sc3J7";
pub const TESTNET3_PRIVATE_KEY2: &str =
    "APrivateKey1zkp3oc2X5EJfAtJNbtZDYq2aUveAAsUxbQxwkh2DFEr67Mn";

/* VERIFIED :) */
