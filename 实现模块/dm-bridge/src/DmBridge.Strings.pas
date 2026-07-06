unit DmBridge.Strings;

interface

uses
  Winapi.Windows,
  DmBridge.Types;

function ReadWideArg(Value: PWideChar): string;
function WriteWideBuffer(const Value: string; OutBuf: PWideChar; OutCapacity: Cardinal;
  OutLen: PCardinal): Integer;

implementation

function ReadWideArg(Value: PWideChar): string;
begin
  if Value = nil then
    Exit('');

  Result := string(Value);
end;

function WriteWideBuffer(const Value: string; OutBuf: PWideChar; OutCapacity: Cardinal;
  OutLen: PCardinal): Integer;
var
  Needed: Cardinal;
begin
  Needed := Length(Value);
  if OutLen <> nil then
    OutLen^ := Needed;

  // 字符串输出统一由调用方提供 UTF-16 buffer。
  // 输入：Delphi string、输出 buffer、buffer 字符容量。
  // 输出：写入 NUL 结尾字符串，或返回 buffer 不足。
  // 边界：OutCapacity 是 WideChar 数量，不是字节数。
  if (OutBuf = nil) or (OutCapacity = 0) or (Needed + 1 > OutCapacity) then
    Exit(DM_BRIDGE_BUFFER_TOO_SMALL);

  if Needed > 0 then
    Move(PWideChar(Value)^, OutBuf^, Needed * SizeOf(WideChar));

  OutBuf[Needed] := #0;
  Result := DM_BRIDGE_OK;
end;

end.
