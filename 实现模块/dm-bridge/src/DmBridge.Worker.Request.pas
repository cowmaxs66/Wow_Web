unit DmBridge.Worker.Request;

interface

uses
  System.SysUtils,
  System.SyncObjs,
  DmBridge.Types,
  DmBridge.Errors,
  DmBridge.Dmsoft,
  DmBridge.Worker.Types;

type
  TDmCallRequest = class
  private
    FCall: TDmsoftCall;
    FDoneEvent: TEvent;
  public
    Status: Integer;

    constructor Create(const ACall: TDmsoftCall);
    destructor Destroy; override;

    procedure WaitDone;
    procedure SignalDone;
    procedure Run(Dm: TDmsoftHost);
  end;

implementation

constructor TDmCallRequest.Create(const ACall: TDmsoftCall);
begin
  inherited Create;
  FCall := ACall;
  Status := DM_BRIDGE_OK;
  FDoneEvent := TEvent.Create(nil, True, False, '');
end;

destructor TDmCallRequest.Destroy;
begin
  FDoneEvent.Free;
  inherited Destroy;
end;

procedure TDmCallRequest.WaitDone;
begin
  FDoneEvent.WaitFor(INFINITE);
end;

procedure TDmCallRequest.SignalDone;
begin
  FDoneEvent.SetEvent;
end;

procedure TDmCallRequest.Run(Dm: TDmsoftHost);
begin
  if not Assigned(FCall) then
  begin
    SetBridgeError(DM_BRIDGE_INVALID_ARG, 'Dmsoft call must not be nil');
    Status := DM_BRIDGE_INVALID_ARG;
    Exit;
  end;

  try
    FCall(Dm);
    ClearBridgeError;
    Status := DM_BRIDGE_OK;
  except
    on E: Exception do
    begin
      SetBridgeError(DM_BRIDGE_COM_ERROR, E.Message);
      Status := DM_BRIDGE_COM_ERROR;
    end;
  end;
end;

end.
