use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Visitor},
    ser::{Error as SerError, Serialize, Serializer},
};
use std::{
    convert::TryInto,
    fmt::{Formatter, Result as FmtResult},
};

bitflags! {
    pub struct Permissions: u128 {
        const CREATE_INVITE = 0x0000_0001;
        const KICK_MEMBERS = 0x0000_0002;
        const BAN_MEMBERS = 0x0000_0004;
        const ADMINISTRATOR = 0x0000_0008;
        const MANAGE_CHANNELS = 0x0000_0010;
        const MANAGE_GUILD = 0x0000_0020;
        const ADD_REACTIONS = 0x0000_0040;
        const VIEW_AUDIT_LOG = 0x0000_0080;
        const PRIORITY_SPEAKER = 0x0000_0100;
        const STREAM = 0x0000_0200;
        const VIEW_CHANNEL = 0x0000_0400;
        const SEND_MESSAGES = 0x0000_0800;
        const SEND_TTS_MESSAGES = 0x0000_1000;
        const MANAGE_MESSAGES = 0x0000_2000;
        const EMBED_LINKS = 0x0000_4000;
        const ATTACH_FILES = 0x0000_8000;
        const READ_MESSAGE_HISTORY = 0x0001_0000;
        const MENTION_EVERYONE = 0x0002_0000;
        const USE_EXTERNAL_EMOJIS = 0x0004_0000;
        const VIEW_GUILD_INSIGHTS = 0x0008_0000;
        const CONNECT = 0x0010_0000;
        const SPEAK = 0x0020_0000;
        const MUTE_MEMBERS = 0x0040_0000;
        const DEAFEN_MEMBERS = 0x0080_0000;
        const MOVE_MEMBERS = 0x0100_0000;
        const USE_VAD = 0x0200_0000;
        const CHANGE_NICKNAME = 0x0400_0000;
        const MANAGE_NICKNAMES = 0x0800_0000;
        const MANAGE_ROLES = 0x1000_0000;
        const MANAGE_WEBHOOKS = 0x2000_0000;
        const MANAGE_EMOJIS = 0x4000_0000;
    }
}

pub(crate) fn serialize_u64<S: Serializer>(
    permissions: &Permissions,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let bits = permissions
        .bits()
        .try_into()
        .map_err(|_| SerError::custom("permission bits can't be a u64"))?;

    serializer.serialize_u64(bits)
}

struct PermissionsVisitor;

impl<'de> Visitor<'de> for PermissionsVisitor {
    type Value = Permissions;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("integer or string permissions")
    }

    fn visit_u128<E: DeError>(self, v: u128) -> Result<Self::Value, E> {
        Ok(Permissions::from_bits_truncate(v))
    }

    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        self.visit_u128(u128::from(v))
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        let num = v
            .parse()
            .map_err(|_| DeError::custom("permissions is not valid bitflags"))?;

        self.visit_u128(num)
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(PermissionsVisitor)
    }
}

impl Serialize for Permissions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.bits().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Permissions;
    use serde_test::Token;

    #[test]
    fn test_permissions() {
        let permissions = Permissions::DEAFEN_MEMBERS;

        serde_test::assert_tokens(&permissions, &[Token::Str("8388608")]);
        // serde_test doesn't support a u128. Only test deserialization here
        // since it serializes into a string.
        serde_test::assert_de_tokens(&permissions, &[Token::U64(8388608)]);
    }
}