use anchor_lang::prelude::*;
use anchor_spl::token::Burn;

#[derive(Accounts)]
pub struct ProxyBurn<'info> {
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

impl <'a, 'b, 'c, 'info> From<&mut ProxyBurn<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(ctx: &mut ProxyBurn<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: ctx.mint.clone(),
            to: ctx.to.clone(),
            authority: ctx.authority.clone(),
        };
        let cpi_program = ctx.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
