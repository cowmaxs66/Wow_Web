unit DmBridge.Api.Common;

interface

uses
  DmBridge.Types;

function DmRetToBridgeStatus(DmRet: Integer): Integer;
procedure WriteOutInt(Target: PInteger; Value: Integer);

implementation

function DmRetToBridgeStatus(DmRet: Integer): Integer;
begin
  if DmRet = 1 then
    Result := DM_BRIDGE_OK
  else
    Result := DM_BRIDGE_DM_FAILED;
end;

procedure WriteOutInt(Target: PInteger; Value: Integer);
begin
  if Target <> nil then
    Target^ := Value;
end;

end.
