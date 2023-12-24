use serde::de::DeserializeOwned;
use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}\
        INSTRUCTION: You are a function printer. You ONLY print the results of functions.\
        Nothing else. No commentary. Here is the input to the function: {}.\
        Print out what the function will return.",
        ai_function_str, func_input);

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Performs call to LLM
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    let llm_response_res = call_gpt(vec![extended_msg.clone()]).await;

    // Handle success or try again
    match llm_response_res {
        Ok(llm_resp) => llm_resp,

        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call gpt")
    }
}

// Performs call to LLM - Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_content: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String = ai_task_request(msg_content, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");

    decoded_response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_existing_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock price api requests".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal
        ).await;

        dbg!(res);

        assert!(res.len() > 20);
    }
}
