//! Model struct for Customer type

/// This is a model struct for Customer type.

use serde::Deserialize;

use super::{Card, Address, CustomerPreferences, CustomerTaxIds};

#[derive(Debug, Default, Deserialize, Hash, PartialEq)]
pub struct Customer {
    /// A unique Square-assigned ID for the customer profile. If you need this ID for an API
    /// request, use the ID returned when you created the customer profile or call the
    /// [SearchCustomers]($e/Customers/SearchCustomers) or
    /// [ListCustomers]($e/Customers/ListCustomers) endpoint.
    pub id: Option<String>,
    /// The timestamp when the customer profile was created, in RFC 3339 format.
    pub created_at: Option<String>,
    /// The timestamp when the customer profile was last updated, in RFC 3339 format.
    pub updated_at: Option<String>,
    /// Payment details of the credit, debit, and gift cards stored on file for the customer
    /// profile. DEPRECATED at version 2021-06-16. Replaced by calling
    /// [ListCards]($e/Cards/ListCards) (for credit and debit cards on file) or
    /// [ListGiftCards]($e/GiftCards/ListGiftCards) (for gift cards on file) and including the
    /// `customer_id` query parameter. For more information, see [Migrate to the Cards API and Gift
    /// Cards
    /// API](https://developer.squareup.com/docs/customers-api/use-the-api/integrate-with-other-services#migrate-customer-cards).
    pub cards: Option<Vec<Card>>,
    /// The given (i.e., first) name associated with the customer profile.
    pub given_name: Option<String>,
    /// The family (i.e., last) name associated with the customer profile.
    pub family_name: Option<String>,
    /// A nickname for the customer profile.
    pub nickname: Option<String>,
    /// A business name associated with the customer profile.
    pub company_name: Option<String>,
    /// The email address associated with the customer profile.
    pub email_address: Option<String>,
    /// Represents a postal address in a country. For more information, see [Working with
    /// Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses).
    pub address: Option<Address>,
    /// The 11-digit phone number associated with the customer profile.
    pub phone_number: Option<String>,
    /// The birthday associated with the customer profile, in RFC 3339 format. The year is optional.
    /// The timezone and time are not allowed. For example, `0000-09-21T00:00:00-00:00` represents a
    /// birthday on September 21 and `1998-09-21T00:00:00-00:00` represents a birthday on September
    /// 21, 1998.
    pub birthday: Option<String>,
    /// An optional second ID used to associate the customer profile with an entity in another
    /// system.
    pub reference_id: Option<String>,
    /// A custom note associated with the customer profile.
    pub note: Option<String>,
    /// Represents communication preferences for the customer profile.
    pub preferences: Option<CustomerPreferences>,
    /// Indicates the method used to create the customer profile.
    pub creation_source: Option<String>,
    /// The IDs of customer groups the customer belongs to.
    pub group_ids: Option<Vec<String>>,
    /// The IDs of segments the customer belongs to.
    pub segment_ids: Option<Vec<String>>,
    /// The Square-assigned version number of the customer profile. The version number is
    /// incremented each time an update is committed to the customer profile, except for changes to
    /// customer segment membership and cards on file.
    pub version: Option<i32>,
    /// Represents the tax ID associated with a [customer profile]($m/Customer). The corresponding
    /// `tax_ids` field is available only for customers of sellers in EU countries or the United
    /// Kingdom. For more information, see [Customer tax
    /// IDs](https://developer.squareup.com/docs/customers-api/what-it-does#customer-tax-ids).
    pub tax_ids: Option<CustomerTaxIds>,
}