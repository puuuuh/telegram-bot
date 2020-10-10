
/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize)]
pub struct ChatPermissions {    
    /// User cam send messages.
    pub can_send_messages: Option<bool>,
    /// User can send media.
    pub can_send_media_messages: Option<bool>,
    /// User can send polls.
    pub can_send_polls: Option<bool>,
    /// User can send animations, games, stickers and use inline bots, implies can_send_media_messages.
    pub can_send_other_messages: Option<bool>,
    /// User can send messages with web previews.
    pub can_add_web_page_previews: Option<bool>,
    /// User can change group info.
    pub can_change_info: Option<bool>,
    /// User can invite other users.
    pub can_invite_users: Option<bool>,
    /// User can pin messages.
    pub can_pin_messages: Option<bool>,
}

