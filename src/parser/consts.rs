pub const ANY: &[u8] = &[];
pub const WHITE_SPACE: &[u8] = " \n\n\r/#".as_bytes();
pub const COMMENTS: &[u8] = "/#".as_bytes();

// legacy (to remove)

pub const TODO_KEY: &[u8] = "@todo".as_bytes();

pub const AT_KEY: &u8 = &"@".as_bytes()[0];
// pub const HIGH: &[u8] = "high".as_bytes();
// pub const LOW: &[u8] = "low".as_bytes();

pub const MSG_START: &u8 = &"{".as_bytes()[0];
pub const MSG_END: &u8 = &"}".as_bytes()[0];

pub const REF_START: &u8 = &"[".as_bytes()[0];
pub const REF_END: &u8 = &"]".as_bytes()[0];
