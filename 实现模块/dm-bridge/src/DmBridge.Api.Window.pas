unit DmBridge.Api.Window;

interface

uses
  Winapi.Windows;

function dm_bridge_find_window(ClassName, TitleName: PWideChar; OutHwnd: PInteger): Integer; stdcall;

implementation

uses
  System.SysUtils,
  DmBridge.Types,
  DmBridge.Strings,
  DmBridge.Errors,
  DmBridge.Worker,
  DmBridge.Dmsoft;

function dm_bridge_find_window(ClassName, TitleName: PWideChar; OutHwnd: PInteger): Integer; stdcall;
var
  Hwnd: Integer;
  ClassText: string;
  TitleText: string;
begin
  if OutHwnd = nil then
  begin
    SetBridgeError(DM_BRIDGE_INVALID_ARG, 'OutHwnd must not be nil');
    Exit(DM_BRIDGE_INVALID_ARG);
  end;

  ClassText := ReadWideArg(ClassName);
  TitleText := ReadWideArg(TitleName);
  Hwnd := 0;

  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      Hwnd := Dm.FindWindow(ClassText, TitleText);
    end);

  OutHwnd^ := Hwnd;
  if (Result = DM_BRIDGE_OK) and (Hwnd = 0) then
  begin
    SetBridgeError(
      DM_BRIDGE_DM_FAILED,
      Format('FindWindow not found: class=%s title=%s', [ClassText, TitleText])
    );
    Result := DM_BRIDGE_DM_FAILED;
  end;
end;

end.
