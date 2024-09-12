# Gemini API

Rust Library for Google Gemini API.

## Usage

call `gemini-api::get_models` to get a list of available models.

call `gemini-api::model::Gemini::new` to create a new instance of a Gemini Api to chat with gemini.

call `gemini-api::model::Gemini::set_system_instruction` to set a system instruction for the Gemini Api.

call `gemini-api::model::Gemini::set_options` to set generation config for the Gemini Api.

### feature `blocking`

call `gemini-api::model::blocking::Gemini::new` to create a new instance of a blocking Gemini Api to chat with gemini.

call `gemini-api::model::Gemini::new` to create a new instance of a blocking Gemini Api to chat with gemini.

call `gemini-api::model::Gemini::set_system_instruction` to set a system instruction for the blocking Gemini Api.

call `gemini-api::model::Gemini::set_options` to set generation config for the blocking Gemini Api.
