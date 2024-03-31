use anchor_lang::prelude::*;
use anchor_spl::token::{Token,Mint,TokenAccount,MintTo};
use anchor_spl::token::mint_to;
use anchor_spl::associated_token::AssociatedToken;

declare_id!("9fxtdVYDHPh9qm34FL6d6aLJ4Hztc28BvGnGMn3aWXEF");

#[program]
pub mod safnect {
    use super::*;

    pub fn initialize(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
        Ok(())
    }

    pub fn add_subscription(
        ctx: Context<AddSubscription>,
        tag: String,
        description: String,
        qty: u8
    ) -> Result<()>{
        msg!("New Subscription Created");
        msg!("Tag: {}", tag);
        msg!("Description:{}", description);
        msg!("quantity:{}", qty);

        let subscription = &mut ctx.accounts.subscription;
        subscription.created_by = ctx.accounts.initializer.key();
        subscription.tag = tag;
        subscription.qty = qty;
        subscription.description = description;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo{
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info()
                },
                &[
                    &[
                        "mint".as_bytes(),
                        &[ctx.bumps.mint]
                    ]
                ]
            ),
            10*10^6
        )?;
        msg!("Minted tokens");

        Ok(())
    }

    pub fn update_subscription(
        ctx: Context<UpdateSubscription>,
        tag: String,
        description: String,
        qty: u8
    )->Result<()>{
        msg!("subscription reallocated");
        msg!("tag:{}", tag);
        msg!("description:{}", description);
        msg!("quantity:{}", qty);

        let subscription = &mut ctx.accounts.subscription;
        subscription.qty = qty;
        subscription.description = description;
        
        Ok(())
    }

}


#[account]
pub struct Subscription{
    pub created_by: Pubkey,
    pub qty: u8,
    pub tag: String,
    pub description: String,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(tag:String, description:String)]
pub struct AddSubscription<'info>{
    #[account(
        init,
        seeds = [tag.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 1 + 4 + tag.len() + 4 + description.len()
    )]
    pub subscription: Account<'info, Subscription>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds=["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
#[instruction(tag:String, description:String)]
pub struct UpdateSubscription<'info>{
    #[account(
        mut,
        seeds = [tag.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 1 + 4 + tag.len() + 4 + description.len(),
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub subscription: Account<'info, Subscription>,
    #[account[mut]]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}