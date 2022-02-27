use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    MintTokens, // Authority to mint new tokens
    FreezeAccount, // Authority to freeze any account associated with the mint
    AccountOwner, // Owner of any given account
    CloseAccount, // Authority to close token account
}

impl From<AuthorityType> for spl_token::instruction::AuthorityType {
    fn from(authority_type: AuthorityType) -> spl_token::instruction::AuthorityType {
        match authority_type {
            AuthorityType::MintTokens => spl_token::instruction::AuthorityType::MintTokens,
            AuthorityType::FreezeAccount => spl_token::instruction::AuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => spl_token::instruction::AuthorityType::AccountOwner,
            AuthorityType::CloseAccount => spl_token::instruction::AuthorityType::CloseAccount,
        }
    }
}