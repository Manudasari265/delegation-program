#![allow(unexpected_cfgs)] // silence clippy for target_os solana and other solana program custom features

use solana_program::declare_id;

pub mod args;
pub mod consts;
mod discriminator;
pub mod error;
pub mod instruction_builder;
pub mod pda;
mod processor;
pub mod state;

#[cfg(feature = "log-cost")]
mod cu;
#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

declare_id!("DELeGGvXpWV2fqJUhqcF5ZSYMS4JTLjteaAMARRSaeSh");

pub mod fast {
    pinocchio_pubkey::declare_id!("DELeGGvXpWV2fqJUhqcF5ZSYMS4JTLjteaAMARRSaeSh");
}

#[cfg(all(not(feature = "no-entrypoint"), feature = "solana-security-txt"))]
solana_security_txt::security_txt! {
    name: "MagicBlock Delegation Program",
    project_url: "https://magicblock.gg",
    contacts: "email:dev@magicblock.gg,twitter:@magicblock",
    policy: "https://github.com/magicblock-labs/delegation-program/blob/master/LICENSE.md",
    preferred_languages: "en",
    source_code: "https://github.com/magicblock-labs/delegation-program"
}
