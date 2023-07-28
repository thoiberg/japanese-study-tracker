// auto-generated by prost-build, but I've just copied it, since I didn't want to
// add a build step into the system for such a small thing.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeckListInfo {
    #[prost(message, optional, tag = "1")]
    pub all_decks_info: ::core::option::Option<AllDecksInfo>,
    #[prost(int32, tag = "2")]
    pub some_date_probably: i32,
    #[prost(int32, tag = "3")]
    pub user_id_maybe: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllDecksInfo {
    #[prost(message, repeated, tag = "3")]
    pub decks: ::prost::alloc::vec::Vec<DeckInfo>,
    #[prost(int32, optional, tag = "6")]
    pub all_decks_review_card_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub all_decks_new_card_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeckInfo {
    #[prost(int32, tag = "1")]
    pub some_date: i32,
    #[prost(string, tag = "2")]
    pub deck_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub deck_id_probably: i32,
    #[prost(int32, optional, tag = "6")]
    pub review_card_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub new_card_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub also_new_card_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub also_review_card_count: ::core::option::Option<i32>,
    #[prost(int32, tag = "13")]
    pub total_card_count: i32,
}