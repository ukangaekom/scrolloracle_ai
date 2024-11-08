
use genai::chat::printer::print_chat_stream;
use genai::chat::{ChatMessage, ChatRequest, MessageContent};
use genai::Client;
// use genai::chat::printer::print_chat_stream;



#[tokio::main]

pub async fn ai_agent(_text:&str) -> Option<std::string::String>  {

    let client = Client::default();

    let chat_req: ChatRequest = ChatRequest::new(vec![
        ChatMessage::system("In Context, you are considered to be an advanced crypto wizard for a blockchain called Scroll Blockchain. You are to leverage your large language model and internet searching capabailities to do the following:
        1. Search for current ongoing events based on Scroll Blockchain Ecosystem.
        2. Search all blogs and websites related to Scroll Blockchain and get data from there.
        3. Get data for all tokens launched on Scroll Blockchain.
        4. Get any reachable valid and upto data data about projects based on Scroll blockchain.
        
        Note: These are your sources of informations. While you generate answers, your answers must correlate to this sources. No Hallucination.
        
        Following this data sources and Information, I name you ```AptosOracle```. You are the wizard of Aptos Blockchain Universe. Use this instructions to engage users more than any paid expert can do."),
        ChatMessage::user(_text.to_string())
    ]);

    let model: &str = "gemini-1.5-flash-latest";

    let chat_res = client.exec_chat_stream(model, chat_req, None).await.ok();
    
    let reply = print_chat_stream(chat_res.expect("REASON"),  None).await.ok();

    return reply;



         

    
    }

   

