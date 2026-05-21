using System.IO.Pipelines;


class WaveFileReader : IDisposable
{
    Stream _stream;
    public Stream Stream => _stream;
    public WaveFileReader(string filePath)
    {
        using var fs = File.OpenRead(filePath);
        using var reader = new BinaryReader(fs, System.Text.Encoding.UTF8, true);

        // Validate RIFF/WAVE header
        var riff = reader.ReadBytes(4);
        if (!riff.SequenceEqual("RIFF"u8))
            throw new InvalidDataException("Not a RIFF file");
        reader.ReadInt32(); // file size
        var wave = reader.ReadBytes(4);
        if (!wave.SequenceEqual("WAVE"u8))
            throw new InvalidDataException("Not a WAVE file");

        // Find "data" chunk
        while (fs.Position < fs.Length)
        {
            var chunkId = reader.ReadBytes(4);
            if (chunkId.Length < 4) break;
            var chunkSize = reader.ReadInt32();
            if (chunkId.SequenceEqual("data"u8))
            {
                var data = reader.ReadBytes(chunkSize);
                _stream = new MemoryStream(data, 0, data.Length, writable: false, publiclyVisible: true);
                _stream.Position = 0;
                return;
            }
            else
            {
                // Skip this chunk (account for odd chunk sizes)
                var toSkip = chunkSize;
                if (toSkip < 0) break;
                fs.Position += toSkip;
                if ((chunkSize & 1) == 1 && fs.Position < fs.Length)
                    fs.Position += 1; // pad byte
            }
        }

        throw new InvalidDataException("No data chunk found in WAV file");
    }

    void IDisposable.Dispose()
    {
        _stream.Dispose();
    }
}
