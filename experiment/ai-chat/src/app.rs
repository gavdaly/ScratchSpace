use crate::{
    conversation::{Conversation, Message},
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_server_action::<SendMessage>();
    //     move || {},
    //     move |text: &String| {
    //         let user_message = Message { text, user: true };
    //         set_conversation(move |c| {
    //             c.message.push(user_message);
    //         });
    //     },
    // );
    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ai-chat.css"/>

        // sets the document title
        <Title text="AI Chat"/>

        <main>

        </main>
        <ActionForm action=send>
            <textarea />
            <button type="submit">"submit"</button>
        </ActionForm>
    }
}

#[server]
async fn send_message(text: String) -> Result<Message, ServerFnError> {
    Ok(Message {
        user: true,
        text: "response".into(),
    })
}
