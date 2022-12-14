#include "audio_capture_module.h"

AudioCaptureModule* AudioCaptureModule::Create()
{
    auto self = new rtc::RefCountedObject<AudioCaptureModule>();
    if (!self->Initialize()) 
    {
        return NULL;
    }

    self->AddRef();
    return self;
}

int32_t AudioCaptureModule::ActiveAudioLayer(AudioLayer* audioLayer) const
{
    *audioLayer = AudioDeviceModule::kDummyAudio;
    return 0;
}

int32_t AudioCaptureModule::RegisterAudioCallback(webrtc::AudioTransport* audio_callback)
{
    _audio_callback = audio_callback;
    return 0;
}

int32_t AudioCaptureModule::Init()
{
    return 0;
}

int32_t AudioCaptureModule::Terminate()
{
    // Clean up in the destructor. No action here, just success.
    return 0;
}

bool AudioCaptureModule::Initialized() const
{
    return 0;
}

int16_t AudioCaptureModule::PlayoutDevices()
{
    return 0;
}

int16_t AudioCaptureModule::RecordingDevices()
{
    return 0;
}

int32_t AudioCaptureModule::PlayoutDeviceName(uint16_t index,
    char name[webrtc::kAdmMaxDeviceNameSize],
    char guid[webrtc::kAdmMaxGuidSize])
{
    return 0;
}

int32_t AudioCaptureModule::RecordingDeviceName(uint16_t index, 
    char name[webrtc::kAdmMaxDeviceNameSize], 
    char guid[webrtc::kAdmMaxGuidSize])
{
    return 0;
}

int32_t AudioCaptureModule::SetPlayoutDevice(uint16_t index)
{
    return 0;
}

int32_t AudioCaptureModule::SetPlayoutDevice(WindowsDeviceType device)
{
    if (_play_is_initialized)
    {
        return -1;
    }
    else
    {
        return 0;
    }
}

int32_t AudioCaptureModule::SetRecordingDevice(uint16_t index)
{
    return 0;
}

int32_t AudioCaptureModule::SetRecordingDevice(WindowsDeviceType device)
{
    if (_rec_is_initialized)
    {
        return -1;
    }
    else
    {
        return 0;
    }
}

int32_t AudioCaptureModule::PlayoutIsAvailable(bool* available)
{
    return 0;
}

int32_t AudioCaptureModule::InitPlayout()
{
    _play_is_initialized = true;
    return 0;
}

bool AudioCaptureModule::PlayoutIsInitialized() const
{
    return _play_is_initialized;
}

int32_t AudioCaptureModule::RecordingIsAvailable(bool* available)
{
    return 0;
}

int32_t AudioCaptureModule::InitRecording()
{
    _rec_is_initialized = true;
    return 0;
}

bool AudioCaptureModule::RecordingIsInitialized() const
{
    return _rec_is_initialized;
}

int32_t AudioCaptureModule::StartPlayout()
{
    if (!_play_is_initialized) 
    {
        return -1;
    }

    _playing = true;
    return 0;
}

int32_t AudioCaptureModule::StopPlayout()
{
    _playing = false;
    return 0;
}

bool AudioCaptureModule::Playing() const
{
    return _playing;
}

int32_t AudioCaptureModule::StartRecording()
{
    if (!_rec_is_initialized) 
    {
        return -1;
    }

    _recording = true;
    return 0;
}

int32_t AudioCaptureModule::StopRecording()
{
    _recording = false;
    return 0;
}

bool AudioCaptureModule::Recording() const
{
    return _recording;
}

int32_t AudioCaptureModule::InitSpeaker()
{
    return 0;
}

bool AudioCaptureModule::SpeakerIsInitialized() const
{
    return 0;
}

int32_t AudioCaptureModule::InitMicrophone()
{
    // No microphone, just playing from file. Return success.
    return 0;
}

bool AudioCaptureModule::MicrophoneIsInitialized() const
{
    return 0;
}

int32_t AudioCaptureModule::SpeakerVolumeIsAvailable(bool* available)
{
    return 0;
}

int32_t AudioCaptureModule::SetSpeakerVolume(uint32_t volume)
{
    return 0;
}

int32_t AudioCaptureModule::SpeakerVolume(uint32_t* volume) const
{
    return 0;
}

int32_t AudioCaptureModule::MaxSpeakerVolume(uint32_t* maxVolume) const
{
    return 0;
}

int32_t AudioCaptureModule::MinSpeakerVolume(uint32_t* minVolume) const
{
    return 0;
}

int32_t AudioCaptureModule::MicrophoneVolumeIsAvailable(bool* available)
{
    return 0;
}

int32_t AudioCaptureModule::SetMicrophoneVolume(uint32_t volume)
{
    return 0;
}

int32_t AudioCaptureModule::MicrophoneVolume(uint32_t* volume) const
{
    *volume = _current_volume;
    return 0;
}

int32_t AudioCaptureModule::MaxMicrophoneVolume(uint32_t* max_volume) const
{
    *max_volume = 14392;
    return 0;
}

int32_t AudioCaptureModule::MinMicrophoneVolume(uint32_t* min_volume) const
{
    *min_volume = 0;
    return 0;
}

int32_t AudioCaptureModule::SpeakerMuteIsAvailable(bool* available)
{
    return 0;
}

int32_t AudioCaptureModule::SetSpeakerMute(bool enable)
{
    return 0;
}

int32_t AudioCaptureModule::SpeakerMute(bool* enabled) const
{
    return 0;
}

int32_t AudioCaptureModule::MicrophoneMuteIsAvailable(bool* available)
{
    return 0;
}

int32_t AudioCaptureModule::SetMicrophoneMute(bool enable)
{
    return 0;
}

int32_t AudioCaptureModule::MicrophoneMute(bool* enabled) const
{
    return 0;
}

int32_t AudioCaptureModule::StereoPlayoutIsAvailable(bool* available) const
{
    *available = true;
    return 0;
}

int32_t AudioCaptureModule::SetStereoPlayout(bool enable)
{
    return 0;
}

int32_t AudioCaptureModule::StereoPlayout(bool* enabled) const
{
    return 0;
}

int32_t AudioCaptureModule::StereoRecordingIsAvailable(bool* available) const
{
    *available = false;
    return 0;
}

int32_t AudioCaptureModule::SetStereoRecording(bool enable)
{
    if (!enable) 
    {
        return 0;
    }
    else
    {
        return -1;
    }
}

int32_t AudioCaptureModule::StereoRecording(bool* enabled) const
{
    return 0;
}

int32_t AudioCaptureModule::PlayoutDelay(uint16_t* delay_ms) const
{
    *delay_ms = 0;
    return 0;
}

bool AudioCaptureModule::BuiltInAECIsAvailable() const
{
    return false;
}

bool AudioCaptureModule::BuiltInAGCIsAvailable() const
{
    return false;
}

bool AudioCaptureModule::BuiltInNSIsAvailable() const
{
    return false;
}

int32_t AudioCaptureModule::EnableBuiltInAEC(bool enable)
{
    return -1;
}

int32_t AudioCaptureModule::EnableBuiltInAGC(bool enable)
{
    return -1;
}

int32_t AudioCaptureModule::EnableBuiltInNS(bool enable)
{
    return -1;
}

bool AudioCaptureModule::Initialize()
{
    _last_process_time_ms = rtc::TimeMillis();
    return true;
}
