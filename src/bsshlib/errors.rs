pub const BSSH_ERR_NO_LINE_TERMINATION_FOUND        : &'static str = "Error while parsing initial message: no proper line termination found (and reached maximum string length).";

pub const BSSH_ERR_NOT_UTF8_STRING                  : &'static str = "Error while parsing initial message: not utf8 string.";
pub const BSSH_ERR_EXPECTED_HEADER_STRING           : &'static str = "Error while parsing initial message: expected SSH-2.0-... string.";
pub const BSSH_ERR_TOO_MANY_COMMENT_LINES           : &'static str = "Error while parsing initial message: too many additional (comment) lines.";

pub const BSSH_ERR_CONNECTION_ENDED_UNEXPECTEDLY    : &'static str = "Connection ended unexpectedly.";

pub const BSSH_ERR_BUFFER_CAPACITY_EXCEEDED         : &'static str = "Buffer capacity exceeded.";
pub const BSSH_ERR_MALFORMED_NAME_LIST              : &'static str = "Malformed name list.";

pub const BSSH_ERR_EXPECTED_KEX_MSG_INIT			: &'static str = "Error while processing key exchange packet: expected KEX_MSG_INIT in header";
pub const BSSH_ERR_EXPECTED_ZERO_U32				: &'static str = "Error while processing key exchange packet: expected 0 (u32) at the end of message";

pub const BSSH_DH_ERR_EXPECTED_KEXDH_INIT			: &'static str = "Error in kex exchange: expected KEXDH_INIT.";
pub const BSSH_DH_ERR_NEGATIVE_E					: &'static str = "Error in kex exchange: e number is negative!";