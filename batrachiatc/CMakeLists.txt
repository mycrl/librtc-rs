cmake_minimum_required(VERSION 3.20)
project(batrachiatc)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Debug")
endif()

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True) 

add_library(batrachiatc STATIC
    src/base.h
    src/base.cpp
    src/media_stream_track.h
    src/media_stream_track.cpp
    src/observer.cpp
    src/observer.h
    src/peer_connection.cpp
    src/peer_connection.h
    src/peer_connection_config.h
    src/peer_connection_config.cpp
    src/session_description.h
    src/session_description.cpp
    src/ice_candidate.h
    src/ice_candidate.cpp
    src/data_channel.h
    src/data_channel.cpp)
    
include_directories(
    third_party/webrtc/src/third_party/libyuv/include
    third_party/webrtc/src/third_party/abseil-cpp
    third_party/webrtc/src)
    
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_compile_definitions(NDEBUG)
else()
    add_compile_definitions(DEBUG)
endif()

if(WIN32)
    add_compile_definitions(
        _CONSOLE
        USE_AURA=1
        _HAS_EXCEPTIONS=0
        __STD_C
        _CRT_RAND_S
        _CRT_SECURE_NO_DEPRECATE
        _SCL_SECURE_NO_DEPRECATE
        _ATL_NO_OPENGL
        _WINDOWS
        CERT_CHAIN_PARA_HAS_EXTRA_FIELDS
        PSAPI_VERSION=2
        WIN32
        _SECURE_ATL
        WINUWP
        __WRL_NO_DEFAULT_LIB__
        WINAPI_FAMILY=WINAPI_FAMILY_PC_APP
        WIN10=_WIN32_WINNT_WIN10
        WIN32_LEAN_AND_MEAN
        NOMINMAX
        _UNICODE
        UNICODE
        NTDDI_VERSION=NTDDI_WIN10_RS2
        _WIN32_WINNT=0x0A00
        WINVER=0x0A00
        NVALGRIND
        DYNAMIC_ANNOTATIONS_ENABLED=0
        WEBRTC_ENABLE_PROTOBUF=0
        WEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE
        RTC_ENABLE_VP9
        HAVE_SCTP
        WEBRTC_LIBRARY_IMPL
        WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS=0
        WEBRTC_WIN
        ABSL_ALLOCATOR_NOTHROW=1
        WEBRTC_VIDEO_CAPTURE_WINRT)
        
    if(CMAKE_BUILD_TYPE STREQUAL "Release")
        set_property(TARGET batrachiatc PROPERTY
            MSVC_RUNTIME_LIBRARY MultiThreaded)
    else()
        set_property(TARGET batrachiatc PROPERTY
            MSVC_RUNTIME_LIBRARY MultiThreadedDebug)
    endif()
endif()

if(APPLE)
    add_compile_definitions(
        APPLE
        WEBRTC_MAC
        WEBRTC_IOS
        WEBRTC_POSIX)
endif()
