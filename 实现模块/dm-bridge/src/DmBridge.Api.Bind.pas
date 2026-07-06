unit DmBridge.Api.Bind;

interface

uses
  Winapi.Windows;

function dm_bridge_bind_window(Hwnd: Integer; Display, Mouse, Keypad: PWideChar; Mode: Integer; OutDmRet: PInteger): Integer; stdcall;
function dm_bridge_unbind_window(OutDmRet: PInteger): Integer; stdcall;

implementation

uses
  DmBridge.Types,
  DmBridge.Strings,
  DmBridge.Errors,
  DmBridge.Worker,
  DmBridge.Dmsoft,
  DmBridge.Api.Common;

function dm_bridge_bind_window(Hwnd: Integer; Display, Mouse, Keypad: PWideChar; Mode: Integer; OutDmRet: PInteger): Integer; stdcall;
var
  DmRet: Integer;
begin
  if Hwnd = 0 then
  begin
    SetBridgeError(DM_BRIDGE_INVALID_ARG, 'Hwnd must not be zero');
    Exit(DM_BRIDGE_INVALID_ARG);
  end;

  DmRet := 0;
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      DmRet := Dm.BindWindow(Hwnd, ReadWideArg(Display), ReadWideArg(Mouse), ReadWideArg(Keypad), Mode);
    end);

  WriteOutInt(OutDmRet, DmRet);
  if Result = DM_BRIDGE_OK then
    Result := DmRetToBridgeStatus(DmRet);
end;

function dm_bridge_unbind_window(OutDmRet: PInteger): Integer; stdcall;
var
  DmRet: Integer;
begin
  DmRet := 0;
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      DmRet := Dm.UnBindWindow;
    end);

  WriteOutInt(OutDmRet, DmRet);
  if Result = DM_BRIDGE_OK then
    Result := DmRetToBridgeStatus(DmRet);
end;

end.
