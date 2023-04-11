// @generated automatically by Diesel CLI.
diesel::table! {
    products (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        price -> Unsigned<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}