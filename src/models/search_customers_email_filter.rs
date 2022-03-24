//! Model struct for SearchCustomersDateTimeFilter type

use crate::models::SearchCustomersTextFilter;
use serde::Serialize;

/// Filter for `Customer` objects based on A filter to select customers by their email address.
/// This filter is case-insensitive.
///
/// **Important:** There are 2 kinds of searches available for phone_numbers: Exact and Fuzzy.
/// The details regarding it are available on the link below.
/// (https://developer.squareup.com/reference/square/customers/search-customers).
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct SearchCustomersEmailAddressFilter {
    /// The time range for filtering on the `created_at` timestamp. If you use this value, you must
    /// set the `sort_field` in the `CustomersSearchSort` object to `CREATED_AT`.
    pub email_address: Option<SearchCustomersTextFilter>,
}