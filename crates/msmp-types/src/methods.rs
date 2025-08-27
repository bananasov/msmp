use serde::{Deserialize, Serialize};
use crate::schemas::*;

/// Get the allowlist
/// 
/// Method: `minecraft:allowlist`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistResponse {
    pub allowlist: Vec<Player>,
}

/// Set the allowlist
/// 
/// Method: `minecraft:allowlist/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistSetRequest {
    pub players: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistSetResponse {
    pub allowlist: Vec<Player>,
}

/// Add players to allowlist
/// 
/// Method: `minecraft:allowlist/add`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistAddRequest {
    pub add: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistAddResponse {
    pub allowlist: Vec<Player>,
}

/// Remove players from allowlist
/// 
/// Method: `minecraft:allowlist/remove`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistRemoveRequest {
    pub remove: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistRemoveResponse {
    pub allowlist: Vec<Player>,
}

/// Clear all players in allowlist
/// 
/// Method: `minecraft:allowlist/clear`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistClearResponse {
    pub allowlist: Vec<Player>,
}

/// Get the ban list
/// 
/// Method: `minecraft:bans`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansResponse {
    pub banlist: Vec<UserBan>,
}

/// Set the banlist
/// 
/// Method: `minecraft:bans/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansSetRequest {
    pub bans: Vec<UserBan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansSetResponse {
    pub banlist: Vec<UserBan>,
}

/// Add players to ban list
/// 
/// Method: `minecraft:bans/add`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansAddRequest {
    pub add: Vec<UserBan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansAddResponse {
    pub banlist: Vec<UserBan>,
}

/// Remove players from ban list
/// 
/// Method: `minecraft:bans/remove`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansRemoveRequest {
    pub remove: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansRemoveResponse {
    pub banlist: Vec<UserBan>,
}

/// Clear all players in ban list
/// 
/// Method: `minecraft:bans/clear`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BansClearResponse {
    pub banlist: Vec<UserBan>,
}

/// Get the ip ban list
/// 
/// Method: `minecraft:ip_bans`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansResponse {
    pub banlist: Vec<Player>,
}

/// Set the ip banlist
/// 
/// Method: `minecraft:ip_bans/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansSetRequest {
    pub banlist: Vec<IpBan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansSetResponse {
    pub banlist: Vec<IpBan>,
}

/// Add ip to ban list
/// 
/// Method: `minecraft:ip_bans/add`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansAddRequest {
    pub add: Vec<IncomingIpBan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansAddResponse {
    pub banlist: Vec<IpBan>,
}

/// Remove ip from ban list
/// 
/// Method: `minecraft:ip_bans/remove`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansRemoveRequest {
    pub ip: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansRemoveResponse {
    pub banlist: Vec<IpBan>,
}

/// Clear all ips in ban list
/// 
/// Method: `minecraft:ip_bans/clear`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBansClearResponse {
    pub banlist: Vec<IpBan>,
}

/// Get all connected players
/// 
/// Method: `minecraft:players`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayersResponse {
    pub players: Vec<Player>,
}

/// Kick players
/// 
/// Method: `minecraft:players/kick`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayersKickRequest {
    pub kick: Vec<KickPlayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayersKickResponse {
    pub kicked: Vec<Player>,
}

/// Get all oped players
/// 
/// Method: `minecraft:operators`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsResponse {
    pub operators: Vec<Operator>,
}

/// Set all oped players
/// 
/// Method: `minecraft:operators/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsSetRequest {
    pub operators: Vec<Operator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsSetResponse {
    pub operators: Vec<Operator>,
}

/// Op players
/// 
/// Method: `minecraft:operators/add`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsAddRequest {
    pub add: Vec<Operator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsAddResponse {
    pub operators: Vec<Operator>,
}

/// Deop players
/// 
/// Method: `minecraft:operators/remove`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsRemoveRequest {
    pub remove: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsRemoveResponse {
    pub operators: Vec<Operator>,
}

/// Deop all players
/// 
/// Method: `minecraft:operators/clear`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorsClearResponse {
    pub operators: Vec<Operator>,
}

/// Get server status
/// 
/// Method: `minecraft:server/status`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatusResponse {
    pub status: ServerState,
}

/// Save server state
/// 
/// Method: `minecraft:server/save`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSaveRequest {
    pub flush: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSaveResponse {
    pub saving: bool,
}

/// Stop server
/// 
/// Method: `minecraft:server/stop`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStopResponse {
    pub stopping: bool,
}

/// Send a system message
/// 
/// Method: `minecraft:server/system_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSystemMessageRequest {
    pub message: SystemMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSystemMessageResponse {
    pub sent: bool,
}

/// Get whether automatic world saving is enabled on the server
/// 
/// Method: `minecraft:serversettings/autosave`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAutosaveResponse {
    pub enabled: bool,
}

/// Enable or disable automatic world saving on the server
/// 
/// Method: `minecraft:serversettings/autosave/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAutosaveSetRequest {
    pub enable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAutosaveSetResponse {
    pub enabled: bool,
}

/// Get the current difficulty level of the server
/// 
/// Method: `minecraft:serversettings/difficulty`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsDifficultyResponse {
    pub difficulty: String,
}

/// Set the difficulty level of the server
/// 
/// Method: `minecraft:serversettings/difficulty/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsDifficultySetRequest {
    pub difficulty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsDifficultySetResponse {
    pub difficulty: String,
}

/// Get whether allowlist enforcement is enabled (kicks players immediately when removed from allowlist)
/// 
/// Method: `minecraft:serversettings/enforce_allowlist`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsEnforceAllowlistResponse {
    pub enforced: bool,
}

/// Enable or disable allowlist enforcement (when enabled, players are kicked immediately upon removal from allowlist)
/// 
/// Method: `minecraft:serversettings/enforce_allowlist/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsEnforceAllowlistSetRequest {
    pub enforce: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsEnforceAllowlistSetResponse {
    pub enforced: bool,
}

/// Get whether the allowlist is enabled on the server
/// 
/// Method: `minecraft:serversettings/use_allowlist`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsUseAllowlistResponse {
    pub used: bool,
}

/// Enable or disable the allowlist on the server (controls whether only allowlisted players can join)
/// 
/// Method: `minecraft:serversettings/use_allowlist/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsUseAllowlistSetRequest {
    #[serde(rename = "use")]
    pub r#use: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsUseAllowlistSetResponse {
    pub used: bool,
}

/// Get the maximum number of players allowed to connect to the server
/// 
/// Method: `minecraft:serversettings/max_players`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsMaxPlayersResponse {
    pub max: i64,
}

/// Set the maximum number of players allowed to connect to the server
/// 
/// Method: `minecraft:serversettings/max_players/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsMaxPlayersSetRequest {
    pub max: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsMaxPlayersSetResponse {
    pub max: i64,
}

/// Get the number of seconds before the game is automatically paused when no players are online
/// 
/// Method: `minecraft:serversettings/pause_when_empty_seconds`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsPauseWhenEmptySecondsResponse {
    pub seconds: i64,
}

/// Set the number of seconds before the game is automatically paused when no players are online
/// 
/// Method: `minecraft:serversettings/pause_when_empty_seconds/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsPauseWhenEmptySecondsSetRequest {
    pub seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsPauseWhenEmptySecondsSetResponse {
    pub seconds: i64,
}

/// Get the number of seconds before idle players are automatically kicked from the server
/// 
/// Method: `minecraft:serversettings/player_idle_timeout`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsPlayerIdleTimeoutResponse {
    pub seconds: i64,
}

/// Set the number of seconds before idle players are automatically kicked from the server
/// 
/// Method: `minecraft:serversettings/player_idle_timeout/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsPlayerIdleTimeoutSetRequest {
    pub seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsPlayerIdleTimeoutSetResponse {
    pub seconds: i64,
}

/// Get whether flight is allowed for players in Survival mode
/// 
/// Method: `minecraft:serversettings/allow_flight`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAllowFlightResponse {
    pub allowed: bool,
}

/// Allow or disallow flight for players in Survival mode
/// 
/// Method: `minecraft:serversettings/allow_flight/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAllowFlightSetRequest {
    pub allow: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAllowFlightSetResponse {
    pub allowed: bool,
}

/// Get the server's message of the day displayed to players
/// 
/// Method: `minecraft:serversettings/motd`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsMotdResponse {
    pub message: String,
}

/// Set the server's message of the day displayed to players
/// 
/// Method: `minecraft:serversettings/motd/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsMotdSetRequest {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsMotdSetResponse {
    pub message: String,
}

/// Get the spawn protection radius in blocks (only operators can edit within this area)
/// 
/// Method: `minecraft:serversettings/spawn_protection_radius`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsSpawnProtectionRadiusResponse {
    pub radius: i64,
}

/// Set the spawn protection radius in blocks (only operators can edit within this area)
/// 
/// Method: `minecraft:serversettings/spawn_protection_radius/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsSpawnProtectionRadiusSetRequest {
    pub radius: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsSpawnProtectionRadiusSetResponse {
    pub radius: i64,
}

/// Get whether players are forced to use the server's default game mode
/// 
/// Method: `minecraft:serversettings/force_game_mode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsForceGameModeResponse {
    pub forced: bool,
}

/// Enable or disable forcing players to use the server's default game mode
/// 
/// Method: `minecraft:serversettings/force_game_mode/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsForceGameModeSetRequest {
    pub force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsForceGameModeSetResponse {
    pub forced: bool,
}

/// Get the server's default game mode
/// 
/// Method: `minecraft:serversettings/game_mode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsGameModeResponse {
    pub mode: String,
}

/// Set the server's default game mode
/// 
/// Method: `minecraft:serversettings/game_mode/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsGameModeSetRequest {
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsGameModeSetResponse {
    pub mode: String,
}

/// Get the server's view distance in chunks
/// 
/// Method: `minecraft:serversettings/view_distance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsViewDistanceResponse {
    pub distance: i64,
}

/// Set the server's view distance in chunks
/// 
/// Method: `minecraft:serversettings/view_distance/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsViewDistanceSetRequest {
    pub distance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsViewDistanceSetResponse {
    pub distance: i64,
}

/// Get the server's simulation distance in chunks
/// 
/// Method: `minecraft:serversettings/simulation_distance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsSimulationDistanceResponse {
    pub distance: i64,
}

/// Set the server's simulation distance in chunks
/// 
/// Method: `minecraft:serversettings/simulation_distance/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsSimulationDistanceSetRequest {
    pub distance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsSimulationDistanceSetResponse {
    pub distance: i64,
}

/// Get whether the server accepts player transfers from other servers
/// 
/// Method: `minecraft:serversettings/accept_transfers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAcceptTransfersResponse {
    pub accepted: bool,
}

/// Enable or disable accepting player transfers from other servers
/// 
/// Method: `minecraft:serversettings/accept_transfers/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAcceptTransfersSetRequest {
    pub accept: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsAcceptTransfersSetResponse {
    pub accepted: bool,
}

/// Get the interval in seconds between server status heartbeats
/// 
/// Method: `minecraft:serversettings/status_heartbeat_interval`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsStatusHeartbeatIntervalResponse {
    pub seconds: i64,
}

/// Set the interval in seconds between server status heartbeats
/// 
/// Method: `minecraft:serversettings/status_heartbeat_interval/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsStatusHeartbeatIntervalSetRequest {
    pub seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsStatusHeartbeatIntervalSetResponse {
    pub seconds: i64,
}

/// Get the permission level required for operator commands
/// 
/// Method: `minecraft:serversettings/operator_user_permission_level`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsOperatorUserPermissionLevelResponse {
    pub level: i64,
}

/// Set the permission level required for operator commands
/// 
/// Method: `minecraft:serversettings/operator_user_permission_level/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsOperatorUserPermissionLevelSetRequest {
    pub level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsOperatorUserPermissionLevelSetResponse {
    pub level: i64,
}

/// Get whether the server hides online player information from status queries
/// 
/// Method: `minecraft:serversettings/hide_online_players`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsHideOnlinePlayersResponse {
    pub hidden: bool,
}

/// Enable or disable hiding online player information from status queries
/// 
/// Method: `minecraft:serversettings/hide_online_players/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsHideOnlinePlayersSetRequest {
    pub hide: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsHideOnlinePlayersSetResponse {
    pub hidden: bool,
}

/// Get whether the server responds to connection status requests
/// 
/// Method: `minecraft:serversettings/status_replies`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsStatusRepliesResponse {
    pub enabled: bool,
}

/// Enable or disable the server responding to connection status requests
/// 
/// Method: `minecraft:serversettings/status_replies/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsStatusRepliesSetRequest {
    pub enable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsStatusRepliesSetResponse {
    pub enabled: bool,
}

/// Get the entity broadcast range as a percentage
/// 
/// Method: `minecraft:serversettings/entity_broadcast_range`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsEntityBroadcastRangeResponse {
    pub percentage_points: i64,
}

/// Set the entity broadcast range as a percentage
/// 
/// Method: `minecraft:serversettings/entity_broadcast_range/set`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsEntityBroadcastRangeSetRequest {
    pub percentage_points: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersettingsEntityBroadcastRangeSetResponse {
    pub percentage_points: i64,
}

/// Get the available game rule keys and their current values
/// 
/// Method: `minecraft:gamerules`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamerulesResponse {
    pub gamerules: Vec<TypedGameRule>,
}

/// Update game rule value
/// 
/// Method: `minecraft:gamerules/update`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamerulesUpdateRequest {
    pub gamerule: UntypedGameRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamerulesUpdateResponse {
    pub gamerule: TypedGameRule,
}

