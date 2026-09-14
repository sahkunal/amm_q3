use anchor_lang:: prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Config{
    pub seed: u64, //to create different pools
    pub authority:Option<Pubkey>, //if want an authority to lock the config account
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16, //swap fee
    pub locked: bool,
    pub config_bump: u8, //bump seed for the config account
    pub lp_bump: u8, // bump seed for th LP token
}