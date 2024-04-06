use drift::controller::position::PositionDirection;
use drift::error::DriftResult;
use drift::math::casting::Cast;
use drift::math::constants::*;
use drift::math::safe_math::SafeMath;
use drift::math::spot_balance::get_token_amount;
use drift::state::oracle::OraclePriceData;
use drift::state::perp_market::PerpMarket;
use drift::state::spot_market::SpotBalanceType;
use drift::state::spot_market::SpotMarket;

pub fn compute_borrow_rate(spot_market: &SpotMarket) -> DriftResult<u128> {
    let deposit_token_amount = get_token_amount(
        spot_market.deposit_balance,
        spot_market,
        &SpotBalanceType::Deposit,
    )?;
    let borrow_token_amount = get_token_amount(
        spot_market.borrow_balance,
        spot_market,
        &SpotBalanceType::Borrow,
    )?;

    let utilization = drift::math::spot_balance::calculate_utilization(
        deposit_token_amount,
        borrow_token_amount,
    )?;

    if utilization == 0 {
        return Ok(0);
    }

    let borrow_rate = if utilization > spot_market.optimal_utilization.cast()? {
        let surplus_utilization = utilization.safe_sub(spot_market.optimal_utilization.cast()?)?;

        let borrow_rate_slope = spot_market
            .max_borrow_rate
            .cast::<u128>()?
            .safe_sub(spot_market.optimal_borrow_rate.cast()?)?
            .safe_mul(SPOT_UTILIZATION_PRECISION)?
            .safe_div(
                SPOT_UTILIZATION_PRECISION.safe_sub(spot_market.optimal_utilization.cast()?)?,
            )?;

        spot_market.optimal_borrow_rate.cast::<u128>()?.safe_add(
            surplus_utilization
                .safe_mul(borrow_rate_slope)?
                .safe_div(SPOT_UTILIZATION_PRECISION)?,
        )?
    } else {
        let borrow_rate_slope = spot_market
            .optimal_borrow_rate
            .cast::<u128>()?
            .safe_mul(SPOT_UTILIZATION_PRECISION)?
            .safe_div(spot_market.optimal_utilization.cast()?)?;

        utilization
            .safe_mul(borrow_rate_slope)?
            .safe_div(SPOT_UTILIZATION_PRECISION)?
    };

    Ok(borrow_rate)
}
