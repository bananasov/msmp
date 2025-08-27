use serde::{Deserialize, Serialize};
use crate::schemas::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum Notification {
    /// Server started
    #[serde(rename = "notification:server/started")]
    ServerStarted,
    /// Server shutting down
    #[serde(rename = "notification:server/stopping")]
    ServerStopping,
    /// Server save started
    #[serde(rename = "notification:server/saving")]
    ServerSaving,
    /// Server save completed
    #[serde(rename = "notification:server/saved")]
    ServerSaved,
    /// Player joined
    #[serde(rename = "notification:players/joined")]
    PlayersJoined(PlayersJoinedParams),
    /// Player left
    #[serde(rename = "notification:players/left")]
    PlayersLeft(PlayersLeftParams),
    /// Player was oped
    #[serde(rename = "notification:operators/added")]
    OperatorsAdded(OperatorsAddedParams),
    /// Player was deoped
    #[serde(rename = "notification:operators/removed")]
    OperatorsRemoved(OperatorsRemovedParams),
    /// Player was added to allowlist
    #[serde(rename = "notification:allowlist/added")]
    AllowlistAdded(AllowlistAddedParams),
    /// Player was removed from allowlist
    #[serde(rename = "notification:allowlist/removed")]
    AllowlistRemoved(AllowlistRemovedParams),
    /// Ip was added to ip ban list
    #[serde(rename = "notification:ip_bans/added")]
    IpBansAdded(IpBansAddedParams),
    /// Ip was removed from ip ban list
    #[serde(rename = "notification:ip_bans/removed")]
    IpBansRemoved(IpBansRemovedParams),
    /// Player was added to ban list
    #[serde(rename = "notification:bans/added")]
    BansAdded(BansAddedParams),
    /// Player was removed from ban list
    #[serde(rename = "notification:bans/removed")]
    BansRemoved(BansRemovedParams),
    /// Gamerule was changed
    #[serde(rename = "notification:gamerules/updated")]
    GamerulesUpdated(GamerulesUpdatedParams),
    /// Server status heartbeat
    #[serde(rename = "notification:server/status")]
    ServerStatus(ServerStatusParams),
}

/// Parameters for player joined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayersJoinedParams {
    pub player: Player,
}

/// Parameters for player left
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayersLeftParams {
    pub player: Player,
}

/// Parameters for player was oped
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsAddedParams {
    pub player: Operator,
}

/// Parameters for player was deoped
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsRemovedParams {
    pub player: Operator,
}

/// Parameters for player was added to allowlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistAddedParams {
    pub player: Player,
}

/// Parameters for player was removed from allowlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistRemovedParams {
    pub player: Player,
}

/// Parameters for ip was added to ip ban list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansAddedParams {
    pub player: IpBan,
}

/// Parameters for ip was removed from ip ban list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansRemovedParams {
    pub player: String,
}

/// Parameters for player was added to ban list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansAddedParams {
    pub player: UserBan,
}

/// Parameters for player was removed from ban list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansRemovedParams {
    pub player: Player,
}

/// Parameters for gamerule was changed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamerulesUpdatedParams {
    pub gamerule: TypedGameRule,
}

/// Parameters for server status heartbeat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatusParams {
    pub status: ServerState,
}

