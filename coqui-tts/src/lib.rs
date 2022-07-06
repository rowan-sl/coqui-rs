//! Rust bindings for the coqui-TTS python library for Text-To-Speech

use std::borrow::Borrow;

use pyo3::{prelude::*, types::{PyDict, PyList}};

/// TTS Synthesizer. equivilant to `TTS.utils.synthesizer.Synthesizer`
#[derive(Debug)]
pub struct Synthesizer {
    locals: Py<PyDict>,
}

impl Synthesizer {
    /// Create a new Synthesizer, performing startup initialization (this method is NOT cheap to call, expect a few SECONDS of runtime)
    ///
    /// this will also download apropreate models if they are missing
    ///
    /// # Arguments
    ///
    /// model: the name of the TTS model to use. see https://github.com/coqui-ai/TTS for models.
    ///
    /// # Note
    ///
    /// this may spew out some text to stdout about initialization,
    /// this is from the python library and there is nothing that can be done about it
    ///
    pub fn new(model: &str) -> Self {
        Python::with_gil(|py| {
            let locals: Py<PyDict> = PyDict::new(py).into();
            locals.as_ref(py).borrow().setattr("model_name", model).unwrap();
            py.run(r#"
from TTS.utils.synthesizer import Synthesizer
from TTS.utils.manage import ModelManager
# create instance of the coqui tts model manager
manager = ModelManager()
# download the model
(
    model_path,
    config_path,
    model_item,
) = manager.download_model(model_name)
# download the vocoder
vocoder_path, vocoder_config_path, _ = manager.download_model(
    model_item["default_vocoder"]
)
# create the coqui tts instance
coqui_tts = Synthesizer(
    model_path,
    config_path,
    vocoder_checkpoint=vocoder_path,
    vocoder_config=vocoder_config_path,
    use_cuda=False
)
            "#, None, Some(locals.as_ref(py).borrow())).unwrap();
            Self { locals }
        })
    }

    /// Synthesize some audio.
    ///
    /// # Returned format
    /// channels: 1?
    /// rate: see [`Synthesizer::sample_rate`]
    ///
    pub fn tts(&mut self, text: &str) -> Vec<f32> {
        Python::with_gil(|py| {
            let tts = self.locals.as_ref(py).borrow().get_item("coqui_tts").unwrap();
            let audio = tts.call_method1("tts", (text,)).unwrap().downcast::<PyList>().unwrap();
            audio.extract::<Vec<f32>>().unwrap()
        })
    }

    pub fn sample_rate(&mut self) -> u64 {
        Python::with_gil(|py| {
            let tts = self.locals.as_ref(py).borrow().get_item("coqui_tts").unwrap();
            tts.getattr("output_sample_rate").unwrap().extract::<u64>().unwrap()
        })
    }
}
