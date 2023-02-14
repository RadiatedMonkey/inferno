pub use add_painting::*;
pub use animate::*;
pub use available_commands::*;
pub use biome_definition_list::*;
pub use cache_status::*;
pub use camera_shake::*;
pub use change_dimension::*;
pub use chunk_radius_reply::*;
pub use chunk_radius_request::*;
pub use client_bound_debug_renderer::*;
pub use client_to_server_handshake::*;
pub use command_request::*;
pub use connect_automation_client::*;
pub use creative_content::*;
pub use death_info::*;
pub use disconnect::*;
pub use game_rules_changed::*;
pub use interact::*;
pub use level_chunk::*;
pub use login::*;
pub use mob_effect::*;
pub use move_player::*;
pub use network_chunk_publisher_update::*;
pub use network_settings::*;
pub use online_ping::*;
pub use online_pong::*;
pub use packet::*;
pub use play_sound::*;
pub use play_status::*;
pub use player_fog::*;
pub use player_list::*;
pub use request_ability::*;
pub use request_network_settings::*;
pub use resource_pack_client_response::*;
pub use resource_pack_stack::*;
pub use resource_packs_info::*;
pub use respawn::*;
pub use server_to_client_handshake::*;
pub use set_commands_enabled::*;
pub use set_default_game_mode::*;
pub use set_difficulty::*;
pub use set_local_player_as_initialized::*;
pub use set_player_gamemode::*;
pub use set_scoreboard_identity::*;
pub use set_time::*;
pub use set_title::*;
pub use show_credits::*;
pub use show_profile::*;
pub use spawn_experience_orb::*;
pub use start_game::*;
pub use text::*;
pub use tick_sync::*;
pub use toast_request::*;
pub use traits::*;
pub use transfer::*;
pub use update_dynamic_enum::*;
pub use violation_warning::*;
pub use update_skin::*;

mod update_skin;
mod add_painting;
mod animate;
mod available_commands;
mod biome_definition_list;
mod cache_status;
mod camera_shake;
mod change_dimension;
mod chunk_radius_reply;
mod chunk_radius_request;
mod client_bound_debug_renderer;
mod client_to_server_handshake;
mod command_request;
mod connect_automation_client;
mod creative_content;
mod death_info;
mod disconnect;
mod game_rules_changed;
mod interact;
mod level_chunk;
mod login;
mod mob_effect;
mod move_player;
mod network_chunk_publisher_update;
mod network_settings;
mod online_ping;
mod online_pong;
mod packet;
mod play_sound;
mod play_status;
mod player_fog;
mod player_list;
mod request_ability;
mod request_network_settings;
mod resource_pack_client_response;
mod resource_pack_stack;
mod resource_packs_info;
mod respawn;
mod server_to_client_handshake;
mod set_commands_enabled;
mod set_default_game_mode;
mod set_difficulty;
mod set_local_player_as_initialized;
mod set_player_gamemode;
mod set_scoreboard_identity;
mod set_time;
mod set_title;
mod show_credits;
mod show_profile;
mod spawn_experience_orb;
mod start_game;
mod text;
mod tick_sync;
mod toast_request;
mod traits;
mod transfer;
mod update_dynamic_enum;
mod violation_warning;

/// ID of Minecraft game packets.
pub const GAME_PACKET_ID: u8 = 0xfe;
/// Semver version that this server supports.
pub const CLIENT_VERSION_STRING: &str = "1.19.60";
/// Protocol version that this server supports.
pub const NETWORK_VERSION: u32 = 567;
