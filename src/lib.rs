extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_kms;

pub enum Error {}

trait KeyService {}

pub enum Status {
    Created(usize),
}

// dynamodb
// table = credential-store
//
//

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
