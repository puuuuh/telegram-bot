use crate::requests::*;
use crate::types::*;

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in
/// the supergroup for this to work and must have the appropriate admin rights.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RestrictChatMember {
    chat_id: ChatRef,
    user_id: UserId,
    until_date: Option<Integer>,
    permissions: ChatPermissions,
}

impl Request for RestrictChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("restrictChatMember"), self)
    }
}

impl RestrictChatMember {
    pub fn new<C, U>(chat: C, user: U, permissions: ChatPermissions) -> Self
    where
        C: ToChatRef,
        U: ToUserId,
    {
        RestrictChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
            until_date: None,
            permissions,
        }
    }

    pub fn until_date(&mut self, value: Integer) -> &mut Self {
        self.until_date = Some(value);
        self
    }
}

/// Restrict a user in a supergroup.
pub trait CanRestrictChatMemberForChat {
    fn restrict<O>(&self, other: O, permissions: ChatPermissions) -> RestrictChatMember
    where
        O: ToUserId;
}

impl<C> CanRestrictChatMemberForChat for C
where
    C: ToChatRef,
{
    fn restrict<O>(&self, other: O, permissions: ChatPermissions) -> RestrictChatMember
    where
        O: ToUserId,
    {
        RestrictChatMember::new(self, other, permissions)
    }
}

/// Restrict a user in a supergroup.
pub trait CanRestrictChatMemberForUser {
    fn restrict_in<O>(&self, other: O, permissions: ChatPermissions) -> RestrictChatMember
    where
        O: ToChatRef;
}

impl<U> CanRestrictChatMemberForUser for U
where
    U: ToUserId,
{
    fn restrict_in<O>(&self, other: O, permissions: ChatPermissions) -> RestrictChatMember
    where
        O: ToChatRef,
    {
        RestrictChatMember::new(other, self, permissions)
    }
}
