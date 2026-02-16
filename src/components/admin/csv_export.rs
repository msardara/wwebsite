//! CSV export utilities for admin dashboard.
//!
//! Provides functions to convert guest groups and guests into CSV strings
//! and trigger browser file downloads via Blob URLs.

use crate::types::{Guest, GuestGroup};
use wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// CSV helpers
// ---------------------------------------------------------------------------

/// Escape a value for CSV: wrap in double-quotes if it contains a comma,
/// double-quote, or newline. Inner double-quotes are doubled.
fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// Build a CSV string for guest groups.
pub fn guest_groups_to_csv(groups: &[GuestGroup]) -> String {
    let mut out = String::from(
        "id,name,email,invitation_code,party_size,locations,default_language,additional_notes\n",
    );

    for g in groups {
        let email = g.email.as_deref().unwrap_or("");
        let locations = g.locations.join("; ");
        let notes = g.additional_notes.as_deref().unwrap_or("");

        out.push_str(&csv_escape(&g.id));
        out.push(',');
        out.push_str(&csv_escape(&g.name));
        out.push(',');
        out.push_str(&csv_escape(email));
        out.push(',');
        out.push_str(&csv_escape(&g.invitation_code));
        out.push(',');
        out.push_str(&g.party_size.to_string());
        out.push(',');
        out.push_str(&csv_escape(&locations));
        out.push(',');
        out.push_str(&csv_escape(&g.default_language));
        out.push(',');
        out.push_str(&csv_escape(notes));
        out.push('\n');
    }

    out
}

/// Build a CSV string for guests.
///
/// `group_lookup` maps `guest_group_id` → group name so the export is
/// human-readable without having to cross-reference another file.
pub fn guests_to_csv(
    guests: &[Guest],
    group_lookup: &std::collections::HashMap<String, String>,
) -> String {
    let mut out = String::from(
        "id,guest_group_id,guest_group_name,name,attending_locations,age_category,\
         vegetarian,vegan,halal,no_pork,gluten_free,other_dietary\n",
    );

    for g in guests {
        let group_name = group_lookup
            .get(&g.guest_group_id)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let locations = g.attending_locations.join("; ");

        out.push_str(&csv_escape(&g.id));
        out.push(',');
        out.push_str(&csv_escape(&g.guest_group_id));
        out.push(',');
        out.push_str(&csv_escape(group_name));
        out.push(',');
        out.push_str(&csv_escape(&g.name));
        out.push(',');
        out.push_str(&csv_escape(&locations));
        out.push(',');
        out.push_str(&csv_escape(g.age_category.display_name()));
        out.push(',');
        out.push_str(if g.dietary_preferences.vegetarian {
            "true"
        } else {
            "false"
        });
        out.push(',');
        out.push_str(if g.dietary_preferences.vegan {
            "true"
        } else {
            "false"
        });
        out.push(',');
        out.push_str(if g.dietary_preferences.halal {
            "true"
        } else {
            "false"
        });
        out.push(',');
        out.push_str(if g.dietary_preferences.no_pork {
            "true"
        } else {
            "false"
        });
        out.push(',');
        out.push_str(if g.dietary_preferences.gluten_free {
            "true"
        } else {
            "false"
        });
        out.push(',');
        out.push_str(&csv_escape(&g.dietary_preferences.other));
        out.push('\n');
    }

    out
}

// ---------------------------------------------------------------------------
// Browser download trigger
// ---------------------------------------------------------------------------

/// Trigger a browser file download from an in-memory CSV string.
///
/// Creates a temporary Blob URL, appends an invisible `<a>` element with a
/// `download` attribute, clicks it, and cleans up.
pub fn trigger_csv_download(csv_content: &str, filename: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or_else(|| "No global `window` found".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "No `document` on window".to_string())?;

    // Add UTF-8 BOM so Excel opens the file with the correct encoding
    let bom = "\u{FEFF}";
    let full_content = format!("{}{}", bom, csv_content);

    // Create a Blob from the CSV text
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&wasm_bindgen::JsValue::from_str(&full_content));

    let opts = web_sys::BlobPropertyBag::new();
    opts.set_type("text/csv;charset=utf-8");

    let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts)
        .map_err(|e| format!("Failed to create Blob: {:?}", e))?;

    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|e| format!("Failed to create object URL: {:?}", e))?;

    // Create a temporary <a> element and click it
    let anchor: web_sys::HtmlAnchorElement = document
        .create_element("a")
        .map_err(|e| format!("Failed to create <a>: {:?}", e))?
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .map_err(|_| "Created element is not an HtmlAnchorElement".to_string())?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor
        .style()
        .set_property("display", "none")
        .map_err(|e| format!("Failed to hide anchor: {:?}", e))?;

    let body = document
        .body()
        .ok_or_else(|| "No <body> in document".to_string())?;
    body.append_child(&anchor)
        .map_err(|e| format!("Failed to append anchor: {:?}", e))?;

    anchor.click();

    // Clean up
    body.remove_child(&anchor)
        .map_err(|e| format!("Failed to remove anchor: {:?}", e))?;
    web_sys::Url::revoke_object_url(&url)
        .map_err(|e| format!("Failed to revoke object URL: {:?}", e))?;

    Ok(())
}
