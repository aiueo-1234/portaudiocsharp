using System.Runtime.InteropServices;
using PortAudioCSharp.AutoGenerated;
using PortAudioCSharp.Utils;

namespace PortAudioCSharp.Wrapper;

public struct HostErrorInfo
{
    public PaHostApiTypeId HostApiType { get; }
    public CLong ErrorCode { get; }
    public string ErrorText { get; }

    internal unsafe HostErrorInfo(PaHostErrorInfo* paHostErrorInfo)
    {
        HostApiType = paHostErrorInfo->hostApiType;
        ErrorCode = paHostErrorInfo->errorCode;
        ErrorText = Helper.ConvertText(paHostErrorInfo->errorText);
    }
}
