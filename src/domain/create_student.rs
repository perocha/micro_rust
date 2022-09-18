use crate::domain::entities::*;

struct Request {
    number: u16,
    name: String,
    colors: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
}

fn execute (req: Request) -> Response {
    match (
        StudentId::try_from(req.number),
        StudentName::try_from(req.name),
        StudentColors::try_from(req.colors),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(u16::from(number)),
        _ => Response::BadRequest,
    }
}

pub fn hello_world () {
    println!("create_something::Hello world!!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_id() {
        let number = 13;
        let req = Request {
            number,
            name: String::from("Pedro"),
            colors: vec![String::from("Blue")],
        };

        let result = execute(req);

        match result {
            Response::Ok(result) => assert_eq!(result, number),
            _ => unreachable!(),            
        }
    }

    #[test]
    fn return_error_when_request_is_invalid() {
        let req = Request {
            number: 13,
            name: String::from(""),
            colors: vec![String::from("Blue")],
        };

        let result = execute(req);

        match result {
            Response::BadRequest => {}
            _ => unreachable!(),
        };
    }
}