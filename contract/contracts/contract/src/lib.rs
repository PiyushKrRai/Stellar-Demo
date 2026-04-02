#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Vec, Symbol
};

#[derive(Clone)]
#[contracttype]
pub struct Pool {
    pub id: u64,
    pub creator: Address,
    pub contribution_amount: i128,
    pub total_members: u32,
    pub round_duration: u64,
    pub start_time: u64,
    pub members: Vec<Address>,
    pub current_round: u32,
    pub payouts: Map<u32, Address>,
    pub contributions: Map<(u32, Address), bool>,
    pub is_active: bool,
}

#[contracttype]
pub enum DataKey {
    Pool(u64),
    PoolCount,
}

#[contract]
pub struct SavingsClub;

#[contractimpl]
impl SavingsClub {

    // 📌 Create Pool (Permissionless)
    pub fn create_pool(
        env: Env,
        creator: Address,
        contribution_amount: i128,
        total_members: u32,
        round_duration: u64,
    ) -> u64 {
        creator.require_auth();

        let mut pool_count: u64 = env.storage().instance().get(&DataKey::PoolCount).unwrap_or(0);
        pool_count += 1;

        let mut members = Vec::new(&env);
        members.push_back(creator.clone());

        let pool = Pool {
            id: pool_count,
            creator: creator.clone(),
            contribution_amount,
            total_members,
            round_duration,
            start_time: env.ledger().timestamp(),
            members,
            current_round: 1,
            payouts: Map::new(&env),
            contributions: Map::new(&env),
            is_active: true,
        };

        env.storage().instance().set(&DataKey::Pool(pool_count), &pool);
        env.storage().instance().set(&DataKey::PoolCount, &pool_count);

        pool_count
    }

    // 📌 Join Pool (Open to anyone)
    pub fn join_pool(env: Env, user: Address, pool_id: u64) {
        user.require_auth();

        let mut pool: Pool = env.storage().instance().get(&DataKey::Pool(pool_id)).unwrap();

        if pool.members.len() >= pool.total_members {
            panic!("Pool full");
        }

        // Prevent duplicate joins
        for member in pool.members.iter() {
            if member == user {
                panic!("Already joined");
            }
        }

        pool.members.push_back(user);

        env.storage().instance().set(&DataKey::Pool(pool_id), &pool);
    }

    // 📌 Contribute (Send XLM)
    pub fn contribute(env: Env, user: Address, pool_id: u64) {
        user.require_auth();

        let mut pool: Pool = env.storage().instance().get(&DataKey::Pool(pool_id)).unwrap();

        if !pool.is_active {
            panic!("Pool inactive");
        }

        let key = (pool.current_round, user.clone());

        if pool.contributions.contains_key(key.clone()) {
            panic!("Already contributed");
        }

        // Transfer XLM to contract
        let client = soroban_sdk::token::Client::new(
            &env,
            &env.current_contract_address(),
        );

        client.transfer(
            &user,
            &env.current_contract_address(),
            &pool.contribution_amount,
        );

        pool.contributions.set(key, true);

        env.storage().instance().set(&DataKey::Pool(pool_id), &pool);
    }

    // 📌 Select Winner (Permissionless trigger)
    pub fn select_winner(env: Env, pool_id: u64) {
        let mut pool: Pool = env.storage().instance().get(&DataKey::Pool(pool_id)).unwrap();

        let now = env.ledger().timestamp();

        // Check if all contributed OR timeout
        let mut all_paid = true;
        for member in pool.members.iter() {
            if !pool.contributions.contains_key((pool.current_round, member.clone())) {
                all_paid = false;
                break;
            }
        }

        if !(all_paid || now >= pool.start_time + pool.round_duration) {
            panic!("Round not finished");
        }

        // Pseudo-random winner
        let seed = now + pool_id + pool.current_round as u64;
        let index = (seed % pool.members.len() as u64) as u32;

        let winner = pool.members.get(index).unwrap();

        pool.payouts.set(pool.current_round, winner.clone());
        pool.current_round += 1;

        // Reset timer for next round
        pool.start_time = now;

        env.storage().instance().set(&DataKey::Pool(pool_id), &pool);
    }

    // 📌 Claim Payout
    pub fn claim_payout(env: Env, user: Address, pool_id: u64, round: u32) {
        user.require_auth();

        let pool: Pool = env.storage().instance().get(&DataKey::Pool(pool_id)).unwrap();

        let winner = pool.payouts.get(round).unwrap();

        if winner != user {
            panic!("Not winner");
        }

        let total_pot = pool.contribution_amount * pool.members.len() as i128;

        let client = soroban_sdk::token::Client::new(
            &env,
            &env.current_contract_address(),
        );

        client.transfer(
            &env.current_contract_address(),
            &user,
            &total_pot,
        );
    }

    // 📌 View Pool
    pub fn get_pool(env: Env, pool_id: u64) -> Pool {
        env.storage().instance().get(&DataKey::Pool(pool_id)).unwrap()
    }
}