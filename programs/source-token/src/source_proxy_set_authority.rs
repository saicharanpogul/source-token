use anchor_lang::prelude::*;
use anchor_spl::token::SetAuthority;

#[derive(Accounts)]
pub struct ProxySetAuthority<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub current_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous
    pub account_or_mint: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    pub token_program: AccountInfo<'info>
}

impl <'a, 'b, 'c, 'info> From<&mut ProxySetAuthority<'info>> for CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
    fn from(ctx: &mut ProxySetAuthority<'info>) -> CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            current_authority: ctx.current_authority.clone(),
            account_or_mint: ctx.account_or_mint.clone(),
        };
        let cpi_program = ctx.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}