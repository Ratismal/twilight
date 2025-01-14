use serde::Serialize;
use twilight_model::{
    channel::message::{
        sticker::{StickerFormatType, StickerId, StickerPackId, StickerType},
        Sticker,
    },
    id::{GuildId, UserId},
};

/// Representation of a cached [`Sticker`].
///
/// [`Sticker`]: twilight_model::channel::message::sticker::Sticker
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedSticker {
    /// Whether the sticker is available.
    pub(crate) available: bool,
    /// Description of the sticker.
    pub(crate) description: String,
    /// Format type.
    pub(crate) format_type: StickerFormatType,
    /// ID of the guild that owns the sticker.
    pub(crate) guild_id: Option<GuildId>,
    /// Unique ID of the sticker.
    pub(crate) id: StickerId,
    /// Kind of sticker.
    pub(crate) kind: StickerType,
    /// Name of the sticker.
    pub(crate) name: String,
    /// Unique ID of the pack the sticker is in.
    pub(crate) pack_id: Option<StickerPackId>,
    /// Sticker's sort order within a pack.
    pub(crate) sort_value: Option<u64>,
    /// CSV list of tags the sticker is assigned to, if any.
    pub(crate) tags: String,
    /// ID of the user that uploaded the sticker.
    pub(crate) user_id: Option<UserId>,
}

impl CachedSticker {
    /// Whether the sticker is available.
    pub const fn available(&self) -> bool {
        self.available
    }

    /// Description of the sticker.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Format type.
    pub const fn format_type(&self) -> StickerFormatType {
        self.format_type
    }

    /// ID of the guild that owns the sticker.
    pub const fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    /// Unique ID of the sticker.
    pub const fn id(&self) -> StickerId {
        self.id
    }

    /// Kind of sticker.
    pub const fn kind(&self) -> StickerType {
        self.kind
    }

    /// Name of the sticker.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Unique ID of the pack the sticker is in.
    pub const fn pack_id(&self) -> Option<StickerPackId> {
        self.pack_id
    }

    /// Sticker's sort order within a pack.
    pub const fn sort_value(&self) -> Option<u64> {
        self.sort_value
    }

    /// CSV list of tags the sticker is assigned to, if any.
    pub fn tags(&self) -> &str {
        &self.tags
    }

    /// ID of the user that uploaded the sticker.
    pub const fn user_id(&self) -> Option<UserId> {
        self.user_id
    }
}

impl PartialEq<Sticker> for CachedSticker {
    fn eq(&self, other: &Sticker) -> bool {
        self.available == other.available
            && self.description.as_str() == other.description.as_ref().map_or("", String::as_str)
            && self.format_type == other.format_type
            && self.guild_id == other.guild_id
            && self.id == other.id
            && self.kind == other.kind
            && self.name == other.name
            && self.pack_id == other.pack_id
            && self.sort_value == other.sort_value
            && self.tags == other.tags
            && self.user_id == other.user.as_ref().map(|user| user.id)
    }
}

#[cfg(test)]
mod tests {
    use super::CachedSticker;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::{
        channel::message::{
            sticker::{StickerFormatType, StickerId, StickerPackId, StickerType},
            Sticker,
        },
        id::{GuildId, UserId},
        user::{PremiumType, User, UserFlags},
    };

    assert_fields!(
        CachedSticker: available,
        description,
        format_type,
        guild_id,
        id,
        kind,
        name,
        pack_id,
        sort_value,
        tags,
        user_id
    );
    assert_impl_all!(CachedSticker: Clone, Debug, Eq, PartialEq);

    #[test]
    fn test_eq_sticker() {
        let sticker = Sticker {
            available: true,
            description: Some("sticker".into()),
            format_type: StickerFormatType::Png,
            guild_id: Some(GuildId::new(1).expect("non zero")),
            id: StickerId::new(2).expect("non zero"),
            kind: StickerType::Guild,
            name: "stick".into(),
            pack_id: Some(StickerPackId::new(3).expect("non zero")),
            sort_value: Some(1),
            tags: "foo,bar,baz".into(),
            user: Some(User {
                accent_color: None,
                avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                banner: None,
                bot: false,
                discriminator: 1,
                email: Some("address@example.com".to_owned()),
                flags: Some(UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER),
                id: UserId::new(1).expect("non zero"),
                locale: Some("en-us".to_owned()),
                mfa_enabled: Some(true),
                name: "test".to_owned(),
                premium_type: Some(PremiumType::Nitro),
                public_flags: Some(
                    UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER,
                ),
                system: Some(true),
                verified: Some(true),
            }),
        };

        let cached = CachedSticker {
            available: true,
            description: "sticker".into(),
            format_type: StickerFormatType::Png,
            guild_id: Some(GuildId::new(1).expect("non zero")),
            id: StickerId::new(2).expect("non zero"),
            kind: StickerType::Guild,
            name: "stick".into(),
            pack_id: Some(StickerPackId::new(3).expect("non zero")),
            sort_value: Some(1),
            tags: "foo,bar,baz".into(),
            user_id: Some(UserId::new(1).expect("non zero")),
        };

        assert_eq!(cached, sticker);
    }
}
