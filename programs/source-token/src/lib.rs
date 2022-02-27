mod authority_type;
mod source_proxy_burn;
mod source_proxy_transfer;
mod source_proxy_mint_to;
mod source_proxy_set_authority;

use anchor_lang::prelude::*;
use authority_type::*;
use source_proxy_transfer::*;
use source_proxy_mint_to::*;
use source_proxy_burn::*;
use source_proxy_set_authority::*;
use anchor_spl::token;

declare_id!("7NGPsRciPut1JeFCLCUhkUkJakqDGYFP4qUEzUHakTq8");

#[program]
pub mod source_token {
    use super::*;
    
    pub fn proxy_transfer(ctx: Context<ProxyTransfer>, amount: u64) -> Result<()> {
        token::transfer(ctx.accounts.into(), amount)
    }

    pub fn proxy_mint_to(ctx: Context<ProxyMintTo>, amount: u64) -> Result<()> {
        token::mint_to(ctx.accounts.into(), amount)
    }

    pub fn proxy_burn(ctx: Context<ProxyBurn>, amount: u64) -> Result<()> {
        token::burn(ctx.accounts.into(), amount)
    }

    pub fn proxy_set_authority(
      ctx: Context<ProxySetAuthority>,
      authority_type: AuthorityType,
      new_authority: Option<Pubkey>,
    ) -> Result<()> {
        token::set_authority(ctx.accounts.into(), authority_type.into(), new_authority)
    }
}