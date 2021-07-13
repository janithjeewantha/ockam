/// Creates a [`CredentialSchema`] containing the arguments.
///
/// ```
/// # use ockam_entity::credential_type;;
/// let schema = credential_type!["TYPE_ID"; "attribute_of_string_type", (Number, "attribute_of_number_type")];
/// ```
///
/// [`CredentialSchema`]: crate::CredentialSchema
#[macro_export]
macro_rules! credential_type {
    ($t:expr; $($x:expr),* $(,)?) => ({
        use $crate::CredentialAttributeSchema;
        use $crate::CredentialAttributeType::{Number, Utf8String, Blob};

        #[allow(unused_mut)]
        let mut attributes = vec![];
        $(attributes.push($x.into());)*

        $crate::CredentialSchema {
            id: $t.into(),
            label: String::new(),
            description: String::new(),
            attributes,
        }
    });
}
