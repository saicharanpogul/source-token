use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous 
    pub from: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous
    pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    pub token_program: AccountInfo<'info>
}

impl <'a, 'b, 'c, 'info> From<&mut ProxyTransfer<'info>> for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
    fn from(ctx: &mut ProxyTransfer<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: ctx.from.clone(),
            to: ctx.to.clone(),
            authority: ctx.authority.clone(),
        };
        let cpi_program = ctx.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}