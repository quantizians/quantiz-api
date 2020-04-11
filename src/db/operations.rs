use diesel::PgConnection;

mod schema;

// pub fn insert<Item: Crud>(item: Item, connection: &PgConnection) -> bool {

// }

// pub trait Crud {
//   type NewItem;
//   fn all(connection: &PgConnection) -> Vec<Self>;
//   fn insert(item: NewItem, connection: &PgConnection) -> Self {
    
//   }
// }