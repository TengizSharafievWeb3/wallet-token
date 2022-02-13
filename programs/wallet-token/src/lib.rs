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

    pub fn init_token_account(_ctx: Context<InitTokenAccount>) -> ProgramResult {
        Ok(())
    }

    pub fn deposit1(ctx: Context<DepositDelegeted>, amount: u64) -> ProgramResult {
        require!(ctx.accounts.from.delegate.contains(&ctx.accounts.wallet.key()), WalletTokenError::NotDelegated);
        require!(ctx.accounts.from.delegated_amount >= amount, WalletTokenError::NotEnoughBalance);
        let transfer_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.token.to_account_info(),
            authority: ctx.accounts.wallet.to_account_info(),
        };


        let bump = *ctx.bumps.get("wallet").unwrap();

        let seeds = &[
            ctx.accounts.wallet.authority.as_ref(),
            &[bump],
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
        seeds = [authority.key().as_ref()], bump
    )]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitTokenAccount<'info> {
    #[account(has_one = authority, seeds = [authority.key().as_ref()], bump)]
    pub wallet: Account<'info, Wallet>,
    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = wallet,
        seeds = [wallet.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub token: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DepositDelegeted<'info> {
    #[account(has_one = authority, seeds = [authority.key().as_ref()], bump)]
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
        constraint = from.mint == mint.key()
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

#[error]
pub enum WalletTokenError {
    #[msg("from token account doesn't have approve for wallet account")]
    NotDelegated,
    #[msg("from token account balance isn't enough")]
    NotEnoughBalance,
}