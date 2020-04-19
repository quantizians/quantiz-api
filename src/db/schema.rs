table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        details -> Nullable<Text>,
        created_date -> Timestamptz,
        updated_date -> Nullable<Timestamptz>,
        deadline -> Nullable<Timestamptz>,
        priority -> Int2,
        persistent -> Bool,
        completed -> Bool,
        supertask -> Nullable<Uuid>,
    }
}
