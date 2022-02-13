use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod wallet_token {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let wallet = &mut ctx.accounts.wallet;
        wallet.authority = ctx.accounts.authority.key();
        wallet.bump = *ctx.bumps.get("wallet").unwrap();
        Ok(())
    }

    pub fn deposit1(ctx: Context<DepositDelegeted>, amount: u64) ->ProgramResult {
        let transfer_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.token.to_account_info(),
            authority: ctx.accounts.wallet.to_account_info(),
        };

        let wallet_key = ctx.accounts.wallet.key();
        let mint_key = ctx.accounts.mint.key();

        let seeds = &[
            wallet_key.as_ref(),
            mint_key.as_ref(),
            &[ctx.accounts.wallet.bump],
        ];

        let signer = &[&seeds[..]];

        let transfer_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            signer,
        );
        anchor_spl::token::transfer(transfer_context, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = authority,
        seeds = [authority.key().as_ref()],
        bump
    )]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DepositDelegeted<'info> {
    #[account(has_one = authority)]
    pub wallet: Account<'info, Wallet>,
    #[account(
        mut,
        seeds = [wallet.key().as_ref(), mint.key().as_ref()],
        constraint = token.owner == wallet.key() && token.mint == mint.key(),
        bump
    )]
    pub token: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = from.delegate.contains(&wallet.key()) && from.delegated_amount >= amount
    )]
    pub from: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct Wallet {
    pub authority: Pubkey,
    pub bump: u8,
}