unit DmBridge.Api.Bind;

interface

uses
  Winapi.Windows;

function dm_bridge_bind_window(Hwnd: Integer; Display, Mouse, Keypad: PWideChar; Mode: Integer; OutDmRet: PInteger): Integer; stdcall;
function dm_bridge_unbind_window(OutDmRet: PInteger): Integer; stdcall;

implementation

uses
  System.SysUtils,
  DmBridge.Types,
  DmBridge.Strings,
  DmBridge.Errors,
  DmBridge.Worker,
  DmBridge.Dmsoft,
  DmBridge.Api.Common;

function SafeLastDmError(Dm: TDmsoftHost): Integer;
begin
  try
    Result := Dm.GetLastError;
  except
    Result := 0;
  end;
end;

function BindContext(Hwnd: Integer; const Display, Mouse, Keypad: string; Mode, LastDmError: Integer): string;
begin
  Result := Format(
    'hwnd=%d display=%s mouse=%s keypad=%s mode=%d last_dm_error=%d',
    [Hwnd, Display, Mouse, Keypad, Mode, LastDmError]
  );
end;

function dm_bridge_bind_window(Hwnd: Integer; Display, Mouse, Keypad: PWideChar; Mode: Integer; OutDmRet: PInteger): Integer; stdcall;
var
  DmRet: Integer;
  DisplayText: string;
  MouseText: string;
  KeypadText: string;
  LastDmError: Integer;
begin
  if Hwnd <= 0 then
  begin
    SetBridgeError(DM_BRIDGE_INVALID_ARG, Format('Hwnd must be positive: %d', [Hwnd]));
    Exit(DM_BRIDGE_INVALID_ARG);
  end;

  if not IsWindow(Hwnd) then
  begin
    SetBridgeError(DM_BRIDGE_INVALID_ARG, Format('Hwnd is not a valid window: %d', [Hwnd]));
    Exit(DM_BRIDGE_INVALID_ARG);
  end;

  DisplayText := ReadWideArg(Display);
  MouseText := ReadWideArg(Mouse);
  KeypadText := ReadWideArg(Keypad);
  if (DisplayText = '') or (MouseText = '') or (KeypadText = '') then
  begin
    SetBridgeError(
      DM_BRIDGE_INVALID_ARG,
      Format(
        'BindWindow mode strings must not be empty: display=%s mouse=%s keypad=%s',
        [DisplayText, MouseText, KeypadText]
      )
    );
    Exit(DM_BRIDGE_INVALID_ARG);
  end;

  DmRet := 0;
  LastDmError := 0;
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      try
        DmRet := Dm.BindWindow(Hwnd, DisplayText, MouseText, KeypadText, Mode);
        if DmRet <> 1 then
          LastDmError := SafeLastDmError(Dm);
      except
        on E: Exception do
        begin
          LastDmError := SafeLastDmError(Dm);
          raise Exception.Create(
            Format(
              'BindWindow exception: %s exception=%s: %s',
              [BindContext(Hwnd, DisplayText, MouseText, KeypadText, Mode, LastDmError), E.ClassName, E.Message]
            )
          );
        end;
      end;
    end);

  WriteOutInt(OutDmRet, DmRet);
  if Result = DM_BRIDGE_OK then
  begin
    Result := DmRetToBridgeStatus(DmRet);
    if Result <> DM_BRIDGE_OK then
      SetBridgeError(DM_BRIDGE_DM_FAILED, 'BindWindow failed: ' + BindContext(Hwnd, DisplayText, MouseText, KeypadText, Mode, LastDmError));
  end;
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
