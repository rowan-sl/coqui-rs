use std::fs;
use std::io::{self, Write};

use coqui_tts::Synthesizer;
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};

// use pyo3::{prelude::*, types::{PyDict, PyList}};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    println!("Starting TTS");
    let mut synth = Synthesizer::new("tts_models/en/ljspeech/tacotron2-DDC", true);
    print!("Ready\n>>");
    io::stdout().flush().unwrap();
    for input in io::stdin().lines() {
        let mut input = input.unwrap();
        if input.is_empty() {
            continue;
        }
        // if !['.', '?', '!'].contains(&input.chars().last().unwrap()) {
        // input.push('.')
        // }
        let audio = synth.tts(&input);
        let rate = synth.sample_rate();
        wav::write(
            wav::Header::new(wav::WAV_FORMAT_IEEE_FLOAT, 1, rate as u32, 32),
            &wav::BitDepth::ThirtyTwoFloat(audio.clone()),
            &mut fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open("cursed_tts.wav")
                .unwrap(),
        )
        .unwrap();
        // break;
        println!("playing audio at rate {}", rate);
        sink.append(SamplesBuffer::new(1, rate as u32, audio.clone()));
        sink.sleep_until_end();
        print!("Ready\n>>");
        io::stdout().flush().unwrap();
    }

    //     let audio = Python::aq(|py| {
    //         // py.eval("print(\"Hello, World!\")", None, None).unwrap();
    //         // let tts_manage = py.import("TTS.utils.manage").unwrap();
    //         // let manager_class = tts_manage.getattr("ModelManager").unwrap();
    //         // let manager_instance = manager_class.call0();

    //         let locals = PyDict::new(py);
    //         let globals = PyDict::new(py);
    //         py.run(r#"
    // from TTS.utils.synthesizer import Synthesizer
    // from TTS.utils.manage import ModelManager
    //         "#, Some(globals), Some(locals)).unwrap();

    //         py.run(r#"
    // # create instance of the coqui tts model manager
    // manager = ModelManager()
    // # download the model
    // (
    //     model_path,
    //     config_path,
    //     model_item,
    // ) = manager.download_model("tts_models/en/ljspeech/tacotron2-DDC")
    // # download the vocoder
    // vocoder_path, vocoder_config_path, _ = manager.download_model(
    //     model_item["default_vocoder"]
    // )
    // # create the coqui tts instance
    // coqui_tts = Synthesizer(
    //     model_path,
    //     config_path,
    //     vocoder_checkpoint=vocoder_path,
    //     vocoder_config=vocoder_config_path,
    //     use_cuda=False
    // )
    //         "#, Some(globals), Some(locals)).unwrap();

    //         let tts = locals.get_item("coqui_tts").unwrap();
    //         let example_text = "Hi.";
    //         let audio = tts.call_method1("tts", (example_text,)).unwrap().downcast::<PyList>().unwrap();
    //         let r_audio = audio.extract::<Vec<f32>>().unwrap();
    //         r_audio
    //     });

    //     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //     let sink = Sink::try_new(&stream_handle).unwrap();

    //     for i in 0..10 {
    //         // Add a dummy source of the sake of the example.
    //         sink.append(SamplesBuffer::new(1, 22050, audio.clone()));

    //         // The sound plays in a separate thread. This call will block the current thread until the sink
    //         // has finished playing all its queued sounds.
    //         sink.sleep_until_end();
    //     }
}
