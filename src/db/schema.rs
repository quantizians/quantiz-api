table! {
  tasks (id) {
      id -> Uuid,
      title -> Varchar,
      details -> Nullable<Text>,
      created_date -> Timestamp,
      deadline -> Nullable<Timestamp>,
      priority -> Nullable<Int2>,
      persistent -> Nullable<Bool>,
      completed -> Nullable<Bool>,
      supertask -> Nullable<Uuid>,
  }
}