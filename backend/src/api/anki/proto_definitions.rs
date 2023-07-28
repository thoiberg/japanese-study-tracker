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
    #[prost(uint32, optional, tag = "6")]
    pub all_decks_review_card_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub all_decks_new_card_count: ::core::option::Option<u32>,
}
/// I think it corresponds to this??? <https://github.com/ankitects/anki/blob/1d7559819ca3520898247585bfcac96904737bec/proto/anki/decks.proto#L147>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeckInfo {
    #[prost(int64, tag = "1")]
    pub deck_id: i64,
    #[prost(string, tag = "2")]
    pub deck_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub level: u32,
    #[prost(uint32, optional, tag = "6")]
    pub review_card_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub learn_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub new_card_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub uncapped_new_card_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub uncapped_review_card_count: ::core::option::Option<u32>,
    #[prost(uint32, tag = "13")]
    pub total_card_count: u32,
}
