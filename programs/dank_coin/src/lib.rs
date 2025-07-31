// Dank Coin - SPL Token Project for Solana
// Anchor-based smart contract for deploying the DANK token

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token, InitializeMint, MintTo};

pub const DANK_DECIMALS: u8 = 9;
pub const DANK_SUPPLY: u64 = 300_000_000_000_000_000; // 300M * 10^9

#[program]
pub mod dank_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("DANK token contract initialized.");

        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            DANK_DECIMALS,
            ctx.accounts.authority.key,
            Some(ctx.accounts.authority.key),
        )?;

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            DANK_SUPPLY,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 82, mint::decimals = DANK_DECIMALS, mint::authority = authority, mint::freeze_authority = authority)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}
