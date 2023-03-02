use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("kenofas1oZFZvrTg75fLuBatmXSxLcCpNqTsnDCGY99");

pub const PREFIX: &str = "keno";
pub const GAME_CONFIG_SIZE: usize = 8 + 32 + 32 + 32 + 1;
pub const TICKET_SIZE: usize = 8 + 4 + 1 * 5;

#[program]
pub mod keno {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>) -> Result<()> {
        ctx.accounts.game_config.token_mint = ctx.accounts.token_mint.key();
        ctx.accounts.game_config.bank = ctx.accounts.bank.key();
        ctx.accounts.game_config.update_authority = ctx.accounts.update_authority.key();
        ctx.accounts.game_config.bump = *ctx.bumps.get("game_config").unwrap();
        Ok(())
    }

    pub fn submit_ticket(
        ctx: Context<SubmitTicket>,
        update_authority: Pubkey,
        numbers: Vec<u8>,
    ) -> Result<()> {
        ctx.accounts.ticket.numbers = numbers;
        ctx.accounts.ticket.bump = *ctx.bumps.get("ticket").unwrap();

        // Start randomness request.
        Ok(())
    }

    pub fn validate(ctx: Context<Validate>, update_authority: Pubkey) -> Result<()> {
        // Retrieve randomness result.

        // Compare to ticket.
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(init, payer = update_authority, space = GAME_CONFIG_SIZE, seeds = [PREFIX.as_bytes(), token_mint.key().as_ref(), bank.key().as_ref(), update_authority.key().as_ref()], bump)]
    pub game_config: Account<'info, GameConfig>,
    pub token_mint: Account<'info, Mint>,
    #[account(token::mint = token_mint)]
    pub bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub update_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameConfig {
    pub token_mint: Pubkey,
    pub bank: Pubkey,
    pub update_authority: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(update_authority: Pubkey)]
pub struct SubmitTicket<'info> {
    #[account(seeds = [PREFIX.as_bytes(), token_mint.key().as_ref(), bank.key().as_ref(), update_authority.key().as_ref()], bump = game_config.bump)]
    pub game_config: Account<'info, GameConfig>,
    #[account(init, payer = player, space = TICKET_SIZE, seeds = [PREFIX.as_bytes(), token_mint.key().as_ref(), bank.key().as_ref(), update_authority.key().as_ref(), player.key().as_ref()], bump)]
    pub ticket: Account<'info, Ticket>,
    pub token_mint: Account<'info, Mint>,
    #[account(token::mint = token_mint)]
    pub bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Ticket {
    pub numbers: Vec<u8>,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(update_authority: Pubkey)]
pub struct Validate<'info> {
    #[account(seeds = [PREFIX.as_bytes(), token_mint.key().as_ref(), bank.key().as_ref(), update_authority.key().as_ref()], bump = game_config.bump)]
    pub game_config: Account<'info, GameConfig>,
    #[account(seeds = [PREFIX.as_bytes(), token_mint.key().as_ref(), bank.key().as_ref(), update_authority.key().as_ref(), player.key().as_ref()], bump)]
    pub ticket: Account<'info, Ticket>,
    pub token_mint: Account<'info, Mint>,
    #[account(token::mint = token_mint)]
    pub bank: Account<'info, TokenAccount>,
    pub player: Signer<'info>,
}
