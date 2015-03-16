pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<'a> AsBytes for &'a str {
    fn as_bytes(&self) -> &[u8] { <str as StrExt>::as_bytes(*self) }
}

impl<'a> AsBytes for &'a [u8] {
    fn as_bytes(&self) -> &[u8] { self }
}

impl AsBytes for String {
    fn as_bytes(&self) -> &[u8] { self[..].as_bytes() }
}

impl AsBytes for Vec<u8> {
    fn as_bytes(&self) -> &[u8] { &self[..] }
}
