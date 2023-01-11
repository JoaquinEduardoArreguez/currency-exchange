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

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    use crate::{
        matchmaking::find_match,
        order::{Order, Side},
    };

    fn generate_id() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect()
    }

    fn generate_matching_orders(side: Side) -> (Order, Order) {
        let high_unit_price = 200;
        let low_unit_price = 100;

        let maker_order = Order {
            uuid: generate_id(),
            maker: crate::order::User::Oscar,
            status: crate::order::Status::Pending,
            side,
            unit_price: match side {
                Side::Ask => low_unit_price,
                Side::Bid => high_unit_price,
            },
            amount: 5,
        };

        let matcheable_order = Order {
            uuid: generate_id(),
            maker: crate::order::User::Joaquin,
            status: crate::order::Status::Pending,
            side: match side {
                Side::Ask => Side::Bid,
                Side::Bid => Side::Ask,
            },
            unit_price: match side {
                Side::Ask => low_unit_price,
                Side::Bid => high_unit_price,
            },
            amount: 5,
        };

        (maker_order, matcheable_order)
    }

    fn success_side_match(side: Side) {
        let (maker_order, matcheable_order) = generate_matching_orders(side);
        let mut available_orders = vec![matcheable_order.clone()];
        let result = find_match(&maker_order, &mut available_orders);
        assert_eq!(result, Some((&maker_order, matcheable_order)));
    }

    #[test]
    fn success_match() {
        success_side_match(Side::Ask);
        success_side_match(Side::Bid);
    }
}
