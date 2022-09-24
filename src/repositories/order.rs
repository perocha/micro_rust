use crate::domain::entities::{Order, OrderId, OrderDescription, OrderTypes};

pub enum Insert {
    Ok(OrderId),
    Conflict,
    Error,
}

pub trait Repository {
    fn insert(&mut self, order_id: OrderId, description: OrderDescription, types: OrderTypes) -> Insert;
}

pub struct InMemoryRepository {
    error: bool,
    orders: Vec<Order>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let orders: Vec<Order> = vec![];
        Self {
            error: false,
            orders,
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

impl Repository for InMemoryRepository {
    fn insert (&mut self, order_id: OrderId, description: OrderDescription, order_types: OrderTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        if self.orders.iter().any(|order| order.order_id == order_id) {
            return Insert::Conflict;
        }

        let order_id_clone = order_id.clone();
        self.orders.push(Order::new(order_id_clone, description, order_types));
        Insert::Ok(order_id)
    }
}