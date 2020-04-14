use diesel::{
  backend::Backend,
  sql_types::{SmallInt},
  deserialize::{self, FromSql},
  serialize::{self, Output, ToSql},
};
use std::io::Write;
use serde::{Serialize, Deserialize};


#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[sql_type = "SmallInt"]
pub enum Priority {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Urgent = 4,
}

impl<DB> ToSql<SmallInt, DB> for Priority
where
  DB: Backend,
  i16: ToSql<SmallInt, DB>,
{
  fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
    return (*self as i16).to_sql(out);
  }
}

impl<DB> FromSql<SmallInt, DB> for Priority
where
  DB: Backend,
  i16: FromSql<SmallInt, DB>,
{
  fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
    return match i16::from_sql(bytes)? {
      0 => Ok(Priority::None),
      1 => Ok(Priority::Low),
      2 => Ok(Priority::Medium),
      3 => Ok(Priority::High),
      4 => Ok(Priority::Urgent),
      int => Err(format!("Invalid Priority of {}", int).into()),
    };
  }
}

// expression::{helper_types::AsExprOf, AsExpression},
// impl AsExpression<SmallInt> for OrderStatus {
//   type Expression = AsExprOf<i16, SmallInt>;

//   fn as_expression(self) -> Self::Expression {
//       <i16 as AsExpression<SmallInt>>::as_expression(self as i16)
//   }
// }


// impl Serialize for Priority {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where S: Serializer,
//   {
    
//   }
// }