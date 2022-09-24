use crate::domain::entities::*;
use crate::repositories::order::{Insert, Repository};

struct Request {
    order_id: u16,
    description: String,
    order_type: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

fn execute (repo: &mut dyn Repository, req: Request) -> Response {
    match (
        OrderId::try_from(req.order_id),
        OrderDescription::try_from(req.description),
        OrderTypes::try_from(req.order_type),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            Insert::Ok(number) => Response::Ok(u16::from(number)),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::order::InMemoryRepository;

    #[test]
    fn return_order_id() {
        let mut repo = InMemoryRepository::new();
        let order_id = 13;
        let req = Request {
            order_id,
            description: String::from("Pedro"),
            order_type: vec![String::from("Blue")],
        };

        let result = execute(&mut repo, req);

        match result {
            Response::Ok(result) => assert_eq!(result, order_id),
            _ => unreachable!(),            
        }
    }

    #[test]
    fn return_error_when_request_is_invalid() {
        let mut repo = InMemoryRepository::new();
        let req = Request {
            order_id: 13,
            description: String::from(""),
            order_type: vec![String::from("Blue")],
        };

        let result = execute(&mut repo, req);

        match result {
            Response::BadRequest => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn return_a_conflict_error_when_order_id_already_exists() {
        let order_id = OrderId::try_from(25).unwrap();
        let description = OrderDescription::try_from(String::from("Pedro")).unwrap();
        let order_types = OrderTypes::try_from(vec![String::from("Blue")]).unwrap();
        let mut repo = InMemoryRepository::new();
        repo.insert(order_id, description, order_types);
        let req = Request {
            order_id: 25,
            description: String::from("Juan"),
            order_type: vec![String::from("Red")],
        };
    
        let res = execute(&mut repo, req);
    
        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn return_error_when_unexpected_error_happens() {

    }
}