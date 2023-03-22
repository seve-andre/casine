// @generated automatically by Diesel CLI.
diesel::table! {
    houses (id) {
        id -> Integer,
        house_name -> Text,
        street_type -> Text,
        street_name -> Text,
        street_number -> Integer,
    }
}

diesel::table! {
    apartments (id) {
        id -> Integer,
        house_name -> Text,
        apartment_number -> Integer,
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
    groups (id) {
        id -> Integer,
        nickname -> Nullable<Text>,
    }
}

diesel::table! {
    group_members (guest_id, group_id) {
        guest_id -> Integer,
        group_id -> Integer,
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

diesel::table! {
    apartments_price (id) {
        id -> Integer,
        apartment_id -> Integer,
        rent_month -> Integer,
        price -> Double,
    }
}

diesel::table! {
    documents (id) {
        id -> Integer,
        leader_id -> Integer,
        doc_type -> Text,
        doc_number -> Text,
        birthplace -> Text,
        released_by -> Text,
        residence -> Text,
    }
}

diesel::joinable!(apartments_price -> apartments (apartment_id));
diesel::joinable!(documents -> guests (leader_id));
diesel::joinable!(group_members -> groups (group_id));
diesel::joinable!(group_members -> guests (guest_id));
diesel::joinable!(rents -> apartments (apartment_id));
diesel::joinable!(rents -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    houses,
    apartments,
    guests,
    groups,
    group_members,
    rents,
    apartments_price,
    documents,
);
