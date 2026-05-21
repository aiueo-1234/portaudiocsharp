using PortAudioCSharp.Devices;
using PortAudioCSharp.Services;

namespace PortAudioCSharp.Sink;

public class PortAudioPlayer
{
    private readonly PortAudioDevice _device;

    public PortAudioPlayer(PortAudioDevice device)
    {
        _device = device;
    }
}
