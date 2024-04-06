use drift::{
    controller::{
        order::place_perp_order, position::PositionDirection,
        spot_position::update_spot_balances_and_cumulative_deposits,
    },
    cpi::accounts::PlaceAndTake,
    error::DriftResult,
    instructions::{
        optional_accounts::{load_maps, AccountMaps},
        user::handle_modify_order,
        OrderParams,
    },
    math::orders::standardize_base_asset_amount_ceil,
    state::{
        oracle::OraclePriceData,
        perp_market_map::MarketSet,
        spot_market::SpotMarket,
        user::{MarketType, OrderTriggerCondition, OrderType, SpotPosition},
    },
};

pub fn get_order_params(
    order_type: OrderType,
    market_type: MarketType,
    direction: PositionDirection,
    base_asset_amount: u64,
    market_index: u16,
    reduce_only: bool,
) -> OrderParams {
    OrderParams {
        order_type,
        market_type,
        direction,
        base_asset_amount,
        market_index,
        reduce_only,
        user_order_id: 0,
        price: 0,
        post_only: false,
        immediate_or_cancel: false,
        trigger_price: None,
        trigger_condition: OrderTriggerCondition::Above,
        oracle_price_offset: None,
        auction_duration: None,
        max_ts: None,
        auction_start_price: None,
        auction_end_price: None,
    }
}

pub fn set_perp_order_param() {
    let order_base_amount =
        standardize_base_asset_amount_ceil(order_base_amount, perp_market.amm.order_step_size)
            .unwrap();

    let params = get_order_params(
        OrderType::Market,
        MarketType::Perp,
        direction,
        order_base_amount,
        perp_market_index,
        false,
    );
}

pub fn set_spot_order_param() {
    let spot_order_size =
        standardize_base_asset_amount_ceil(spot_order_size, spot_market.order_step_size).unwrap();

    let params = get_order_params(
        OrderType::Market,
        MarketType::Spot,
        direction,
        spot_order_size,
        spot_market_index,
        false,
    );
}

fn place_and_take<'info>(
    ctx: &Context<'_, '_, '_, 'info, Delta<'info>>,
    orders_params: OrderParams,
) -> Result<()> {
    let drift_program = ctx.accounts.drift_program.to_account_info().clone();

    let cpi_accounts = PlaceAndTake {
        state: ctx.accounts.state.to_account_info().clone(),
        user: ctx.accounts.user.to_account_info().clone(),
        user_stats: ctx.accounts.user_stats.to_account_info().clone(),
        authority: ctx.accounts.authority.to_account_info().clone(),
    };

    let cpi_context = CpiContext::new(drift_program, cpi_accounts)
        .with_remaining_accounts(ctx.remaining_accounts.into());

    drift::cpi::place_and_take_perp_order(cpi_context, order_params, None)?;

    Ok(())
}

/// lend and borrow
// pub fn transfer_spot_position_deposit(
//     token_amount: i128,
//     spot_market: &mut SpotMarket,
//     from_spot_position: &mut SpotPosition,
//     to_spot_position: &mut SpotPosition,
// ) -> DriftResult {
//     validate!(
//         from_spot_position.market_index == to_spot_position.market_index,
//         ErrorCode::UnequalMarketIndexForSpotTransfer,
//         "transfer market indexes arent equal",
//     )?;

//     update_spot_balances_and_cumulative_deposits(
//         token_amount.unsigned_abs(),
//         &SpotBalanceType::Deposit,
//         spot_market,
//         from_spot_position,
//         false,
//         None,
//     )?;

//     update_spot_balances_and_cumulative_deposits(
//         token_amount.unsigned_abs(),
//         &SpotBalanceType::Borrow,
//         spot_market,
//         to_spot_position,
//         false,
//         None,
//     )?;
// }

// pub fn add_short_position<'info>(
//     ctx: Context<'_, '_, '_, 'info, Delta<'info>>,
//     market_index: u16,
//     amount: u128,
// ) -> Result<()> {
//     let clock = Clock::get()?;
//     let slot = clock.slot;
//     let now = clock.unix_timestamp;

//     let taker = ctx.accounts.user.load()?;

//     let (base_init, quote_init) = taker
//         .get_perp_position(market_index)
//         .map_or((0, 0), |p| (p.base_asset_amount, p.quote_asset_amount));

//     let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
//     let AccountMaps {
//         perp_market_map,
//         mut oracle_map,
//         spot_market_map,
//     } = load_maps(
//         remaining_accounts_iter,
//         &MarketSet::new(),
//         &MarketSet::new(),
//         slot,
//         None,
//     )?;

//     let perp_market = perp_market_map.get_ref(&market_index)?;
//     let oracle_price_data = oracle_map.get_price_data(&perp_market.amm.oracle)?;

//     (base_init + amount)
// }

#[derive(Accounts)]
pub struct Delta<'info> {
    pub state: Box<Account<'info, State>>,
    #[account(mut)]
    pub user: AccountLoader<'info, User>,
    #[account(mut)]
    pub user_stats: AccountLoader<'info, UserStats>,
    pub authority: Signer<'info>,
    pub drift_program: Program<'info, Drift>,
}
