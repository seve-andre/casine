// @generated automatically by Diesel CLI.

diesel::table! {
    apartments (id) {
        id -> Integer,
        house_name -> Char,
        apartment_number -> Integer,
    }
}

diesel::table! {
    apartments_prices (id) {
        id -> Integer,
        apartment_id -> Integer,
        rent_month -> Integer,
        price -> Decimal,
    }
}

diesel::table! {
    documents (id) {
        id -> Integer,
        leader_id -> Integer,
        doc_type -> Varchar,
        doc_number -> Varchar,
        birthplace -> Text,
        released_by -> Text,
        residence -> Text,
    }
}

diesel::table! {
    group_members (guest_id, group_id) {
        guest_id -> Integer,
        group_id -> Integer,
    }
}

diesel::table! {
    groupz (id) {
        id -> Integer,
        nickname -> Nullable<Varchar>,
    }
}

diesel::table! {
    guests (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        birth_date -> Date,
        phone_number -> Nullable<Text>,
        nationality -> Nullable<Text>,
    }
}

diesel::table! {
    houses (house_name) {
        house_name -> Char,
        street_type -> Text,
        street_name -> Text,
        street_number -> Integer,
    }
}

diesel::table! {
    rents (id) {
        id -> Integer,
        apartment_id -> Integer,
        group_id -> Integer,
        start_date -> Date,
        end_date -> Date,
    }
}

diesel::joinable!(apartments -> houses (house_name));
diesel::joinable!(apartments_prices -> apartments (apartment_id));
diesel::joinable!(documents -> guests (leader_id));
diesel::joinable!(group_members -> groupz (group_id));
diesel::joinable!(group_members -> guests (guest_id));
diesel::joinable!(rents -> apartments (apartment_id));
diesel::joinable!(rents -> groupz (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    apartments,
    apartments_prices,
    documents,
    group_members,
    groupz,
    guests,
    houses,
    rents,
);
