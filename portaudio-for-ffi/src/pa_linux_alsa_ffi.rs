/* automatically generated by csbindgen */

#[allow(unused)]
use ::std::os::raw::*;

use super::pa_linux_alsa::*;


#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetVersion(

) -> c_int
{
    Pa_GetVersion(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetVersionText(

) -> *const c_char
{
    Pa_GetVersionText(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetVersionInfo(

) -> *const PaVersionInfo
{
    Pa_GetVersionInfo(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetErrorText(
    errorCode: PaError
) -> *const c_char
{
    Pa_GetErrorText(
        errorCode
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_Initialize(

) -> PaError
{
    Pa_Initialize(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_Terminate(

) -> PaError
{
    Pa_Terminate(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetHostApiCount(

) -> PaHostApiIndex
{
    Pa_GetHostApiCount(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetDefaultHostApi(

) -> PaHostApiIndex
{
    Pa_GetDefaultHostApi(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetHostApiInfo(
    hostApi: PaHostApiIndex
) -> *const PaHostApiInfo
{
    Pa_GetHostApiInfo(
        hostApi
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_HostApiTypeIdToHostApiIndex(
    type_: PaHostApiTypeId
) -> PaHostApiIndex
{
    Pa_HostApiTypeIdToHostApiIndex(
        type_
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_HostApiDeviceIndexToDeviceIndex(
    hostApi: PaHostApiIndex,
    hostApiDeviceIndex: c_int
) -> PaDeviceIndex
{
    Pa_HostApiDeviceIndexToDeviceIndex(
        hostApi,
        hostApiDeviceIndex
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetLastHostErrorInfo(

) -> *const PaHostErrorInfo
{
    Pa_GetLastHostErrorInfo(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetDeviceCount(

) -> PaDeviceIndex
{
    Pa_GetDeviceCount(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetDefaultInputDevice(

) -> PaDeviceIndex
{
    Pa_GetDefaultInputDevice(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetDefaultOutputDevice(

) -> PaDeviceIndex
{
    Pa_GetDefaultOutputDevice(

    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetDeviceInfo(
    device: PaDeviceIndex
) -> *const PaDeviceInfo
{
    Pa_GetDeviceInfo(
        device
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_IsFormatSupported(
    inputParameters: *const PaStreamParameters,
    outputParameters: *const PaStreamParameters,
    sampleRate: f64
) -> PaError
{
    Pa_IsFormatSupported(
        inputParameters,
        outputParameters,
        sampleRate
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_OpenStream(
    stream: *mut *mut PaStream,
    inputParameters: *const PaStreamParameters,
    outputParameters: *const PaStreamParameters,
    sampleRate: f64,
    framesPerBuffer: c_ulong,
    streamFlags: PaStreamFlags,
    streamCallback: PaStreamCallback,
    userData: *mut c_void
) -> PaError
{
    Pa_OpenStream(
        stream,
        inputParameters,
        outputParameters,
        sampleRate,
        framesPerBuffer,
        streamFlags,
        streamCallback,
        userData
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_OpenDefaultStream(
    stream: *mut *mut PaStream,
    numInputChannels: c_int,
    numOutputChannels: c_int,
    sampleFormat: PaSampleFormat,
    sampleRate: f64,
    framesPerBuffer: c_ulong,
    streamCallback: PaStreamCallback,
    userData: *mut c_void
) -> PaError
{
    Pa_OpenDefaultStream(
        stream,
        numInputChannels,
        numOutputChannels,
        sampleFormat,
        sampleRate,
        framesPerBuffer,
        streamCallback,
        userData
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_CloseStream(
    stream: *mut PaStream
) -> PaError
{
    Pa_CloseStream(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_SetStreamFinishedCallback(
    stream: *mut PaStream,
    streamFinishedCallback: PaStreamFinishedCallback
) -> PaError
{
    Pa_SetStreamFinishedCallback(
        stream,
        streamFinishedCallback
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_StartStream(
    stream: *mut PaStream
) -> PaError
{
    Pa_StartStream(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_StopStream(
    stream: *mut PaStream
) -> PaError
{
    Pa_StopStream(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_AbortStream(
    stream: *mut PaStream
) -> PaError
{
    Pa_AbortStream(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_IsStreamStopped(
    stream: *mut PaStream
) -> PaError
{
    Pa_IsStreamStopped(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_IsStreamActive(
    stream: *mut PaStream
) -> PaError
{
    Pa_IsStreamActive(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetStreamInfo(
    stream: *mut PaStream
) -> *const PaStreamInfo
{
    Pa_GetStreamInfo(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetStreamTime(
    stream: *mut PaStream
) -> PaTime
{
    Pa_GetStreamTime(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetStreamCpuLoad(
    stream: *mut PaStream
) -> f64
{
    Pa_GetStreamCpuLoad(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_ReadStream(
    stream: *mut PaStream,
    buffer: *mut c_void,
    frames: c_ulong
) -> PaError
{
    Pa_ReadStream(
        stream,
        buffer,
        frames
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_WriteStream(
    stream: *mut PaStream,
    buffer: *const c_void,
    frames: c_ulong
) -> PaError
{
    Pa_WriteStream(
        stream,
        buffer,
        frames
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetStreamReadAvailable(
    stream: *mut PaStream
) -> c_long
{
    Pa_GetStreamReadAvailable(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetStreamWriteAvailable(
    stream: *mut PaStream
) -> c_long
{
    Pa_GetStreamWriteAvailable(
        stream
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_GetSampleSize(
    format: PaSampleFormat
) -> PaError
{
    Pa_GetSampleSize(
        format
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_Pa_Sleep(
    msec: c_long
)
{
    Pa_Sleep(
        msec
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_PaAlsa_InitializeStreamInfo(
    info: *mut PaAlsaStreamInfo
)
{
    PaAlsa_InitializeStreamInfo(
        info
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_PaAlsa_EnableRealtimeScheduling(
    s: *mut PaStream,
    enable: c_int
)
{
    PaAlsa_EnableRealtimeScheduling(
        s,
        enable
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_PaAlsa_GetStreamInputCard(
    s: *mut PaStream,
    card: *mut c_int
) -> PaError
{
    PaAlsa_GetStreamInputCard(
        s,
        card
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_PaAlsa_GetStreamOutputCard(
    s: *mut PaStream,
    card: *mut c_int
) -> PaError
{
    PaAlsa_GetStreamOutputCard(
        s,
        card
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_PaAlsa_SetNumPeriods(
    numPeriods: c_int
) -> PaError
{
    PaAlsa_SetNumPeriods(
        numPeriods
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_PaAlsa_SetRetriesBusy(
    retries: c_int
) -> PaError
{
    PaAlsa_SetRetriesBusy(
        retries
    )
}

#[no_mangle]
pub unsafe extern "C" fn csbindgen_PaAlsa_SetLibraryPathName(
    pathName: *const c_char
)
{
    PaAlsa_SetLibraryPathName(
        pathName
    )
}

    