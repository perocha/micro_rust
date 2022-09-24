use std::cmp::PartialEq;

pub struct Order {
    pub order_id: OrderId,
    description: OrderDescription,
    order_types: OrderTypes,
}

impl Order {
    pub fn new (order_id: OrderId, description: OrderDescription, order_types: OrderTypes) -> Self {
        Self {
            order_id,
            description,
            order_types
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct OrderId(u16);

impl TryFrom<u16> for OrderId {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 1000 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<OrderId> for u16 {
    fn from(n: OrderId) -> u16 {
        n.0
    }
}


pub struct OrderDescription(String);

impl TryFrom<String> for OrderDescription {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}


pub struct OrderTypes(Vec<OrderType>);

impl TryFrom<Vec<String>> for OrderTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match OrderType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

enum OrderType {
    Blue,
    Red,
}

impl TryFrom<String> for OrderType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Blue" => Ok(Self::Blue),
            "Red" => Ok(Self::Red),
            _ => Err(()),
        }
    }
}