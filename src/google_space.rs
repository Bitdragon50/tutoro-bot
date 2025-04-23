use std::sync::Arc;
use derive_more::*;
use google_cloud_auth::{credentials::service_account, *};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let scopes = ["https://www.googleapis.com/auth/chat.bot"];
    let service_account_cred = google_cloud_auth::credentials::service_account::Builder::with_scopes(self, scopes).build();
    let token = service_account_cred.unwrap().token().await.unwrap();
}
#[derive(Debug,Error)]
struct GoogleSpacesError {}

fn start_app(){}

fn get_tutoros_messaseges_from_a_space(  ) -> Result<GoogleSpacesMessages, GoogleSpacesError> {}

fn get_tutoros_messaseges(  ) -> Result<GoogleSpacesMessages, GoogleSpacesError> {
    let _ = list_of_components;
    let googl_chat_auth = "https://developers.google.com/workspace/chat/authenticate-authorize-chat-app#python)";
    let auth_google = google_chat_auth();    
    let spaces = fetch_spaces("https://developers.google.com/workspace/chat/api/reference/rest");
    let google_messages: Vec<GoogleMessage> = fetch_messages();
    match spaces {
        Ok(vec_of_spaces) => todo!(),
        Err(google_spaces_error) => todo!()
    }
    let turoros_messages = fetch_spaces_messages("https://developers.google.com/workspace/chat/api/reference/rest");
}

fn fetch_messages() {

}

fn fetch_turoros_messages(end_point: &str) -> Result<Vec<GoogleSpace> , GoogleSpacesError>{
 
}

fn fetch_spaces(end_point: &str) -> Result<Vec<GoogleSpace> , GoogleSpacesError>{
 
}
enum GoogleSpacesMessages {
    GoogleSpace,
    GoogleMessages
}

struct GoogleSpaceMessage {
    google_space: Arc<GoogleSpace>,
    google_message: GoogleMessage
}

struct GoogleMessage {
    body: String
}
struct GoogleSpace {
    google_messages: Vec<GoogleMessage>
}

fn google_chat_auth() {
   let python_script = r#"
        from apiclient.discovery import build
        from google.oauth2 import service_account
        
        # Specify required scopes.
        SCOPES = ['https://www.googleapis.com/auth/chat.bot']
        
        # Specify service account details.
        creds = service_account.Credentials.from_service_account_file(
            'credentials.json', scopes=SCOPES)
        
        # Build the URI and authenticate with the service account.
        chat = build('chat', 'v1', credentials=creds)
        
        # Create a Chat message.
        result = chat.spaces().messages().create(
        
            # The space to create the message in.
            #
            # Replace SPACE_NAME with a space name.
            # Obtain the space name from the spaces resource of Chat API,
            # or from a space's URL.
            parent='spaces/SPACE_NAME',
        
            # The message to create.
            body={'text': 'Hello, world!'}
        
        ).execute()
        
        # Prints details about the created message.
        print(result)
    "#;



}