table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        details -> Nullable<Text>,
        created_timestamp -> Timestamptz,
        updated_timestamp -> Nullable<Timestamptz>,
        deadline -> Nullable<Timestamptz>,
        completed_timestamp -> Nullable<Timestamptz>,
        priority -> Int2,
        persistent -> Bool,
        supertask -> Nullable<Uuid>,
    }
}
