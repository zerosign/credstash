use futures::{Future, Stream};

trait KeyService {
    fn decrypt(&self, buffer: &[u8]) -> Future<Item, Error> ;
    fn encrypt<T>(&self, T) where ;
}

pub struct FilterBuilder {
    fn build(&self) -> Cursor<Item = Self::Item, Error = Error>;
}

pub struct Cursor {}

trait Repository {
    type Item;

    fn fetch<S>(&self, name: S) -> Future<Item = Self::Item, Error = Error>
    where
        S: Into<String>;

    fn store<S>(&self, secret: Self::Item) -> Future<Item = (), Error = Error>
    where
        S: Into<String>;

    fn delete<S>(&self, name: S) -> Future<Item = (), Error = Error> where S: Into<String>;

    fn filter<S>(&self, name: S) -> FilterBuilder<Item = Self::Item, Error = Error> where S: Into<String>;
}

trait Backend {
    fn setup(&self) -> Future<Item = Status, Error = Error>;
    fn status(&self) -> Future<Item = Status, Error = Error>;
}
