use anchor_lang::prelude::*;
use anchor_spl::token::MintTo;

#[derive(Accounts)]
pub struct ProxyMintTo<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous
    pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    pub token_program: AccountInfo<'info>
}

impl <'a, 'b, 'c, 'info> From<&mut ProxyMintTo<'info>> for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
    fn from(ctx: &mut ProxyMintTo<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: ctx.mint.clone(),
            to: ctx.to.clone(),
            authority: ctx.authority.clone(),
        };
        let cpi_program = ctx.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
