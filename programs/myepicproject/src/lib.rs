use anchor_lang::prelude::*;

declare_id!("2wX9TTY9jPMQTsqZWcdjhYUePRAdrp914VLEkhc7CXCV");

// Programs are stateless, so we need to store data in accounts (see below)
#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // 3. Grab base_account from the StartStuffOff context. &mut means mutable reference; we can make changes to base_account, not just a copy
        let base_account = &mut ctx.accounts.base_account; 
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link:String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key
        };
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// 2. Specify what data we want in each Context and what functions can do to them
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // We initialize the account and what to hold in the StartStuffOff context
    #[account(init, payer = user, space = 9000)] // user will pay 'rent' for the account, size of 9000 bytes 
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>, // Data passed to the program that proves that they own their wallet account
    pub system_program: Program<'info, System>, // SystemProgram runs Solana. It creates accounts on Solana.
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)] // mutable reference to base_account: if i change base_account in my function, it will change the data on the account
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info> // we need a signer so that we can save the data into base_account
}

// Create a custom struct for us to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)] // We need to serialize/deserialize the struct into binary format to store in an account
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Define what account can store
// 1. What we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>, // Attach a Vector of type ItemStruct to the account. It's like an array
}