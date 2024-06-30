using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Threading;
using PortAudioCSharp.AutoGenerated;
using PortAudioCSharp.Exceptions;

[assembly: InternalsVisibleTo("PortAudioCSharp")]
namespace PortAudioCSharp.Wrapper;

internal unsafe class PortAudioOutStrem : IDisposable
{
    private bool _disposed;
    private readonly CancellationTokenSource cancellationTokenSource = new();
    private bool _isInitialized = false;
    private readonly void* _stream = NativeMemory.AllocZeroed((nuint)sizeof(void*));
    private readonly int _deviceIndex;
    private StreamParameters _outputParameter;
    private readonly double _sampleRate;
    private readonly PaStreamFlags _streamFlag;

    public PortAudioOutStrem(int deviceIndex, StreamParameters outputPatameter, double sampleRate, PaStreamFlags streamFlags)
    {
        _deviceIndex = deviceIndex;
        _outputParameter = outputPatameter;
        _sampleRate = sampleRate;
        _streamFlag = streamFlags;
        Initialize();
    }

    private void Initialize()
    {
        var parameter = _outputParameter.ToPaStreamParameters();
        PortAudioException.ThrowIfError(NativeMethods.Pa_IsFormatSupported((PaStreamParameters*)nint.Zero, &parameter, _sampleRate));
        fixed (void** pStream = &_stream)
        {
            PortAudioException.ThrowIfError(NativeMethods.Pa_OpenStream(pStream, (PaStreamParameters*)nint.Zero, &parameter, _sampleRate, new CULong(NativeConsts.paFramesPerBufferUnspecified), new CULong((uint)_streamFlag), &StreamCallBack, (void*)nint.Zero));
        }
        _isInitialized = true;
    }

    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }

    public void Dispose(bool disposing)
    {
        if (_disposed)
        {
            return;
        }

        if (disposing)
        {
            _outputParameter.Dispose();
        }
        if (_isInitialized)
        {
            PortAudioException.ThrowIfError(NativeMethods.Pa_CloseStream(_stream));
        }
        NativeMemory.Free(_stream);
        _disposed = true;
        _isInitialized = true;
    }

    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)])]
    private static int StreamCallBack(void* input, void* output, CULong frameCount, PaStreamCallbackTimeInfo* timeInfo, CULong statusFlag, void* userData) => (int)PaStreamCallbackResult.paComplete;
}
