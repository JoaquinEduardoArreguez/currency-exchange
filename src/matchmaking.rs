use crate::order::Order;
use crate::order::Side;
use crate::order::Status;

pub fn find_match<'a, 'b>(
    maker_order: &'a Order,
    all_orders: &'b mut Vec<Order>,
) -> Option<(&'a Order, Order)> {
    let available_orders = all_orders.clone().into_iter().filter(|order| {
        let matching_side = match maker_order.side() {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        };

        let unit_price_matches = if *maker_order.side() == Side::Bid {
            order.unit_price() <= maker_order.unit_price()
        } else {
            order.unit_price() >= maker_order.unit_price()
        };

        order.uuid() != maker_order.uuid()
            && order.maker() != maker_order.maker()
            && *order.status() == Status::Pending
            && *order.side() == matching_side
            && order.amount() >= maker_order.amount()
            && unit_price_matches
    });

    let matching_order = match maker_order.side {
        Side::Bid => available_orders.min_by(|x, y| x.unit_price.cmp(&y.unit_price)),
        Side::Ask => available_orders.max_by(|x, y| x.unit_price.cmp(&y.unit_price)),
    };

    match matching_order {
        Some(taker_order) => Some((maker_order, taker_order)),
        None => None,
    }
}
