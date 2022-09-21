use crate::domain::entities::*;
use crate::repositories::student::{Insert, Repository};

struct Request {
    number: u16,
    name: String,
    colors: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

fn execute (repo: &mut dyn Repository, req: Request) -> Response {
    match (
        StudentId::try_from(req.number),
        StudentName::try_from(req.name),
        StudentColors::try_from(req.colors),
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
    use crate::repositories::student::InMemoryRepository;

    #[test]
    fn return_student_id() {
        let mut repo = InMemoryRepository::new();
        let number = 13;
        let req = Request {
            number,
            name: String::from("Pedro"),
            colors: vec![String::from("Blue")],
        };

        let result = execute(&mut repo, req);

        match result {
            Response::Ok(result) => assert_eq!(result, number),
            _ => unreachable!(),            
        }
    }

    #[test]
    fn return_error_when_request_is_invalid() {
        let mut repo = InMemoryRepository::new();
        let req = Request {
            number: 13,
            name: String::from(""),
            colors: vec![String::from("Blue")],
        };

        let result = execute(&mut repo, req);

        match result {
            Response::BadRequest => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn return_a_conflict_error_when_student_number_already_exists() {
        let number = StudentId::try_from(25).unwrap();
        let name = StudentName::try_from(String::from("Pedro")).unwrap();
        let types = StudentColors::try_from(vec![String::from("Blue")]).unwrap();
        let mut repo = InMemoryRepository::new();
        repo.insert(number, name, types);
        let req = Request {
            number: 25,
            name: String::from("Juan"),
            colors: vec![String::from("Red")],
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