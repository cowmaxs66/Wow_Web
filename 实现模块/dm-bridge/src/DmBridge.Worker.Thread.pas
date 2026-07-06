unit DmBridge.Worker.Thread;

interface

uses
  System.SysUtils,
  System.Classes,
  System.SyncObjs,
  System.Generics.Collections,
  System.Win.ComObj,
  Winapi.ActiveX,
  DmBridge.Types,
  DmBridge.Errors,
  DmBridge.Dmsoft,
  DmBridge.Worker.Types,
  DmBridge.Worker.Request;

type
  TDmWorkerThread = class(TThread)
  private
    FDmRoot: string;
    FDm: TDmsoftHost;
    FQueue: TQueue<TDmCallRequest>;
    FQueueLock: TCriticalSection;
    FQueueEvent: TEvent;
    FInitEvent: TEvent;
    FInitStatus: Integer;
    FCoInitialized: Boolean;

    function PopRequest: TDmCallRequest;
    function QueueIsEmpty: Boolean;
    procedure InitDm;
    procedure FreeDm;
  protected
    procedure Execute; override;
  public
    constructor Create(const ADmRoot: string);
    destructor Destroy; override;

    function WaitInit: Integer;
    function Invoke(const Call: TDmsoftCall): Integer;
    procedure StopAndWait;
  end;

implementation

constructor TDmWorkerThread.Create(const ADmRoot: string);
begin
  inherited Create(True);
  FreeOnTerminate := False;
  FDmRoot := ADmRoot;
  FInitStatus := DM_BRIDGE_NOT_INITIALIZED;
  FQueue := TQueue<TDmCallRequest>.Create;
  FQueueLock := TCriticalSection.Create;
  FQueueEvent := TEvent.Create(nil, False, False, '');
  FInitEvent := TEvent.Create(nil, True, False, '');
end;

destructor TDmWorkerThread.Destroy;
begin
  FInitEvent.Free;
  FQueueEvent.Free;
  FQueueLock.Free;
  FQueue.Free;
  inherited Destroy;
end;

procedure TDmWorkerThread.Execute;
var
  Request: TDmCallRequest;
begin
  InitDm;
  FInitEvent.SetEvent;

  // 所有 dm.dmsoft 调用都进入这个 STA 线程。
  // 输入：外部线程提交的 TDmCallRequest。
  // 输出：请求完成事件和状态码。
  // 边界：终止信号到来后仍会处理已入队请求，再释放 COM 对象。
  while FInitStatus = DM_BRIDGE_OK do
  begin
    FQueueEvent.WaitFor(INFINITE);

    repeat
      Request := PopRequest;
      if Request = nil then
        Break;

      Request.Run(FDm);
      Request.SignalDone;
    until False;

    if Terminated and QueueIsEmpty then
      Break;
  end;

  FreeDm;
end;

procedure TDmWorkerThread.InitDm;
var
  SetPathRet: Integer;
begin
  try
    OleCheck(CoInitializeEx(nil, COINIT_APARTMENTTHREADED));
    FCoInitialized := True;
    FDm := TDmsoftHost.Create;

    if FDmRoot <> '' then
    begin
      SetPathRet := FDm.SetPath(FDmRoot);
      if SetPathRet <> 1 then
      begin
        SetBridgeError(DM_BRIDGE_DM_FAILED, 'init SetPath failed');
        FInitStatus := DM_BRIDGE_DM_FAILED;
        Exit;
      end;
    end;

    ClearBridgeError;
    FInitStatus := DM_BRIDGE_OK;
  except
    on E: Exception do
    begin
      SetBridgeError(DM_BRIDGE_COM_ERROR, E.Message);
      FInitStatus := DM_BRIDGE_COM_ERROR;
    end;
  end;
end;

procedure TDmWorkerThread.FreeDm;
begin
  FreeAndNil(FDm);

  if FCoInitialized then
  begin
    CoUninitialize;
    FCoInitialized := False;
  end;
end;

function TDmWorkerThread.PopRequest: TDmCallRequest;
begin
  FQueueLock.Enter;
  try
    if FQueue.Count = 0 then
      Exit(nil);

    Result := FQueue.Dequeue;
  finally
    FQueueLock.Leave;
  end;
end;

function TDmWorkerThread.QueueIsEmpty: Boolean;
begin
  FQueueLock.Enter;
  try
    Result := FQueue.Count = 0;
  finally
    FQueueLock.Leave;
  end;
end;

function TDmWorkerThread.WaitInit: Integer;
begin
  FInitEvent.WaitFor(INFINITE);
  Result := FInitStatus;
end;

function TDmWorkerThread.Invoke(const Call: TDmsoftCall): Integer;
var
  Request: TDmCallRequest;
begin
  if WaitInit <> DM_BRIDGE_OK then
    Exit(FInitStatus);

  Request := TDmCallRequest.Create(Call);
  try
    FQueueLock.Enter;
    try
      FQueue.Enqueue(Request);
    finally
      FQueueLock.Leave;
    end;

    FQueueEvent.SetEvent;
    Request.WaitDone;
    Result := Request.Status;
  finally
    Request.Free;
  end;
end;

procedure TDmWorkerThread.StopAndWait;
begin
  Terminate;
  FQueueEvent.SetEvent;
  WaitFor;
end;

end.
