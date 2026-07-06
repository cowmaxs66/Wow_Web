unit DmBridge.Api.Lifecycle;

interface

uses
  Winapi.Windows;

function dm_bridge_init(DmRoot: PWideChar): Integer; stdcall;
function dm_bridge_shutdown: Integer; stdcall;
function dm_bridge_get_last_bridge_error: Integer; stdcall;
function dm_bridge_get_last_dm_error(OutError: PInteger): Integer; stdcall;
function dm_bridge_get_last_message(OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer; stdcall;

implementation

uses
  DmBridge.Types,
  DmBridge.Strings,
  DmBridge.Errors,
  DmBridge.Worker;

function dm_bridge_init(DmRoot: PWideChar): Integer; stdcall;
begin
  Result := WorkerInit(ReadWideArg(DmRoot));
end;

function dm_bridge_shutdown: Integer; stdcall;
begin
  Result := WorkerShutdown;
end;

function dm_bridge_get_last_bridge_error: Integer; stdcall;
begin
  Result := GetLastBridgeError;
end;

function dm_bridge_get_last_dm_error(OutError: PInteger): Integer; stdcall;
var
  ErrorCode: Integer;
begin
  if OutError = nil then
  begin
    SetBridgeError(DM_BRIDGE_INVALID_ARG, 'OutError must not be nil');
    Exit(DM_BRIDGE_INVALID_ARG);
  end;

  Result := WorkerGetLastDmError(ErrorCode);
  OutError^ := ErrorCode;
end;

function dm_bridge_get_last_message(OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer; stdcall;
begin
  Result := WriteLastBridgeMessage(OutBuf, OutCapacity, OutLen);
end;

end.
