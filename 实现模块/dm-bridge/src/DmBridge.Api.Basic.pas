unit DmBridge.Api.Basic;

interface

uses
  Winapi.Windows;

function dm_bridge_ver(OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer; stdcall;
function dm_bridge_set_path(Path: PWideChar): Integer; stdcall;

implementation

uses
  DmBridge.Types,
  DmBridge.Strings,
  DmBridge.Errors,
  DmBridge.Worker,
  DmBridge.Dmsoft,
  DmBridge.Api.Common;

function dm_bridge_ver(OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer; stdcall;
var
  VersionText: string;
begin
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      VersionText := Dm.Ver;
    end);

  if Result <> DM_BRIDGE_OK then
    Exit;

  Result := WriteWideBuffer(VersionText, OutBuf, OutCapacity, OutLen);
end;

function dm_bridge_set_path(Path: PWideChar): Integer; stdcall;
var
  DmRet: Integer;
  PathText: string;
begin
  PathText := ReadWideArg(Path);
  if PathText = '' then
  begin
    SetBridgeError(DM_BRIDGE_INVALID_ARG, 'Path must not be empty');
    Exit(DM_BRIDGE_INVALID_ARG);
  end;

  DmRet := 0;
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      DmRet := Dm.SetPath(PathText);
    end);

  if Result = DM_BRIDGE_OK then
    Result := DmRetToBridgeStatus(DmRet);
end;

end.
