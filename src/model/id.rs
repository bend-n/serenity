//! A collection of newtypes defining type-strong IDs.

use std::fmt;
use std::num::{NonZeroI64, NonZeroU64};

use super::Timestamp;

#[derive(Debug, Clone, Copy)]
pub struct IDFromStrError;

impl fmt::Display for IDFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attempted to construct an ID of zero")
    }
}

impl std::error::Error for IDFromStrError {}

macro_rules! id_u64 {
    ($($name:ident;)*) => {
        $(
            impl $name {
                #[doc = concat!("Creates a new ", stringify!($name), " from a u64.")]
                /// # Panics
                /// Panics if `id` is zero.
                #[inline]
                #[must_use]
                #[track_caller]
                pub const fn new(id: u64) -> Self {
                    match NonZeroU64::new(id) {
                        Some(inner) => Self(inner),
                        None => panic!(concat!("Attempted to call ", stringify!($name), "::new with invalid (0) value"))
                    }
                }

                /// Retrieves the inner `id` as a [`u64`].
                #[inline]
                #[must_use]
                pub const fn get(self) -> u64 {
                    self.0.get()
                }

                #[doc = concat!("Retrieves the time that the ", stringify!($name), " was created.")]
                #[must_use]
                pub fn created_at(&self) -> Timestamp {
                    Timestamp::from_discord_id(self.get())
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self(NonZeroU64::MIN)
                }
            }

            // This is a hack so functions can accept iterators that either:
            // 1. return the id itself (e.g: `MessageId`)
            // 2. return a reference to it (`&MessageId`).
            impl AsRef<$name> for $name {
                fn as_ref(&self) -> &Self {
                    self
                }
            }

            impl<'a> From<&'a $name> for $name {
                fn from(id: &'a $name) -> $name {
                    id.clone()
                }
            }

            impl From<u64> for $name {
                fn from(id: u64) -> $name {
                    $name::new(id)
                }
            }

            impl From<NonZeroU64> for $name {
                fn from(id: NonZeroU64) -> $name {
                    $name(id)
                }
            }

            impl PartialEq<u64> for $name {
                fn eq(&self, u: &u64) -> bool {
                    self.get() == *u
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let inner = self.0;
                    fmt::Display::fmt(&inner, f)
                }
            }

            impl From<$name> for NonZeroU64 {
                fn from(id: $name) -> NonZeroU64 {
                    id.0
                }
            }

            impl From<$name> for NonZeroI64 {
                fn from(id: $name) -> NonZeroI64 {
                    unsafe {NonZeroI64::new_unchecked(id.get() as i64)}
                }
            }

            impl From<$name> for u64 {
                fn from(id: $name) -> u64 {
                    id.get()
                }
            }

            impl From<$name> for i64 {
                fn from(id: $name) -> i64 {
                    id.get() as i64
                }
            }

            impl std::str::FromStr for $name {
                type Err = IDFromStrError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    #[cfg(debug_assertions)]
                    { return Ok(Self(s.parse::<u64>().ok().and_then(NonZeroU64::new).ok_or(IDFromStrError)?)) }
                    #[cfg(not(debug_assertions))]
                    { return Ok(Self(snowflake::parse(s).ok_or(IDFromStrError)?)) }
                }
            }

            #[cfg(feature = "typesize")]
            impl typesize::TypeSize for $name {}
        )*
    }
}

/// An identifier for an Application.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ApplicationId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a Channel
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ChannelId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an Emoji
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct EmojiId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an unspecific entity.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
// TODO: replace occurences of `#[serde(with = "snowflake")] u64` in the codebase with GenericId
pub struct GenericId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a Guild
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct GuildId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an Integration
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct IntegrationId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a Message
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct MessageId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a Role
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct RoleId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an auto moderation rule
#[repr(packed)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RuleId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a Scheduled Event
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ScheduledEventId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a User
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct UserId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a [`Webhook`][super::webhook::Webhook]
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct WebhookId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an audit log entry.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AuditLogEntryId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an attachment.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AttachmentId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a sticker.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct StickerId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a sticker pack.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct StickerPackId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a sticker pack banner.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct StickerPackBannerId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a SKU.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SkuId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an interaction.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct InteractionId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a slash command.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct CommandId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a slash command permission Id. Can contain
/// a [`RoleId`] or [`UserId`].
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct CommandPermissionId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a slash command version Id.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct CommandVersionId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a slash command target Id. Can contain
/// a [`UserId`] or [`MessageId`].
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct TargetId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a stage channel instance.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct StageInstanceId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for a forum tag.
#[repr(packed)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ForumTagId(#[serde(with = "snowflake")] NonZeroU64);

/// An identifier for an entitlement.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct EntitlementId(#[serde(with = "snowflake")] pub NonZeroU64);

id_u64! {
    AttachmentId;
    ApplicationId;
    ChannelId;
    EmojiId;
    GenericId;
    GuildId;
    IntegrationId;
    MessageId;
    RoleId;
    ScheduledEventId;
    StickerId;
    StickerPackId;
    StickerPackBannerId;
    SkuId;
    UserId;
    WebhookId;
    AuditLogEntryId;
    InteractionId;
    CommandId;
    CommandPermissionId;
    CommandVersionId;
    TargetId;
    StageInstanceId;
    RuleId;
    ForumTagId;
    EntitlementId;
}

/// An identifier for a Shard.
///
/// This identifier is special, it simply models internal IDs for type safety,
/// and therefore cannot be [`Serialize`]d or [`Deserialize`]d.
#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ShardId(pub u32);

impl fmt::Display for ShardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Used with `#[serde(with|deserialize_with|serialize_with)]`
///
/// # Examples
///
/// ```rust,ignore
/// #[derive(Deserialize, Serialize)]
/// struct A {
///     #[serde(with = "snowflake")]
///     id: u64,
/// }
///
/// #[derive(Deserialize)]
/// struct B {
///     #[serde(deserialize_with = "snowflake::deserialize")]
///     id: u64,
/// }
///
/// #[derive(Serialize)]
/// struct C {
///     #[serde(serialize_with = "snowflake::serialize")]
///     id: u64,
/// }
/// ```
pub(crate) mod snowflake {
    use std::fmt;
    use std::num::NonZeroU64;

    use serde::de::{Error, Visitor};
    use serde::{Deserializer, Serializer};

    fn simple_parse(mut n: u64, s: &[u8]) -> u64 {
        for &b in s {
            n = n * 10 + (b - b'0') as u64;
        }
        n
    }

    fn parse_eight(s: [u8; 8]) -> u64 {
        // reinterpret as u64 ("92233721" => 92233721)
        let n = u64::from_le_bytes(s);
        // combine 4 pairs of single digits:
        // split pieces into odd and even
        //  1_7_3_2_ (le repr)
        // _2_3_2_9
        // then combine
        // _21_37_23_92 (le repr, each byte as 2 digits)
        let n = ((n & 0x0f000f000f000f00) >> 8) + ((n & 0x000f000f000f000f) * 10);
        // combine 2 pairs of 2 digits:
        // split again
        // _21___23__
        // ___37___92
        // then combine
        // __14|137__36|7 (le repr, pipes separating bytes)
        let n = ((n & 0x00ff000000ff0000) >> 16) + ((n & 0x000000ff000000ff) * 100);
        // combine pair of 4 digits
        // split again
        // __14|137____ (then moved to ______14|137, as u64:3721)
        // ______36|07 (as u64: 9223)
        // then combine
        ((n & 0x0000ffff00000000) >> 32) + ((n & 0x000000000000ffff) * 10000)
    }

    // this parse is 4x faster than [`str::parse`], see <https://github.com/serenity-rs/serenity/pull/2677#issue-2060912973>
    pub fn parse(x: &(impl AsRef<[u8]> + ?Sized)) -> Option<NonZeroU64> {
        NonZeroU64::new(match pieced::as_with_rest(x.as_ref()) {
            // 16, ..4 (most flakes are length 18, so this is on top)
            (&[a, b], rest) => simple_parse(parse_eight(a) * 100000000 + parse_eight(b), rest),
            // 8, 4..8
            (&[a], rest) => simple_parse(parse_eight(a), rest),
            // largest number is 20 digits, if this is a problem you have bigger problems
            (_, rest) => simple_parse(0, rest),
        })
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NonZeroU64, D::Error> {
        deserializer.deserialize_any(SnowflakeVisitor)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(id: &NonZeroU64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&id.get())
    }

    struct SnowflakeVisitor;

    impl<'de> Visitor<'de> for SnowflakeVisitor {
        type Value = NonZeroU64;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a non-zero string or integer snowflake")
        }

        // Called by formats like TOML.
        fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
            self.visit_u64(u64::try_from(value).map_err(Error::custom)?)
        }

        fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
            NonZeroU64::new(value).ok_or_else(|| Error::custom("invalid value, expected non-zero"))
        }

        fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
            parse(value).ok_or(Error::custom("invalid value, expected non-zero"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use super::GuildId;

    #[test]
    fn test_created_at() {
        // The id is from discord's snowflake docs
        let id = GuildId::new(175928847299117063);
        assert_eq!(id.created_at().unix_timestamp(), 1462015105);
        assert_eq!(id.created_at().to_string(), "2016-04-30T11:18:25.796Z");
    }

    #[test]
    fn test_id_serde() {
        use serde::{Deserialize, Serialize};

        use super::snowflake;
        use crate::json::{assert_json, json};

        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct S {
            #[serde(with = "snowflake")]
            id: NonZeroU64,
        }

        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct Opt {
            id: Option<GuildId>,
        }

        let id = GuildId::new(17_5928_8472_9911_7063);
        assert_json(&id, json!("175928847299117063"));

        let s = S {
            id: NonZeroU64::new(17_5928_8472_9911_7063).unwrap(),
        };
        assert_json(&s, json!({"id": "175928847299117063"}));

        let s = Opt {
            id: Some(GuildId::new(17_5928_8472_9911_7063)),
        };
        assert_json(&s, json!({"id": "175928847299117063"}));
    }
}
