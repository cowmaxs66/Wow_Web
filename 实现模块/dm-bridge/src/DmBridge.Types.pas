unit DmBridge.Types;

interface

const
  DM_BRIDGE_CURRENT_ABI_VERSION = 1;

  DM_BRIDGE_OK = 1;
  DM_BRIDGE_DM_FAILED = 0;
  DM_BRIDGE_NOT_INITIALIZED = -1;
  DM_BRIDGE_INVALID_ARG = -2;
  DM_BRIDGE_COM_ERROR = -3;
  DM_BRIDGE_BUFFER_TOO_SMALL = -4;
  DM_BRIDGE_THREAD_ERROR = -5;
  DM_BRIDGE_UNSUPPORTED = -6;

type
  PDmBridgePoint = ^TDmBridgePoint;
  TDmBridgePoint = packed record
    X: Integer;
    Y: Integer;
  end;

  PDmBridgeRect = ^TDmBridgeRect;
  TDmBridgeRect = packed record
    X1: Integer;
    Y1: Integer;
    X2: Integer;
    Y2: Integer;
  end;

  PDmBridgeSize = ^TDmBridgeSize;
  TDmBridgeSize = packed record
    Width: Integer;
    Height: Integer;
  end;

  PDmBridgeFindResult = ^TDmBridgeFindResult;
  TDmBridgeFindResult = packed record
    DmRet: Integer;
    X: Integer;
    Y: Integer;
  end;

implementation

end.
