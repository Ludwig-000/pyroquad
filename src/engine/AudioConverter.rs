use std::io::Cursor;
use macroquad::Error;
use symphonia::core::audio::Signal;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};
use hound::{WavWriter, WavSpec, SampleFormat};

use crate::engine::PError::PError;

// Converts audio-formats to .wav, since the macroquad sound engine only supports .wav
//
// TODO: directly convert files to bytes.
pub fn ensure_wav(data: Vec<u8>) -> Result<Vec<u8>, PError> {
    let cursor = Cursor::new(data);
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

    let probed = get_probe()
        .format(&Hint::new(), mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e: SymphoniaError| { PError::from(e)
        })?;


    let mut format = probed.format;
 
    
    let track = format
        .default_track()
        .ok_or_else(|| PError::BasicErr("No default audio track found".to_string()))?;

    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e: SymphoniaError| PError::from(e))?;

    let mut wav_cursor = Cursor::new(Vec::new());

    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or_else(|| Error::from("No sample rate"))?;
    let channels = track
        .codec_params
        .channels
        .ok_or_else(|| Error::from("No channel info"))?
        .count() as u16;

    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::new(&mut wav_cursor, spec)
        .map_err( PError::from )?;

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::IoError(_)) => break, // End of stream
            Err(_) => return Err(PError::BasicErr("Packet read error".to_string())),
        };

        let decoded = match decoder.decode(&packet) {
            Ok(audio_buf) => audio_buf,
            Err(SymphoniaError::DecodeError(_)) => continue, // Skip corrupt frame
            Err(_) => return Err(PError::BasicErr("Decode error".to_string())),
        };

        use symphonia::core::audio::AudioBufferRef;
        match decoded {
            AudioBufferRef::F32(buf) => {
                for s in buf.chan(0) {
                    let sample = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    writer.write_sample(sample).map_err(|e: hound::Error| PError::from(e))?;
                }
            }
            AudioBufferRef::S16(buf) => {
                for s in buf.chan(0) {
                    writer.write_sample(*s).map_err( PError::from )?;
                }
            }
            _ => return Err(PError::BasicErr("Unsupported audio buffer format".to_string())),
        }
    }

    writer.finalize().map_err( PError::from )?;

    Ok(wav_cursor.into_inner())
}
