unit DmBridge.Worker;

interface

uses
  System.SysUtils,
  System.SyncObjs,
  DmBridge.Types,
  DmBridge.Errors,
  DmBridge.Dmsoft,
  DmBridge.Worker.Types,
  DmBridge.Worker.Thread;

type
  TDmsoftCall = DmBridge.Worker.Types.TDmsoftCall;

function WorkerInit(const DmRoot: string): Integer;
function WorkerShutdown: Integer;
function WorkerInvoke(const Call: TDmsoftCall): Integer;
function WorkerGetLastDmError(out ErrorCode: Integer): Integer;

implementation

var
  GWorker: TDmWorkerThread = nil;
  GWorkerLock: TCriticalSection = nil;

function WorkerInit(const DmRoot: string): Integer;
var
  NewWorker: TDmWorkerThread;
begin
  NewWorker := nil;
  try
    GWorkerLock.Enter;
  except
    on E: Exception do
    begin
      SetBridgeError(DM_BRIDGE_COM_ERROR, E.Message);
      Exit(DM_BRIDGE_COM_ERROR);
    end;
  end;

  try
    try
      if GWorker <> nil then
        Exit(DM_BRIDGE_OK);

      NewWorker := TDmWorkerThread.Create(DmRoot);
      NewWorker.Start;
      Result := NewWorker.WaitInit;
      if Result = DM_BRIDGE_OK then
      begin
        GWorker := NewWorker;
        Exit;
      end;

      NewWorker.StopAndWait;
      FreeAndNil(NewWorker);
    except
      on E: Exception do
      begin
        if NewWorker <> nil then
        begin
          NewWorker.StopAndWait;
          FreeAndNil(NewWorker);
        end;
        SetBridgeError(DM_BRIDGE_COM_ERROR, E.Message);
        Result := DM_BRIDGE_COM_ERROR;
      end;
    end;
  finally
    GWorkerLock.Leave;
  end;
end;

function WorkerShutdown: Integer;
var
  WorkerToStop: TDmWorkerThread;
begin
  GWorkerLock.Enter;
  try
    WorkerToStop := GWorker;
    GWorker := nil;
  finally
    GWorkerLock.Leave;
  end;

  if WorkerToStop = nil then
    Exit(DM_BRIDGE_OK);

  try
    WorkerToStop.StopAndWait;
    WorkerToStop.Free;
    ClearBridgeError;
    Result := DM_BRIDGE_OK;
  except
    on E: Exception do
    begin
      SetBridgeError(DM_BRIDGE_COM_ERROR, E.Message);
      Result := DM_BRIDGE_COM_ERROR;
    end;
  end;
end;

function WorkerInvoke(const Call: TDmsoftCall): Integer;
begin
  GWorkerLock.Enter;
  try
    if GWorker = nil then
    begin
      SetBridgeError(DM_BRIDGE_NOT_INITIALIZED, 'DmBridge is not initialized');
      Exit(DM_BRIDGE_NOT_INITIALIZED);
    end;

    // 对外函数可以来自任意线程，但实际 COM 调用只投递到内部 STA Worker。
    // 输入：需要访问 dm.dmsoft 的闭包。
    // 输出：Bridge 状态码。
    // 边界：这里保持同步等待，避免调用方在闭包捕获变量释放后 Worker 才执行。
    Result := GWorker.Invoke(Call);
  finally
    GWorkerLock.Leave;
  end;
end;

function WorkerGetLastDmError(out ErrorCode: Integer): Integer;
var
  LocalErrorCode: Integer;
begin
  LocalErrorCode := 0;
  Result := WorkerInvoke(
    procedure(Dm: TDmsoftHost)
    begin
      LocalErrorCode := Dm.GetLastError;
    end);
  ErrorCode := LocalErrorCode;
end;

initialization
  GWorkerLock := TCriticalSection.Create;

finalization
  if GWorker <> nil then
  begin
    GWorker.StopAndWait;
    FreeAndNil(GWorker);
  end;
  FreeAndNil(GWorkerLock);

end.
