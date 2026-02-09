use crate::contexts::{use_supabase_rpc, GuestContext};
use crate::i18n::Translations;
use crate::types::{Language, Photo};
use leptos::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn GalleryPage() -> impl IntoView {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let _guest_context = use_context::<GuestContext>().expect("GuestContext not found");

    let translations = move || Translations::new(language.get());

    let (photos, set_photos) = create_signal::<Vec<Photo>>(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (selected_photo, set_selected_photo) = create_signal::<Option<usize>>(None);

    // Load photos from Supabase
    create_effect(move |_| {
        let client = use_supabase_rpc();
        spawn_local(async move {
            set_loading.set(true);
            match client.get_all_photos().await {
                Ok(photos_list) => {
                    set_photos.set(photos_list);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load photos: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    let open_lightbox = move |index: usize| {
        set_selected_photo.set(Some(index));
    };

    let close_lightbox = move |_| {
        set_selected_photo.set(None);
    };

    let next_photo = move |_| {
        set_selected_photo.update(|current| {
            if let Some(index) = current {
                let photos_len = photos.get().len();
                *current = Some((*index + 1) % photos_len);
            }
        });
    };

    let prev_photo = move |_| {
        set_selected_photo.update(|current| {
            if let Some(index) = current {
                let photos_len = photos.get().len();
                *current = Some(if *index == 0 {
                    photos_len - 1
                } else {
                    *index - 1
                });
            }
        });
    };

    view! {
        <div class="max-w-6xl mx-auto px-4">
            <div class="text-center mb-12 animate-fade-in">
                <h1 class="text-4xl md:text-5xl font-serif font-bold text-primary-600 mb-4">
                    {move || translations().t("gallery.title")}
                </h1>
                <p class="text-lg text-gray-600">
                    "Moments from our journey together"
                </p>
            </div>

            {move || error.get().map(|err| view! {
                <div class="mb-6 bg-red-50 border-l-4 border-red-500 text-red-800 px-4 py-3 rounded">
                    <p>{err}</p>
                </div>
            })}

            <Show
                when=move || !loading.get()
                fallback=move || view! {
                    <div class="flex items-center justify-center py-12">
                        <div class="text-center">
                            <div class="animate-spin text-6xl mb-4">"⏳"</div>
                            <p class="text-gray-600">"Loading photos..."</p>
                        </div>
                    </div>
                }
            >
                <Show
                    when=move || !photos.get().is_empty()
                    fallback=move || view! {
                        <div class="text-center py-12 bg-white rounded-lg shadow-md animate-fade-in">
                            <div class="text-6xl mb-4">"📷"</div>
                            <p class="text-xl text-gray-600 mb-2">
                                {move || translations().t("gallery.empty")}
                            </p>
                            <p class="text-sm text-gray-500">
                                "No photos have been uploaded yet"
                            </p>
                        </div>
                    }
                >
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                        <For
                            each=move || photos.get()
                            key=|photo| photo.id.clone()
                            children=move |photo| {
                                let photo_clone = photo.clone();
                                let index = photos.get().iter().position(|p| p.id == photo.id).unwrap_or(0);
                                let client = use_supabase_rpc();
                                let photo_url = format!(
                                    "{}/storage/v1/object/public/wedding-photos/{}",
                                    client.base_url,
                                    photo.filename
                                );

                                view! {
                                    <div
                                        class="relative group cursor-pointer overflow-hidden rounded-lg shadow-md hover:shadow-xl transition-all duration-300 animate-fade-in"
                                        on:click=move |_| open_lightbox(index)
                                    >
                                        <div class="aspect-square bg-gray-100">
                                            <img
                                                src=photo_url.clone()
                                                alt=photo_clone.caption.clone().unwrap_or_else(|| "Wedding photo".to_string())
                                                class="w-full h-full object-cover"
                                                loading="lazy"
                                            />
                                        </div>
                                        <div class="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-40 transition-all duration-300 flex items-center justify-center">
                                            {photo_clone.caption.clone().map(|caption| view! {
                                                <p class="text-white opacity-0 group-hover:opacity-100 transition-opacity duration-300 px-4 text-center font-semibold">
                                                    {caption}
                                                </p>
                                            })}
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>
            </Show>

            <Show when=move || selected_photo.get().is_some()>
                {move || {
                    let current = selected_photo.get().unwrap_or(0);
                    let photos_list = photos.get();
                    if let Some(photo) = photos_list.get(current) {
                        let client = use_supabase_rpc();
                        let photo_url = format!(
                            "{}/storage/v1/object/public/wedding-photos/{}",
                            client.base_url,
                            photo.filename
                        );
                        view! {
                            <Lightbox
                                photo_url=photo_url
                                caption=photo.caption.clone()
                                current_index=current
                                total_photos=photos_list.len()
                                on_close=close_lightbox
                                on_next=next_photo
                                on_prev=prev_photo
                            />
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
            </Show>
        </div>
    }
}

#[component]
fn Lightbox(
    photo_url: String,
    caption: Option<String>,
    current_index: usize,
    total_photos: usize,
    on_close: impl Fn(web_sys::MouseEvent) + 'static + Copy,
    on_next: impl Fn(web_sys::MouseEvent) + 'static + Copy,
    on_prev: impl Fn(web_sys::MouseEvent) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div
            class="fixed inset-0 bg-black bg-opacity-90 z-50 flex items-center justify-center p-4 animate-fade-in"
            on:click=on_close
        >
            <button
                class="absolute top-4 right-4 text-white text-4xl hover:text-gray-300 transition-colors z-10"
                on:click=on_close
            >
                "×"
            </button>

            {if total_photos > 1 {
                view! {
                    <>
                        <button
                            class="absolute left-4 top-1/2 -translate-y-1/2 text-white text-4xl hover:text-gray-300 transition-colors z-10 bg-black bg-opacity-50 rounded-full w-12 h-12 flex items-center justify-center"
                            on:click=move |e| {
                                e.stop_propagation();
                                on_prev(e);
                            }
                        >
                            "‹"
                        </button>

                        <button
                            class="absolute right-4 top-1/2 -translate-y-1/2 text-white text-4xl hover:text-gray-300 transition-colors z-10 bg-black bg-opacity-50 rounded-full w-12 h-12 flex items-center justify-center"
                            on:click=move |e| {
                                e.stop_propagation();
                                on_next(e);
                            }
                        >
                            "›"
                        </button>
                    </>
                }.into_view()
            } else {
                ().into_view()
            }}

            <div
                class="max-w-5xl max-h-full bg-white rounded-lg overflow-hidden"
                on:click=|e| e.stop_propagation()
            >
                <div class="relative">
                    <img
                        src=photo_url
                        alt=caption.clone().unwrap_or_else(|| "Wedding photo".to_string())
                        class="max-w-full max-h-[70vh] w-auto h-auto mx-auto"
                    />
                </div>
                <div class="p-6 bg-white">
                    {caption.map(|cap| view! {
                        <p class="text-gray-800 text-center text-lg mb-2">
                            {cap}
                        </p>
                    })}
                    <p class="text-gray-500 text-center text-sm">
                        {format!("{} / {}", current_index + 1, total_photos)}
                    </p>
                </div>
            </div>
        </div>
    }
}
