use anchor_lang::prelude::*;

// devnet Id
declare_id!("8jBNH5HbemwKZSyNfz1fGLqp4tRTotU6fK8G8swXnWdj");

// localnet Id
// declare_id!("6VHsjGp5crWb4mhSjKwbu3nv5ReMpTmNCYjsQ8a1mSm");

#[program]
pub mod pixel_art_solana_app {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // get the reference to the base account from the accounts
        let base_account = &mut ctx.accounts.base_account;

        // return success
        Ok(())
    }

    pub fn add_pixel_art(ctx: Context<PixelTxn>, seed: String) -> ProgramResult {
        // get the reference to the base account from the accounts
        let base_account = &mut ctx.accounts.base_account;

        let user = &mut ctx.accounts.user;

        let item = PixelArtStruct {
            pixel_seed: seed.to_string(),
            user_address: *user.to_account_info().key,
            upvotes: 0,
            votes: Vec::new(),
        };

        let found = base_account.pixel_art_list.iter().position(|r| r.pixel_seed == seed.to_string());
        
        if found.is_some() {
            return Err(ProgramError::Custom(0));
        }

        // add the item to the list
        base_account.pixel_art_list.push(item);

        // return success
        Ok(())
    }

    pub fn vote(ctx: Context<PixelTxn>, seed: String) -> ProgramResult {
        // get the reference to the base account from the accounts
        let base_account = &mut ctx.accounts.base_account;

        let user = &mut ctx.accounts.user;

        // let index:usize = *base_account.seed_index.get(&seed).unwrap();
        let found = base_account.pixel_art_list.iter().position(|r| r.pixel_seed == seed.to_string());

        if found.is_none() {
            return Err(ProgramError::Custom(1));
        }

        let index = found.unwrap();

        let mut item  = &mut base_account.pixel_art_list[index];

        // check if the user hasnt already voted, increase the votes.
        let found_vote = item.votes.iter().position(|r| r == &*user.to_account_info().key);

        if found_vote.is_none() {
            item.upvotes += 1;
            item.votes.push(*user.to_account_info().key);
        } else  {
            //  the user has upvoted already, then remove the users address and decrease the vote
            item.upvotes -= 1;
            item.votes.retain(|&x| x != *user.to_account_info().key);
        }
        
        // if votes.len() > 0 {
        //     for (pos, vote) in &mut votes.iter().enumerate() {
        //         if vote.user_address == *user.to_account_info().key {
        //             votes[pos].upvote = upvote;
        //             updated = true;
        //         }
        //     }
        // } 

        // if the voter and vote data wasn't found, add it
        // if(!updated) {
        //     item.votes.push(VoteStruct {
        //         user_address: *user.to_account_info().key,
        //         upvote: upvote,
        //     });
        // }
        
        // increment or decrement upvote based on the bool value
        // if upvote {
        //     item.upvotes += 1;
        // } else {
        //     item.upvotes -= 1;
        // }
        
        // update the item in the list
        //base_account.pixel_art_list[index] = item;

        // return success
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program <'info, System>,

}

#[derive(Accounts)]
pub struct PixelTxn<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PixelArtStruct {
    pub pixel_seed: String,
    pub user_address: Pubkey,
    pub votes: Vec<Pubkey>,
    pub upvotes: u64,
}

#[account]
pub struct BaseAccount {
    // list of the pixel arts stored in the account
    pub pixel_art_list: Vec<PixelArtStruct>,
}