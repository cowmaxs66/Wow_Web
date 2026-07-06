unit DmBridge.Errors;

interface

uses
  System.SysUtils,
  DmBridge.Types,
  DmBridge.Strings;

procedure SetBridgeError(Code: Integer; const MessageText: string);
procedure ClearBridgeError;
function GetLastBridgeError: Integer;
function WriteLastBridgeMessage(OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer;

implementation

var
  GLastBridgeError: Integer = DM_BRIDGE_OK;
  GLastBridgeMessage: string = '';

procedure SetBridgeError(Code: Integer; const MessageText: string);
begin
  GLastBridgeError := Code;
  GLastBridgeMessage := MessageText;
end;

procedure ClearBridgeError;
begin
  GLastBridgeError := DM_BRIDGE_OK;
  GLastBridgeMessage := '';
end;

function GetLastBridgeError: Integer;
begin
  Result := GLastBridgeError;
end;

function WriteLastBridgeMessage(OutBuf: PWideChar; OutCapacity: Cardinal; OutLen: PCardinal): Integer;
begin
  Result := WriteWideBuffer(GLastBridgeMessage, OutBuf, OutCapacity, OutLen);
end;

end.
