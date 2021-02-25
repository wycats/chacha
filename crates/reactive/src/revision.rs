use chacha_macros::chacha_value;

#[chacha_value]
pub struct Revision {
    timestamp: u64,
}

impl Revision {
    pub(crate) fn next(self) -> Revision {
        Revision {
            timestamp: self.timestamp + 1,
        }
    }

    pub(crate) fn is_newer_than(self, revision: Revision) -> bool {
        self.timestamp > revision.timestamp
    }
}

macro_rules! numeric_revision {
    ($($ty:ty)*) => {
        $(
            impl From<$ty> for Revision {
                fn from(timestamp: $ty) -> Self {
                    Revision {
                        timestamp: timestamp as u64,
                    }
                }
            }
        )*
    };
}

numeric_revision!(u8 u16 u32 u64 i8 i16 i32 i64);
