fn main() {
    #[cfg(windows)]
    windows::build!(
        Windows::Foundation::{TypedEventHandler, EventRegistrationToken, IClosable, IAsyncOperation, TimeSpan, MemoryBuffer, IMemoryBufferReference},
        Windows::Devices::Midi::*,
        Windows::Devices::Enumeration::{DeviceInformation, DeviceInformationCollection},
        Windows::Storage::Streams::{Buffer, DataWriter},
        Windows::Win32::System::WinRT::IMemoryBufferByteAccess,
        Windows::Win32::Media::Multimedia::{
            midiInAddBuffer, midiInClose, midiInGetDevCapsW, midiInGetNumDevs,
            midiInOpen, midiInPrepareHeader, midiInReset, midiInStart,
            midiInStop, midiInUnprepareHeader, midiOutClose,
            midiOutGetDevCapsW, midiOutGetNumDevs, midiOutLongMsg, midiOutOpen,
            midiOutPrepareHeader, midiOutReset, midiOutShortMsg,
            midiOutUnprepareHeader, midiInMessage, midiOutMessage,
            HMIDIIN, HMIDIOUT, MIDIHDR, MIDIINCAPSW, MIDIOUTCAPSW,
            MMSYSERR_NOERROR, MMSYSERR_ALLOCATED, MMSYSERR_BADDEVICEID,
            MIDIERR_NOTREADY, MIDIERR_STILLPLAYING,
            DRV_QUERYDEVICEINTERFACE, DRV_QUERYDEVICEINTERFACESIZE,
            MM_MIM_DATA, MM_MIM_LONGDATA, MM_MIM_LONGERROR
    },
    );
}
