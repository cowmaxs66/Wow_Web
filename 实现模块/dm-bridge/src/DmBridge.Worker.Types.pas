unit DmBridge.Worker.Types;

interface

uses
  DmBridge.Dmsoft;

type
  TDmsoftCall = reference to procedure(Dm: TDmsoftHost);

implementation

end.
