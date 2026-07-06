unit DmBridge.Dmsoft;

interface

uses
  System.SysUtils,
  System.Variants,
  System.Win.ComObj;

type
  TDmsoftHost = class
  private
    FObj: OleVariant;
  public
    constructor Create;
    destructor Destroy; override;

    function Ver: string;
    function GetLastError: Integer;
    function SetPath(const Path: string): Integer;
    function FindWindow(const ClassName, TitleName: string): Integer;
    function BindWindow(Hwnd: Integer; const Display, Mouse, Keypad: string; Mode: Integer): Integer;
    function GetColor(X, Y: Integer): string;
    function MoveTo(X, Y: Integer): Integer;
    function LeftClick: Integer;
    function UnBindWindow: Integer;
  end;

implementation

constructor TDmsoftHost.Create;
begin
  inherited Create;
  FObj := CreateOleObject('dm.dmsoft');
end;

destructor TDmsoftHost.Destroy;
begin
  FObj := Unassigned;
  inherited Destroy;
end;

function TDmsoftHost.Ver: string;
begin
  Result := string(FObj.Ver);
end;

function TDmsoftHost.GetLastError: Integer;
begin
  Result := Integer(FObj.GetLastError);
end;

function TDmsoftHost.SetPath(const Path: string): Integer;
begin
  Result := Integer(FObj.SetPath(WideString(Path)));
end;

function TDmsoftHost.FindWindow(const ClassName, TitleName: string): Integer;
begin
  Result := Integer(FObj.FindWindow(WideString(ClassName), WideString(TitleName)));
end;

function TDmsoftHost.BindWindow(Hwnd: Integer; const Display, Mouse, Keypad: string; Mode: Integer): Integer;
begin
  Result := Integer(FObj.BindWindow(Hwnd, WideString(Display), WideString(Mouse), WideString(Keypad), Mode));
end;

function TDmsoftHost.GetColor(X, Y: Integer): string;
begin
  Result := string(FObj.GetColor(X, Y));
end;

function TDmsoftHost.MoveTo(X, Y: Integer): Integer;
begin
  Result := Integer(FObj.MoveTo(X, Y));
end;

function TDmsoftHost.LeftClick: Integer;
begin
  Result := Integer(FObj.LeftClick);
end;

function TDmsoftHost.UnBindWindow: Integer;
begin
  Result := Integer(FObj.UnBindWindow);
end;

end.
