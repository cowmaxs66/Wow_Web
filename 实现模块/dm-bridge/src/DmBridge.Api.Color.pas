unit DmBridge.Api.Color;

interface

uses
  Winapi.Windows;

function dm_bridge_get_color(X, Y: Integer; OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer; stdcall;

implementation

uses
  DmBridge.Types,
  DmBridge.Strings,
  DmBridge.Worker,
  DmBridge.Dmsoft;

function dm_bridge_get_color(X, Y: Integer; OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer; stdcall;
var
  ColorText: string;
begin
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      ColorText := Dm.GetColor(X, Y);
    end);

  if Result <> DM_BRIDGE_OK then
    Exit;

  Result := WriteWideBuffer(ColorText, OutBuf, OutCapacity, OutLen);
end;

end.
