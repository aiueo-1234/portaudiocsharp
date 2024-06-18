using System.Diagnostics.CodeAnalysis;
using PortAudio.AutoGenerated;
using PortAudio.Wrapper;

namespace PortAudio.Exceptions;

public class PortAudioException : Exception
{
    public PortAudioException() { }
    public PortAudioException(string message) : base(message) { }
    public PortAudioException(string message, Exception inner) : base(message, inner) { }
    public PortAudioException(PaErrorCode paErrorCode) : base(PortAudioWrapper.GetErrorText(paErrorCode)) { }

    public static void ThrowIfError(PaErrorCode paErrorCode)
    {
        if (paErrorCode != PaErrorCode.paNoError)
        {
            throw new PortAudioException(paErrorCode);
        }
    }

    public static void ThrowIfError(int value)
    {
        if (Enum.IsDefined(typeof(PaErrorCode), value))
        {
            ThrowIfError((PaErrorCode)value);
        }
    }

    [DoesNotReturn]
    public static void Throw(PaErrorCode paErrorCode)
    {
        throw new PortAudioException(paErrorCode);
    }
}