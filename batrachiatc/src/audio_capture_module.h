#ifndef BATRACHIATC_AUDIO_CAPTURE_MODULE_H_
#define BATRACHIATC_AUDIO_CAPTURE_MODULE_H_
#pragma once

#include "modules/audio_device/include/audio_device.h"
#include "modules/audio_mixer/audio_mixer_impl.h"
#include "api/peer_connection_interface.h"

class AudioCaptureModule 
    : public webrtc::AudioDeviceModule
    , public rtc::RefCountInterface
{
public:
    static AudioCaptureModule* Create();
    int32_t ActiveAudioLayer(AudioLayer* audioLayer) const;
    int32_t RegisterAudioCallback(webrtc::AudioTransport* audio_callback);
    
    // Main initialization and termination
    int32_t Init();
    int32_t Terminate();
    bool Initialized() const;
    
    // Device enumeration
    int16_t PlayoutDevices();
    int16_t RecordingDevices();
    int32_t PlayoutDeviceName(uint16_t index,
        char name[webrtc::kAdmMaxDeviceNameSize],
        char guid[webrtc::kAdmMaxGuidSize]);
    int32_t RecordingDeviceName(uint16_t index, 
        char name[webrtc::kAdmMaxDeviceNameSize], 
        char guid[webrtc::kAdmMaxGuidSize]);
        
    // Device selection
    int32_t SetPlayoutDevice(uint16_t index);
    int32_t SetPlayoutDevice(WindowsDeviceType device);
    int32_t SetRecordingDevice(uint16_t index);
    int32_t SetRecordingDevice(WindowsDeviceType device);
    
    // Audio transport initialization
    int32_t PlayoutIsAvailable(bool* available);
    int32_t InitPlayout();
    bool PlayoutIsInitialized() const;
    int32_t RecordingIsAvailable(bool* available);
    int32_t InitRecording();
    bool RecordingIsInitialized() const;
    
    // Audio transport control
    int32_t StartPlayout();
    int32_t StopPlayout();
    
    // True when audio is being pulled by the instance.
    bool Playing() const;
    int32_t StartRecording();
    int32_t StopRecording();
    bool Recording() const;
    
    // Audio mixer initialization
    int32_t InitSpeaker();
    bool SpeakerIsInitialized() const;
    int32_t InitMicrophone();
    bool MicrophoneIsInitialized() const;
    
    // Speaker volume controls
    int32_t SpeakerVolumeIsAvailable(bool* available);
    int32_t SetSpeakerVolume(uint32_t volume);
    int32_t SpeakerVolume(uint32_t* volume) const;
    int32_t MaxSpeakerVolume(uint32_t* maxVolume) const;
    int32_t MinSpeakerVolume(uint32_t* minVolume) const;
    
    // Microphone volume controls
    int32_t MicrophoneVolumeIsAvailable(bool* available);
    int32_t SetMicrophoneVolume(uint32_t volume);
    int32_t MicrophoneVolume(uint32_t* volume) const;
    int32_t MaxMicrophoneVolume(uint32_t* max_volume) const;
    int32_t MinMicrophoneVolume(uint32_t* min_volume) const;
    
    // Speaker mute control
    int32_t SpeakerMuteIsAvailable(bool* available);
    int32_t SetSpeakerMute(bool enable);
    int32_t SpeakerMute(bool* enabled) const;
    
    // Microphone mute control
    int32_t MicrophoneMuteIsAvailable(bool* available);
    int32_t SetMicrophoneMute(bool enable);
    int32_t MicrophoneMute(bool* enabled) const;
    
    // Stereo support
    int32_t StereoPlayoutIsAvailable(bool* available) const;
    int32_t SetStereoPlayout(bool enable);
    int32_t StereoPlayout(bool* enabled) const;
    int32_t StereoRecordingIsAvailable(bool* available) const;
    int32_t SetStereoRecording(bool enable);
    int32_t StereoRecording(bool* enabled) const;
    
    // Playout delay
    int32_t PlayoutDelay(uint16_t* delay_ms) const;
    bool BuiltInAECIsAvailable() const;
    bool BuiltInAGCIsAvailable() const;
    bool BuiltInNSIsAvailable() const;
    
    // Enables the built-in audio effects. Only supported on Android.
    int32_t EnableBuiltInAEC(bool enable);
    int32_t EnableBuiltInAGC(bool enable);
    int32_t EnableBuiltInNS(bool enable);
private:
    bool Initialize();
    
    int32_t _current_volume = 0;
    uint64_t _last_process_time_ms = 0;
    webrtc::AudioMixerImpl* _audio_mixer = NULL;
    webrtc::AudioTransport* _audio_callback = NULL;
    bool _play_is_initialized = false;
    bool _rec_is_initialized = false;
    bool _playing = false;
    bool _recording = false;
};

#endif  // BATRACHIATC_AUDIO_CAPTURE_MODULE_H_
