use diesel::{
    connection::{AnsiTransactionManager},
    Connection, SqliteConnection, RunQueryDsl, backend::{UsesAnsiSavepointSyntax, Backend}, types::TypeMetadata,
};
use diesel_generic_demo::{models::CarNew, schema::cars};

fn main() {
    let connection = SqliteConnection::establish("/tmp/test.db").unwrap();
    //let connection = PgConnection::establish("/tmp/test.db").unwrap();
    let gdc = GenericDatabaseConnector::new(connection);
    gdc.create_and_read();
}

pub struct GenericDatabaseConnector<T, U>
where
    T: Connection<Backend = U, TransactionManager = AnsiTransactionManager>,
    U: UsesAnsiSavepointSyntax + TypeMetadata + Backend + 'static
{
    connection: T,
}

impl<T, U> GenericDatabaseConnector<T, U>
where
    T: Connection<Backend = U, TransactionManager = AnsiTransactionManager>,
    U: UsesAnsiSavepointSyntax + TypeMetadata + Backend
{
    fn new(connection: T) -> GenericDatabaseConnector<T, U> {
        GenericDatabaseConnector { connection }
    }
    fn create_and_read(&self) {

        let new_car = CarNew {
            name:"Fiat Multipla"
        };

        diesel::insert_into(cars::table)
            .values(&new_car)
            .execute(&self.connection).unwrap();
    }
}
