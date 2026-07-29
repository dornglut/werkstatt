use std::fmt;

use uuid::Uuid;

macro_rules! typed_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct $name(Uuid);

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

typed_id!(ProjectId);
typed_id!(WorkItemId);
typed_id!(ContractId);
typed_id!(AuthorityObservationId);
typed_id!(ExecutionId);
typed_id!(EvidenceId);
typed_id!(FindingId);
