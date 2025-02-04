# Gemini API

Rust Library for Google Gemini API.

## Usage

call `gemini-api::get_models` to get a list of available models.

call `gemini-api::model::GeminiModel::new_default_model` to create a new instance of default Gemini Model.

call `gemini-api::model::Gemini::new` to create a new instance of a Gemini Api to chat with gemini.

call `gemini-api::model::Gemini::set_system_instruction` to set a system instruction for the Gemini Api.

call `gemini-api::model::Gemini::set_options` to set generation config for the Gemini Api.

call `gemini-api::model::Gemini::start_chat` to start a conversation with gemini.

call `gemini-api::model::Gemini::rebuild` to start a conversation with gemini.

call `gemini-api::model::Gemini::send_message` to chat with gemini.

call `gemini-api::model::Gemini::send_simple_message` to send a text message to gemini.

### feature `blocking`

call `gemini-api::model::GeminiModel::new_default_model` to create a new instance of default Gemini Model.

call `gemini-api::model::blocking::Gemini::new` to create a new instance of a blocking Gemini Api to chat with gemini.

call `gemini-api::model::blocking::Gemini::set_system_instruction` to set a system instruction for the blocking Gemini Api.

call `gemini-api::model::blocking::Gemini::set_options` to set generation config for the blocking Gemini Api.

call `gemini-api::model::blocking::Gemini::start_chat` to start a conversation with blocking gemini.

call `gemini-api::model::blocking::Gemini::rebuild` to start a conversation with blocking gemini.

call `gemini-api::model::blocking::Gemini::send_message` to chat with blocking gemini.

call `gemini-api::model::blocking::Gemini::send_simple_message` to send a text message to blocking gemini.

### feature `image_analysis`

call `gemini-api::model::Gemini::send_image_message` to send an image and text message to gemini.

call `gemini-api::model::blocking::Gemini::send_image_message` to send an image and text message to blocking gemini.
