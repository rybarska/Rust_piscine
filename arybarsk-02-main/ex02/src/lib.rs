
#[derive(Debug)]
pub enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

impl PizzaStatus {
    pub fn from_delivery_time(ordered_days_ago: u32) -> Self
    {
        match ordered_days_ago
        {
            (0..=1) => PizzaStatus::Ordered,
            (2..=6) => PizzaStatus::Cooking,
            (7..=9) => PizzaStatus::Cooked,
            (10..=16) => PizzaStatus::Delivering,
            (17..) => PizzaStatus::Delivered,
        }
    }

    pub fn get_delivery_time_in_days(&self) -> u32
    {
        match self
        {
            PizzaStatus::Ordered => 17,
            PizzaStatus::Cooking => 15,
            PizzaStatus::Cooked => 10,
            PizzaStatus::Delivering=> 7,
            PizzaStatus::Delivered => 0,
        }
    }
}