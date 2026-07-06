unit DmBridge.Worker;

interface

uses
  System.SysUtils,
  System.Win.ComObj,
  Winapi.Windows,
  Winapi.ActiveX,
  DmBridge.Types,
  DmBridge.Errors,
  DmBridge.Dmsoft;

type
  TDmsoftCall = reference to procedure(Dm: TDmsoftHost);

function WorkerInit(const DmRoot: string): Integer;
function WorkerShutdown: Integer;
function WorkerInvoke(const Call: TDmsoftCall): Integer;
function WorkerGetLastDmError(out ErrorCode: Integer): Integer;

implementation

var
  GDm: TDmsoftHost = nil;
  GInitialized: Boolean = False;
  GOwnerThreadId: Cardinal = 0;

function EnsureOwnerThread: Boolean;
begin
  Result := (GOwnerThreadId = 0) or (GOwnerThreadId = GetCurrentThreadId);
end;

function WorkerInit(const DmRoot: string): Integer;
var
  SetPathRet: Integer;
begin
  if GInitialized then
    Exit(DM_BRIDGE_OK);

  if not EnsureOwnerThread then
  begin
    SetBridgeError(DM_BRIDGE_THREAD_ERROR, 'DmBridge 必须在同一线程初始化和调用');
    Exit(DM_BRIDGE_THREAD_ERROR);
  end;

  try
    OleCheck(CoInitializeEx(nil, COINIT_APARTMENTTHREADED));
    GOwnerThreadId := GetCurrentThreadId;
    GDm := TDmsoftHost.Create;

    // P2-S03 使用最小直接 STA 模式：所有调用必须发生在初始化线程。
    // 输入：可选大漠资源根目录。
    // 输出：已创建的 dm.dmsoft COM 对象。
    // 边界：P2-S04 Rust 多线程接入前必须升级为真正 STA Worker 队列。
    if DmRoot <> '' then
    begin
      SetPathRet := GDm.SetPath(DmRoot);
      if SetPathRet <> 1 then
      begin
        SetBridgeError(DM_BRIDGE_DM_FAILED, 'init SetPath failed');
        Exit(DM_BRIDGE_DM_FAILED);
      end;
    end;

    GInitialized := True;
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

function WorkerShutdown: Integer;
begin
  if not GInitialized then
    Exit(DM_BRIDGE_OK);

  if not EnsureOwnerThread then
  begin
    SetBridgeError(DM_BRIDGE_THREAD_ERROR, 'DmBridge shutdown 必须在初始化线程调用');
    Exit(DM_BRIDGE_THREAD_ERROR);
  end;

  try
    FreeAndNil(GDm);
    GInitialized := False;
    GOwnerThreadId := 0;
    CoUninitialize;
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
  if (not GInitialized) or (GDm = nil) then
  begin
    SetBridgeError(DM_BRIDGE_NOT_INITIALIZED, 'DmBridge is not initialized');
    Exit(DM_BRIDGE_NOT_INITIALIZED);
  end;

  if not EnsureOwnerThread then
  begin
    SetBridgeError(DM_BRIDGE_THREAD_ERROR, 'minimal bridge allows calls only on init thread');
    Exit(DM_BRIDGE_THREAD_ERROR);
  end;

  try
    Call(GDm);
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

end.
