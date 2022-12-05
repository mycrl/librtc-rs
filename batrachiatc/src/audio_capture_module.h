#ifndef BATRACHIATC_AUDIO_CAPTURE_MODULE_H_
#define BATRACHIATC_AUDIO_CAPTURE_MODULE_H_
#pragma once

#include "modules/audio_device/include/audio_device.h"
#include "api/peer_connection_interface.h"

class AudioCaptureModule 
    : public webrtc::AudioDeviceModule
    , public rtc::RefCountInterface
{
public:
    static AudioCaptureModule* Create()
    {
        auto self = new rtc::RefCountedObject<AudioCaptureModule>();
        if (!self->Initialize()) 
        {
            return NULL;
        }

        self->AddRef();
        return self;
    }

    int32_t ActiveAudioLayer(AudioLayer* audioLayer) const
    {
        return 0;
    }

    int32_t RegisterAudioCallback(webrtc::AudioTransport* audio_callback)
    {
        _audio_callback = audio_callback;
        return 0;
    }

    int32_t Init()
    {
        return 0;
    }

    int32_t Terminate()
    {
        // Clean up in the destructor. No action here, just success.
        return 0;
    }

    bool Initialized() const
    {
        return 0;
    }

    int16_t PlayoutDevices()
    {
        return 0;
    }

    int16_t RecordingDevices()
    {
        return 0;
    }

    int32_t PlayoutDeviceName(uint16_t index,
        char name[webrtc::kAdmMaxDeviceNameSize],
        char guid[webrtc::kAdmMaxGuidSize])
    {
        return 0;
    }

    int32_t RecordingDeviceName(uint16_t index, 
        char name[webrtc::kAdmMaxDeviceNameSize], 
        char guid[webrtc::kAdmMaxGuidSize])
    {
        return 0;
    }

    int32_t SetPlayoutDevice(uint16_t index)
    {
        return 0;
    }

    int32_t SetPlayoutDevice(WindowsDeviceType device)
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

    int32_t SetRecordingDevice(uint16_t index)
    {
        return 0;
    }

    int32_t SetRecordingDevice(WindowsDeviceType device)
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

    int32_t PlayoutIsAvailable(bool* available)
    {
        return 0;
    }

    int32_t InitPlayout()
    {
        _play_is_initialized = true;
        return 0;
    }

    bool PlayoutIsInitialized() const
    {
        return _play_is_initialized;
}

    int32_t RecordingIsAvailable(bool* available)
    {
        return 0;
    }

    int32_t InitRecording()
    {
        _rec_is_initialized = true;
        return 0;
    }

    bool RecordingIsInitialized() const
    {
        return _rec_is_initialized;
    }

    int32_t StartPlayout()
    {
        if (!_play_is_initialized) 
        {
            return -1;
        }

        _playing = true;
        return 0;
    }

    int32_t StopPlayout()
    {
        _playing = false;
        return 0;
    }

    bool Playing() const
    {
        return _playing;
    }

    int32_t StartRecording()
    {
        if (!_rec_is_initialized) 
        {
            return -1;
        }

        _recording = true;
        return 0;
    }

    int32_t StopRecording()
    {
        _recording = false;
        return 0;
    }

    bool Recording() const
    {
        return _recording;
    }

    int32_t InitSpeaker()
    {
        return 0;
    }

    bool SpeakerIsInitialized() const
    {
        return 0;
    }

    int32_t InitMicrophone()
    {
        // No microphone, just playing from file. Return success.
        return 0;
    }

    bool MicrophoneIsInitialized() const
    {
        return 0;
    }

    int32_t SpeakerVolumeIsAvailable(bool* available)
    {
        return 0;
    }

    int32_t SetSpeakerVolume(uint32_t volume)
    {
        return 0;
    }

    int32_t SpeakerVolume(uint32_t* volume) const
    {
        return 0;
    }

    int32_t MaxSpeakerVolume(uint32_t* maxVolume) const
    {
        return 0;
    }

    int32_t MinSpeakerVolume(uint32_t* minVolume) const
    {
        return 0;
    }

    int32_t MicrophoneVolumeIsAvailable(bool* available)
    {
        return 0;
    }

    int32_t SetMicrophoneVolume(uint32_t volume)
    {
        return 0;
    }

    int32_t MicrophoneVolume(uint32_t* volume) const
    {
        *volume = _current_volume;
        return 0;
    }

    int32_t MaxMicrophoneVolume(uint32_t* max_volume) const
    {
        *max_volume = 14392;
        return 0;
    }

    int32_t MinMicrophoneVolume(uint32_t* min_volume) const
    {
        *min_volume = 0;
        return 0;
    }

    int32_t SpeakerMuteIsAvailable(bool* available)
    {
        return 0;
    }

    int32_t SetSpeakerMute(bool enable)
    {
        return 0;
    }

    int32_t SpeakerMute(bool* enabled) const
    {
        return 0;
    }

    int32_t MicrophoneMuteIsAvailable(bool* available)
    {
        return 0;
    }

    int32_t SetMicrophoneMute(bool enable)
    {
        return 0;
    }

    int32_t MicrophoneMute(bool* enabled) const
    {
        return 0;
    }

    int32_t StereoPlayoutIsAvailable(bool* available) const
    {
        *available = true;
        return 0;
    }

    int32_t SetStereoPlayout(bool enable)
    {
        return 0;
    }

    int32_t StereoPlayout(bool* enabled) const
    {
        return 0;
    }

    int32_t StereoRecordingIsAvailable(bool* available) const
    {
        *available = false;
        return 0;
    }

    int32_t SetStereoRecording(bool enable)
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

    int32_t StereoRecording(bool* enabled) const
    {
        return 0;
    }

    int32_t PlayoutDelay(uint16_t* delay_ms) const
    {
        *delay_ms = 0;
        return 0;
    }

    bool BuiltInAECIsAvailable() const
    {
        return false;
    }

    bool BuiltInAGCIsAvailable() const
    {
        return false;
    }

    bool BuiltInNSIsAvailable() const
    {
        return false;
    }

    int32_t EnableBuiltInAEC(bool enable)
    {
        return -1;
    }

    int32_t EnableBuiltInAGC(bool enable)
    {
        return -1;
    }

    int32_t EnableBuiltInNS(bool enable)
    {
        return -1;
    }
private:
    bool Initialize()
    {
        _last_process_time_ms = rtc::TimeMillis();
        return true;
    }

    webrtc::AudioTransport* _audio_callback;
    uint64_t _last_process_time_ms;
    bool _play_is_initialized = false;
    bool _rec_is_initialized = false;
    bool _playing = false;
    bool _recording = false;
    int32_t _current_volume;
};

#endif  // BATRACHIATC_AUDIO_CAPTURE_MODULE_H_