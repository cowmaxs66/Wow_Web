unit DmBridge.Api.Input;

interface

uses
  Winapi.Windows;

function dm_bridge_move_to(X, Y: Integer; OutDmRet: PInteger): Integer; stdcall;
function dm_bridge_left_click(OutDmRet: PInteger): Integer; stdcall;

implementation

uses
  DmBridge.Types,
  DmBridge.Worker,
  DmBridge.Dmsoft,
  DmBridge.Api.Common;

function dm_bridge_move_to(X, Y: Integer; OutDmRet: PInteger): Integer; stdcall;
var
  DmRet: Integer;
begin
  DmRet := 0;
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      DmRet := Dm.MoveTo(X, Y);
    end);

  WriteOutInt(OutDmRet, DmRet);
  if Result = DM_BRIDGE_OK then
    Result := DmRetToBridgeStatus(DmRet);
end;

function dm_bridge_left_click(OutDmRet: PInteger): Integer; stdcall;
var
  DmRet: Integer;
begin
  DmRet := 0;
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      DmRet := Dm.LeftClick;
    end);

  WriteOutInt(OutDmRet, DmRet);
  if Result = DM_BRIDGE_OK then
    Result := DmRetToBridgeStatus(DmRet);
end;

end.
