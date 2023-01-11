use rand::{
    distributions::{Alphanumeric, Distribution, Standard},
    thread_rng, Rng,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Side {
    Ask,
    Bid,
}

#[derive(Debug, Clone, PartialEq, Copy)]

pub enum Status {
    Pending,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Copy)]

pub enum User {
    Joaquin,
    Oscar,
    Carlos,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    pub uuid: String,
    pub maker: User,
    pub status: Status,
    pub side: Side,
    pub amount: u32,
    pub unit_price: u32,
}

impl Order {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn maker(&self) -> &User {
        &self.maker
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn side(&self) -> &Side {
        &self.side
    }
    pub fn amount(&self) -> &u32 {
        &self.amount
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Order {{uuid: {},maker: {:?},status: {:?},side: {:?},amount: {},unit_price: {}}}",
            self.uuid, self.maker, self.status, self.side, self.amount, self.unit_price
        )
    }
}

impl Distribution<User> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> User {
        match rng.gen_range(0..=2) {
            0 => User::Joaquin,
            1 => User::Carlos,
            _ => User::Oscar,
        }
    }
}

impl Distribution<Side> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
        match rng.gen_range(0..=1) {
            0 => Side::Bid,
            _ => Side::Ask,
        }
    }
}

impl Distribution<Status> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Status {
        match rng.gen_range(0..=2) {
            0 => Status::Filled,
            1 => Status::Cancelled,
            _ => Status::Pending,
        }
    }
}

impl Distribution<Order> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Order {
        Order {
            uuid: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect(),
            maker: rng.gen(),
            //status: rng.gen(),
            status: Status::Pending,
            side: rng.gen(),
            amount: thread_rng().gen_range(1..500),
            unit_price: thread_rng().gen_range(1..10),
        }
    }
}

pub fn generate_random() -> Order {
    rand::thread_rng().gen::<Order>()
}
