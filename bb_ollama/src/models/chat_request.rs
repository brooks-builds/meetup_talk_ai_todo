use eyre::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::api;

use super::{message::Message, options::ChatRequestOptions, tool::Tool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
    pub raw: Option<bool>,
    pub tools: Vec<Tool>,
    pub options: Option<ChatRequestOptions>,
}

impl Chat {
    pub fn new(model: impl Into<String>, options: Option<ChatRequestOptions>) -> Self {
        let messages = vec![];
        let stream = Some(false);
        let raw = Some(false);
        let tools = vec![];

        Self {
            model: model.into(),
            messages,
            stream,
            raw,
            tools,
            options,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message)
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    pub fn send(&mut self) -> Result<Message> {
        let message = api::send_to_ollama(&self).context("Sending chat to Ollama")?;

        if self
            .options
            .as_ref()
            .is_some_and(|options| options.save_messages)
        {
            self.add_message(message.clone());
        }

        Ok(message)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::models::tool::Property;
    #[allow(unused_imports)]
    use eyre::OptionExt;

    #[test]
    fn can_send_and_receive_chat() -> Result<()> {
        let message = Message::new_user("Hello");
        let model = "llama3.1:8b-instruct-fp16";
        let options = ChatRequestOptions::new().seed(123);
        let mut chat = Chat::new(model, Some(options));

        chat.add_message(message);

        let message_response = chat.send()?;

        assert_eq!(message_response.content, "How can I assist you today?");

        Ok(())
    }

    #[test]
    fn can_get_a_tool_call_from_ollama() -> Result<()> {
        let message = Message::new_user("What is the weather like in New York?");
        let model = "llama3.1:8b-instruct-fp16";
        let options = ChatRequestOptions::new().seed(123);
        let mut chat = Chat::new(model, Some(options));
        let weather_tool_name = "check_weather";
        let weather_tool = Tool::new()
            .function_name(weather_tool_name)
            .function_description("Get the weather in farhenheight degrees for any given location")
            .add_function_property("location", Property::new_string("location for where you want to check the weather. Use the format 'city, state' if the location is in the United States. Otherwise use 'city, country'"))
            .add_required_property("location")
            .build();

        chat.add_tool(weather_tool);
        chat.add_message(message);
        let received_message = chat.send()?;

        assert!(received_message.tool_calls.is_some());
        assert_eq!(received_message.content, "");

        let tool_call = received_message.tool_calls.unwrap();

        assert_eq!(tool_call.len(), 1);

        let tool_call = &tool_call[0];

        assert_eq!(tool_call.function.name, weather_tool_name);

        let tool_call_location = tool_call.function.arguments.get("location");

        assert!(tool_call_location.is_some());

        assert_eq!(tool_call_location.unwrap(), "New York, NY");

        Ok(())
    }

    #[test]
    fn should_use_info_from_tool_call() -> Result<()> {
        let message = Message::new_user("What is the weather like in New York?");
        let model = "llama3.1:8b-instruct-fp16";
        let options = ChatRequestOptions::new().seed(123);
        let mut chat = Chat::new(model, Some(options));
        let weather_tool_name = "check_weather";
        let weather_tool = Tool::new()
            .function_name(weather_tool_name)
            .function_description("Get the weather in farhenheight degrees for any given location")
            .add_function_property("location", Property::new_string("location for where you want to check the weather. Use the format 'city, state' if the location is in the United States. Otherwise use 'city, country'"))
            .add_required_property("location")
            .build();

        chat.add_tool(weather_tool);
        chat.add_message(message);

        let received_message = chat.send()?;

        chat.add_message(received_message);
        chat.add_message(Message::new_tool("sunny, 55 degrees"));

        let weather_description = chat.send()?;

        assert_eq!(weather_description.content, "Based on the output of the tool call, it appears that the current weather in New York is sunny with a temperature of 55 degrees.");

        Ok(())
    }

    #[test]
    fn should_save_chats_into_history() -> Result<()> {
        let mut chat = Chat::new(
            "llama3.2:1b-instruct-fp16",
            Some(ChatRequestOptions::new().save_messages().seed(123)),
        );

        chat.add_message(Message::new_user("Hello"));
        let response = chat.send()?;

        assert_eq!(chat.messages.len(), 2);
        assert_eq!(response, *chat.messages.last().unwrap());

        Ok(())
    }

    #[test]
    fn should_not_save_chats_into_history() -> Result<()> {
        let mut chat = Chat::new(
            "llama3.2:1b-instruct-fp16",
            Some(ChatRequestOptions::new().seed(123)),
        );

        let hello = Message::new_user("Hello");
        chat.add_message(hello.clone());
        chat.send()?;

        assert_eq!(chat.messages.len(), 1);

        Ok(())
    }
}
