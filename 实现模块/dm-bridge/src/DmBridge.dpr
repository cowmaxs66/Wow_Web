library DmBridge;

uses
  System.SysUtils,
  DmBridge.Types in 'DmBridge.Types.pas',
  DmBridge.Strings in 'DmBridge.Strings.pas',
  DmBridge.Errors in 'DmBridge.Errors.pas',
  DmBridge.Dmsoft in 'DmBridge.Dmsoft.pas',
  DmBridge.Worker.Types in 'DmBridge.Worker.Types.pas',
  DmBridge.Worker.Request in 'DmBridge.Worker.Request.pas',
  DmBridge.Worker.Thread in 'DmBridge.Worker.Thread.pas',
  DmBridge.Worker in 'DmBridge.Worker.pas',
  DmBridge.Api.Common in 'DmBridge.Api.Common.pas',
  DmBridge.Api.Abi in 'DmBridge.Api.Abi.pas',
  DmBridge.Api.Lifecycle in 'DmBridge.Api.Lifecycle.pas',
  DmBridge.Api.Basic in 'DmBridge.Api.Basic.pas',
  DmBridge.Api.Window in 'DmBridge.Api.Window.pas',
  DmBridge.Api.Bind in 'DmBridge.Api.Bind.pas',
  DmBridge.Api.Color in 'DmBridge.Api.Color.pas',
  DmBridge.Api.Input in 'DmBridge.Api.Input.pas';

exports
  dm_bridge_abi_version name 'dm_bridge_abi_version',
  dm_bridge_init name 'dm_bridge_init',
  dm_bridge_shutdown name 'dm_bridge_shutdown',
  dm_bridge_get_last_bridge_error name 'dm_bridge_get_last_bridge_error',
  dm_bridge_get_last_dm_error name 'dm_bridge_get_last_dm_error',
  dm_bridge_get_last_message name 'dm_bridge_get_last_message',
  dm_bridge_ver name 'dm_bridge_ver',
  dm_bridge_set_path name 'dm_bridge_set_path',
  dm_bridge_find_window name 'dm_bridge_find_window',
  dm_bridge_bind_window name 'dm_bridge_bind_window',
  dm_bridge_get_color name 'dm_bridge_get_color',
  dm_bridge_move_to name 'dm_bridge_move_to',
  dm_bridge_left_click name 'dm_bridge_left_click',
  dm_bridge_unbind_window name 'dm_bridge_unbind_window';

begin
end.
