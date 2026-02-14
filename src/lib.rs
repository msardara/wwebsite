use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod constants;
mod contexts;
mod i18n;
mod pages;
pub mod styles;
mod supabase;
mod types;

use constants::LANGUAGE_KEY;

use components::layout::Layout;
use contexts::{GuestContext, SupabaseContext};
use pages::admin::AdminPage;
use pages::events::EventsPage;
use pages::gallery::GalleryPage;
use pages::home::HomePage;
use pages::invitation::InvitationPage;
use pages::rsvp::RsvpPage;
use supabase::SupabaseError;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize Supabase context (singleton clients)
    let supabase_context = SupabaseContext::new();
    provide_context(supabase_context);

    // Initialize language context at app level
    let (language, set_language) = create_signal(types::Language::from_browser());

    // Initialize guest context (needs set_language to auto-set guest's default language)
    let guest_context = GuestContext::new(set_language);
    provide_context(guest_context);

    // Try to load saved language preference
    use gloo_storage::{LocalStorage, Storage};
    create_effect(move |_| {
        if let Ok(saved_lang) = LocalStorage::get::<String>(LANGUAGE_KEY) {
            set_language.set(types::Language::from_code(&saved_lang));
        }
    });

    // Provide language context to all components
    provide_context(language);
    provide_context(set_language);

    view! {
        <Stylesheet id="leptos" href="/pkg/wedding-website.css"/>
        <Link rel="icon" type_="image/x-icon" href="/favicon.ico"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Wedding Website - Celebrating our special day"/>

        <Router>
            <Routes>
                // Invitation entry page (no auth required)
                <Route path="/invitation" view=InvitationPage/>

                // Admin panel (independent authentication)
                <Route path="/admin" view=AdminPage/>

                // Protected routes - all require guest authentication
                <Route path="/" view=Layout>
                    <Route path="" view=HomePage/>
                    <Route path="events" view=EventsPage/>
                    <Route path="gallery" view=GalleryPage/>
                    <Route path="rsvp" view=RsvpPage/>
                </Route>

                // Catch-all route for unknown paths - redirect to home
                <Route path="/*any" view=|| view! {
                    {
                        let navigate = leptos_router::use_navigate();
                        create_effect(move |_| {
                            navigate("/", Default::default());
                        });
                        view! {}
                    }
                }/>
            </Routes>
        </Router>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}
