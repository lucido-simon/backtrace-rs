/// A platform independent representation of a string. When working with `std`
/// enabled it is recommended to the convenience methods for providing
/// conversions to `std` types.
#[derive(Debug)]
pub enum BytesOrWideString<'a> {
    /// A slice, typically provided on Unix platforms.
    Bytes(&'a [u8]),
    /// Wide strings typically from Windows.
    Wide(&'a [u16]),
}

impl<'a> BytesOrWideString<'a> {
    /// Lossy converts to a `Cow<str>`, will allocate if `Bytes` is not valid
    /// UTF-8 or if `BytesOrWideString` is `Wide`.
    ///
    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn to_str_lossy(&self) -> alloc::borrow::Cow<'a, str> {
        use self::BytesOrWideString::*;

        match self {
            Bytes(slice) => alloc::string::String::from_utf8_lossy(slice),
            Wide(wide) => alloc::borrow::Cow::Owned(alloc::string::String::from_utf16_lossy(wide)),
        }
    }
}

impl<'a> core::fmt::Display for BytesOrWideString<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_str_lossy().fmt(f)
    }
}
