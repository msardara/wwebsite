use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{Photo, PhotoInput};
use gloo_file::futures::read_as_bytes;
use gloo_file::File as GlooFile;
use leptos::html::Input;
use leptos::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement};

#[component]
pub fn PhotoManagement() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (photos, set_photos) = create_signal::<Vec<Photo>>(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);

    // Load photos on mount
    create_effect({
        move |_| {
            spawn_local(async move {
                let admin_ctx = admin_context;
                set_loading.set(true);
                set_error.set(None);

                match admin_ctx.authenticated_client().get_all_photos().await {
                    Ok(photo_list) => {
                        set_photos.set(photo_list);
                        set_loading.set(false);
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load photos: {}", e)));
                        set_loading.set(false);
                    }
                }
            });
        }
    });

    // Refresh handler
    let refresh_photos = {
        move |_: web_sys::MouseEvent| {
            spawn_local(async move {
                let admin_ctx = admin_context;
                set_loading.set(true);
                set_error.set(None);

                match admin_ctx.authenticated_client().get_all_photos().await {
                    Ok(photo_list) => {
                        set_photos.set(photo_list);
                        set_loading.set(false);
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load photos: {}", e)));
                        set_loading.set(false);
                    }
                }
            });
        }
    };

    // Store refresh function for child components
    let refresh_fn = store_value(refresh_photos);

    view! {
        <div class=ADMIN_CONTAINER>
            <div class=PAGE_HEADER_CONTAINER>
                <h2 class=PAGE_HEADER>"Photo Gallery Management"</h2>
                <button on:click=refresh_photos class=REFRESH_BUTTON>
                    "↻ Refresh"
                </button>
            </div>

            {move || error.get().map(|err| view! { <div class=ALERT_ERROR>{err}</div> })}

            <PhotoUploadForm on_upload_complete=move || {
                let ev = web_sys::MouseEvent::new("click").unwrap();
                refresh_fn.with_value(|f| f(ev));
            } />

            <Show
                when=move || loading.get()
                fallback={
                    move || {
                        let admin_ctx = admin_context;
                        let photo_list = photos.get();
                        if photo_list.is_empty() {
                            view! {
                                <div class=EMPTY_STATE>
                                    <div class=EMPTY_STATE_ICON>"📷"</div>
                                    <h3 class=EMPTY_STATE_TITLE>"No photos yet"</h3>
                                    <p class=EMPTY_STATE_MESSAGE>"Add photos to the Supabase storage bucket to display them here"</p>
                                </div>
                            }
                        } else {
                            let base_url = admin_ctx.authenticated_client().base_url.clone();
                            view! {
                                <div class=CARD>
                                    <h3 class=SECTION_TITLE>{format!("Gallery ({} photos)", photo_list.len())}</h3>
                                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                                        {photo_list.into_iter().map(|photo| {
                                            let photo_url = format!("{}/storage/v1/object/public/wedding-photos/{}", base_url, photo.filename);
                                            view! { <PhotoCard photo=photo photo_url=photo_url on_refresh=move || {
                                                let ev = web_sys::MouseEvent::new("click").unwrap();
                                                refresh_fn.with_value(|f| f(ev));
                                            }/> }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }
                        }
                    }
                }
            >
                <div class=LOADING_CONTAINER>
                    <div class=LOADING_SPINNER></div>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn PhotoCard(
    photo: Photo,
    photo_url: String,
    on_refresh: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");
    let (deleting, set_deleting) = create_signal(false);

    let display_caption = photo
        .caption
        .clone()
        .unwrap_or_else(|| "No caption".to_string());
    let display_order = photo.display_order;
    let created_at = photo
        .created_at
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let photo_id = store_value(photo.id.clone());
    let filename = store_value(photo.filename.clone());

    let handle_delete = move |_| {
        set_deleting.set(true);

        spawn_local(async move {
            // Delete from storage first - use authenticated client
            let _ = admin_context
                .authenticated_client()
                .delete_photo_from_storage(&filename.get_value())
                .await;

            // Then delete from database
            match admin_context
                .authenticated_client()
                .delete_photo(&photo_id.get_value())
                .await
            {
                Ok(_) => {
                    on_refresh();
                }
                Err(_) => {
                    set_deleting.set(false);
                }
            }
        });
    };

    view! {
        <div class="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-xl transition-shadow duration-200">
            <div class="aspect-square bg-gray-100">
                <img
                    src=photo_url
                    alt=photo.filename.clone()
                    class="w-full h-full object-cover"
                />
            </div>
            <div class="p-3 space-y-2">
                <p class="text-sm font-medium text-gray-900">{display_caption}</p>
                <div class="flex justify-between text-xs text-gray-500">
                    <span>"Order: " {display_order}</span>
                    <span>{created_at}</span>
                </div>
                <button
                    on:click=handle_delete
                    disabled=move || deleting.get()
                    class="w-full bg-red-500 hover:bg-red-600 text-white font-semibold py-2 px-3 rounded-lg shadow-md hover:shadow-lg transition-all duration-200 disabled:opacity-50 text-sm"
                >
                    {move || if deleting.get() { "Deleting..." } else { "🗑️ Delete" }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn PhotoUploadForm<F>(on_upload_complete: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (selected_file, set_selected_file) = create_signal::<Option<(String, Vec<u8>)>>(None);
    let (preview_url, set_preview_url) = create_signal::<Option<String>>(None);
    let (caption, set_caption) = create_signal(String::new());
    let (uploading, set_uploading) = create_signal(false);
    let (upload_success, set_upload_success) = create_signal::<Option<String>>(None);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (file_size, set_file_size) = create_signal::<Option<usize>>(None);

    let file_input_ref = create_node_ref::<Input>();

    const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

    // Store closures in StoredValue to make them Copy
    let on_file_change_fn = store_value(move |ev: Event| {
        let input = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let gloo_file = GlooFile::from(file.clone());
                    let file_name = gloo_file.name();
                    let size = gloo_file.size() as usize;

                    // Check file size
                    if size > MAX_FILE_SIZE {
                        set_error.set(Some(format!(
                            "File too large: {:.2}MB. Maximum size is 50MB.",
                            size as f64 / 1024.0 / 1024.0
                        )));
                        set_selected_file.set(None);
                        set_preview_url.set(None);
                        set_file_size.set(None);
                        return;
                    }

                    set_file_size.set(Some(size));

                    // Create preview URL
                    if let Ok(url) = web_sys::Url::create_object_url_with_blob(gloo_file.as_ref()) {
                        set_preview_url.set(Some(url));
                    }

                    // Read file as bytes
                    spawn_local(async move {
                        match read_as_bytes(&gloo_file).await {
                            Ok(bytes) => {
                                set_selected_file.set(Some((file_name, bytes)));
                            }
                            Err(_) => {
                                set_error.set(Some("Failed to read file".to_string()));
                            }
                        }
                    });
                }
            }
        }
    });

    let handle_upload_fn = store_value({
        move |_| {
            if let Some((filename, file_data)) = selected_file.get() {
                let caption_value = caption.get();

                set_uploading.set(true);
                set_upload_success.set(None);
                set_error.set(None);

                spawn_local(async move {
                    // Upload to storage - use authenticated client
                    match admin_context
                        .authenticated_client()
                        .upload_photo_to_storage(&filename, file_data)
                        .await
                    {
                        Ok(_) => {
                            // Create database entry
                            let photo_input = PhotoInput {
                                filename: filename.clone(),
                                caption: if caption_value.is_empty() {
                                    None
                                } else {
                                    Some(caption_value)
                                },
                                display_order: 0,
                            };

                            match admin_context
                                .authenticated_client()
                                .create_photo(&photo_input)
                                .await
                            {
                                Ok(_) => {
                                    set_upload_success
                                        .set(Some("Photo uploaded successfully!".to_string()));
                                    set_uploading.set(false);
                                    set_selected_file.set(None);
                                    set_preview_url.set(None);
                                    set_caption.set(String::new());

                                    // Clear file input
                                    if let Some(input) = file_input_ref.get() {
                                        input.set_value("");
                                    }

                                    // Notify parent
                                    on_upload_complete();
                                }
                                Err(e) => {
                                    set_error
                                        .set(Some(format!("Failed to create photo entry: {}", e)));
                                    set_uploading.set(false);
                                }
                            }
                        }
                        Err(e) => {
                            set_error.set(Some(format!("Failed to upload photo: {}", e)));
                            set_uploading.set(false);
                        }
                    }
                });
            }
        }
    });

    let clear_selection_fn = store_value(move |_| {
        set_selected_file.set(None);
        set_preview_url.set(None);
        set_caption.set(String::new());
        set_upload_success.set(None);
        set_file_size.set(None);
        if let Some(input) = file_input_ref.get() {
            input.set_value("");
        }
    });

    view! {
        <div>
            {move || error.get().map(|err| view! { <div class=ALERT_ERROR>{err}</div> })}
            {move || upload_success.get().map(|msg| view! { <div class=ALERT_SUCCESS>{msg}</div> })}

            <div class=CARD>
                <h3 class=SECTION_TITLE>"Upload Photo"</h3>
                <div class="space-y-4">
                    <div>
                        <label class=FORM_LABEL>"Select Photo"</label>
                        <input
                            type="file"
                            accept="image/*"
                            on:change=move |ev| on_file_change_fn.with_value(|f| f(ev))
                            node_ref=file_input_ref
                            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-pink-500 focus:border-transparent"
                        />
                        <p class="text-sm text-gray-500 mt-1">
                            "Supported formats: JPEG, PNG, GIF, WebP. Max size: 50MB"
                        </p>
                    </div>

                    <Show when=move || selected_file.get().is_some()>
                        <div class="space-y-4">
                            {move || preview_url.get().map(|url| view! {
                                <div>
                                    <div class="flex items-center justify-between mb-2">
                                        <label class=FORM_LABEL>"Preview"</label>
                                        {move || file_size.get().map(|size| view! {
                                            <span class="text-sm text-gray-500">
                                                {format!("{:.2}MB", size as f64 / 1024.0 / 1024.0)}
                                            </span>
                                        })}
                                    </div>
                                    <div class="w-full max-w-md mx-auto">
                                        <img src=url class="w-full h-auto rounded-lg shadow-md" alt="Preview" />
                                    </div>
                                </div>
                            })}

                            <div>
                                <label class=FORM_LABEL>"Caption (optional)"</label>
                                <input
                                    type="text"
                                    prop:value=move || caption.get()
                                    on:input=move |ev| set_caption.set(event_target_value(&ev))
                                    placeholder="Enter a caption for this photo"
                                    class=FORM_INPUT
                                />
                            </div>

                            <div class="flex gap-2">
                                <button
                                    on:click=move |ev| handle_upload_fn.with_value(|f| f(ev))
                                    disabled=move || uploading.get()
                                    class=BUTTON_PRIMARY
                                >
                                    {move || if uploading.get() { "Uploading..." } else { "📤 Upload Photo" }}
                                </button>

                                <button
                                    on:click=move |ev| clear_selection_fn.with_value(|f| f(ev))
                                    disabled=move || uploading.get()
                                    class="flex-1 bg-gray-500 hover:bg-gray-600 text-white font-semibold py-3 px-6 rounded-full shadow-md hover:shadow-lg transition-all duration-200 hover:scale-105 disabled:opacity-50 disabled:hover:scale-100"
                                >
                                    "✖ Cancel"
                                </button>
                            </div>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}
