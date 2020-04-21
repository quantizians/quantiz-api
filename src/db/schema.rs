table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        details -> Nullable<Text>,
        created_date -> Timestamptz,
        updated_date -> Nullable<Timestamptz>,
        deadline -> Nullable<Timestamptz>,
        completed_date -> Nullable<Timestamptz>,
        priority -> Int2,
        persistent -> Bool,
        supertask -> Nullable<Uuid>,
    }
}
