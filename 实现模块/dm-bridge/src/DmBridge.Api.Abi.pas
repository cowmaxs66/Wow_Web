unit DmBridge.Api.Abi;

interface

{$O-}

function dm_bridge_abi_version: Integer; stdcall;

implementation

uses
  DmBridge.Types;

function dm_bridge_abi_version: Integer; stdcall;
var
  Version: Integer;
begin
  // ABI 版本函数必须生成真实函数体。
  // 输入：无。
  // 输出：当前 C ABI 版本号。
  // 边界：该函数用于 Rust 加载后第一步校验，不能触发 Worker 或 COM 初始化。
  Version := DM_BRIDGE_CURRENT_ABI_VERSION;
  Result := Version;
end;

end.
