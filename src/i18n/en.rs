//! English translations.

use std::collections::HashMap;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Navigation
    map.insert("nav.home", "Home");
    map.insert("nav.events", "Program");
    map.insert("nav.rsvp", "RSVP");
    map.insert("nav.admin", "Admin");

    // Home page
    map.insert("home.title", "We're Getting Married!");
    map.insert("home.subtitle", "ARE GETTING MARRIED");
    map.insert("home.welcome", "Welcome");
    map.insert(
        "home.intro_p1",
        "We are so excited to celebrate our special day with you!",
    );
    map.insert("home.intro_p2", "Please explore our website to find all the information you need about the events, venues, and how to RSVP. We can't wait to see you there!");
    map.insert("home.sardinia_title", "Sardinia");
    map.insert(
        "home.sardinia_desc",
        "Join us to celebrate our wedding in Sardinia!",
    );
    map.insert("home.tunisia_title", "Tunisia");
    map.insert(
        "home.tunisia_desc",
        "Celebrate with us in the heart of North Africa with Tunisian hospitality",
    );
    map.insert("home.gift_message", "Let's Celebrate Together");
    map.insert(
        "home.contribution_text",
        "Our joy wouldn't be complete without you. We'd be honored to have you celebrate this moment with us.",
    );
    map.insert(
        "home.rsvp_instruction",
        "Please let us know if you can join by filling out the RSVP.",
    );
    map.insert("home.our_love", "Once Upon a Time...");
    map.insert("home.for_gardens", "");
    map.insert("home.and_each", "at the Cité Universitaire");
    map.insert("home.other", "de Paris");
    map.insert(
        "home.couple_story",
        "The year was 2017. We had both just arrived in Paris; Mouna was finishing her studies and Mauro was starting his PhD. It all began with a dinner, then a friendship, a connection. What was meant to be a simple meeting marked the beginning of our story.",
    );
    map.insert("home.see_you_there", "See you there!");

    // Events page
    map.insert("events.subtitle_single", "Join us in celebrating our love");
    map.insert(
        "events.subtitle_multiple",
        "Join us in celebrating our love across beautiful destinations",
    );
    map.insert("events.title", "Event Details");
    map.insert("events.sardinia", "Sardinia, Italy");
    map.insert("events.tunisia", "Tunisia");
    map.insert("events.nice", "Nice, France");
    map.insert("events.schedule", "Schedule");
    map.insert("events.venue", "Venue");
    map.insert("events.accommodation", "Accommodation");
    map.insert("events.travel", "Travel Information");
    map.insert("events.view_on_maps", "View on Google Maps");

    // Event content placeholders
    map.insert("events.date_sardinia", "September 19, 2026");
    map.insert("events.sort_date_sardinia", "2026-09-19");
    map.insert("events.schedule_sardinia", "Ceremony at 6:00 PM");
    map.insert("events.venue_sardinia_name", "Sa Mola Hotel Ristorante");
    map.insert(
        "events.venue_sardinia_link",
        "https://maps.app.goo.gl/yNLukc3C9V6bPL4DA",
    );
    map.insert(
        "events.accommodation_sardinia",
        "• We recommend checking hotels in <a href='https://maps.app.goo.gl/N4KVpYEZF7G4jWbC6' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Oristano</a> / <a href='https://maps.app.goo.gl/x72Q9zfYCDEZWMem9' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cabras</a>",
    );
    map.insert(
        "events.travel_sardinia",
        "• Closest airport: <a href='https://maps.app.goo.gl/uvofAX2NkqLeoi2D7' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cagliari</a><br/>• We recommend renting a car",
    );
    map.insert("events.date_tunisia", "June 27, 2026");
    map.insert("events.sort_date_tunisia", "2026-06-27");
    map.insert("events.schedule_tunisia", "Starts at 9:00 PM");
    map.insert("events.venue_tunisia_name", "Espace La Vallée, Monastir");
    map.insert(
        "events.venue_tunisia_link",
        "https://maps.app.goo.gl/Y4dCfdekMGiWvMFX6",
    );
    map.insert("events.accommodation_tunisia", "• Hotels in Monastir<br/>• We recommend checking the <a href='https://www.google.com/maps/d/edit?mid=1saWGZmjkgOkyQZxfyFeMldJy3JWWvg8&usp=sharing' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>tourist area of Monastir</a>");
    map.insert(
        "events.travel_tunisia",
        "• Closest airport: <a href='https://maps.app.goo.gl/YyvPgoUmRDPqmzgy8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Monastir</a><br/>• We recommend avoiding Tunisair airlines<br/>• Tap water is not drinkable in Tunisia",
    );
    map.insert("events.date_nice", "April 8, 2026");
    map.insert("events.sort_date_nice", "2026-04-08");
    map.insert(
        "events.schedule_nice",
        "Ceremony at 11:00 AM, Lunch to follow",
    );
    map.insert("events.venue_nice_name", "Nice City Hall");
    map.insert(
        "events.venue_nice_link",
        "https://maps.app.goo.gl/D9hQbstQqHWxa1m49",
    );
    map.insert("events.accommodation_nice", "• Hotels in Nice<br/>• Prefer accommodation along tram lines 2 and 3, preferably near the beach");
    map.insert(
        "events.travel_nice",
        "• Closest airport: <a href='https://maps.app.goo.gl/8KRRidQakgL2C97t8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Nice Côte d'Azur</a>",
    );

    // Location names (short, for RSVP checkboxes etc.)
    map.insert("location.sardinia", "Sardinia");
    map.insert("location.tunisia", "Tunisia");
    map.insert("location.nice", "Nice");

    // RSVP
    map.insert("rsvp.title", "RSVP");
    map.insert("rsvp.subtitle", "Let us know if you can join us!");
    map.insert("rsvp.lookup", "Find Your Invitation");
    map.insert("rsvp.name", "Your Name");
    map.insert("rsvp.code", "Invitation Code");
    map.insert("rsvp.code_placeholder", "ABC123");
    map.insert(
        "rsvp.code_help",
        "Enter the invitation code from your invitation card",
    );
    map.insert("rsvp.email", "Email");
    map.insert("rsvp.find", "Find RSVP");
    map.insert("rsvp.attending", "Will you be attending?");
    map.insert("rsvp.yes", "Yes");
    map.insert("rsvp.no", "No");
    map.insert("rsvp.guests", "Number of Guests");
    map.insert("rsvp.dietary", "Dietary Restrictions");
    map.insert("rsvp.guest_list_title", "Guest List & Dietary Preferences");
    map.insert("rsvp.guest_list_description", "Manage your guest list and dietary restrictions here. You can then select which guests attend each location below.");
    map.insert("rsvp.vegetarian", "Vegetarian");
    map.insert("rsvp.vegan", "Vegan");
    map.insert("rsvp.halal", "Halal");
    map.insert("rsvp.no_pork", "No Pork");
    map.insert("rsvp.other", "Other (Allergies, etc.)");
    map.insert(
        "rsvp.other_dietary",
        "Other dietary restrictions (specify number and type)",
    );
    map.insert("rsvp.dietary_number", "Guests");
    map.insert(
        "rsvp.dietary_placeholder",
        "e.g., Gluten-free, No shellfish, Lactose intolerant",
    );
    map.insert(
        "rsvp.dietary_help",
        "Add dietary restrictions with the number of people for each",
    );
    map.insert("rsvp.dietary_remaining", "remaining");
    map.insert("rsvp.notes", "Additional Notes");
    map.insert("rsvp.submit", "Submit RSVP");
    map.insert("rsvp.update", "Update RSVP");
    map.insert("rsvp.form_title_new", "Complete Your RSVP");
    map.insert("rsvp.form_title_update", "Update Your RSVP");
    map.insert("rsvp.welcome", "Welcome");
    map.insert(
        "rsvp.both_events",
        "You are invited to both events. Please complete your RSVP for each location separately.",
    );
    map.insert("rsvp.party_size", "Party size");
    map.insert("rsvp.guests_label", "guest(s)");
    map.insert("rsvp.success", "Thank you! Your RSVP has been saved.");
    map.insert("rsvp.success_thank_you", "Thank you for your response!");
    map.insert("rsvp.success_refresh", "The page will reload in 5 seconds.");
    map.insert("rsvp.error", "Something went wrong. Please try again.");
    map.insert(
        "rsvp.error_code_required",
        "Please enter your invitation code",
    );
    map.insert("rsvp.error_loading", "Error loading RSVP");
    map.insert(
        "rsvp.error_code_invalid",
        "Invitation code not found. Please check your code and try again.",
    );
    map.insert(
        "rsvp.error_network",
        "Network error. Please check your connection and try again.",
    );
    map.insert(
        "rsvp.error_generic",
        "An error occurred. Please try again later.",
    );
    map.insert(
        "rsvp.error_empty_names",
        "Please fill in all guest names before submitting.",
    );
    map.insert(
        "rsvp.error_no_locations",
        "Please select at least one location for each guest.",
    );
    map.insert(
        "rsvp.not_found",
        "Guest not found. Please check your information.",
    );
    map.insert("rsvp.invitees_title", "Your Guests");
    map.insert("rsvp.invitee_name", "Guest Name");
    map.insert("rsvp.add_invitee", "Add Guest");
    map.insert("rsvp.delete_invitee", "Remove Guest");
    map.insert("rsvp.gluten_free", "Gluten Free");
    map.insert("rsvp.guest_list", "Guest List");
    map.insert(
        "rsvp.guest_list_help",
        "Add all guests in your party and their dietary preferences",
    );
    map.insert("rsvp.add_another_guest", "+ Add Another Guest");
    map.insert(
        "rsvp.notes_help",
        "Any special requests, dietary restrictions, or messages for us?",
    );
    map.insert(
        "rsvp.notes_placeholder",
        "Any special requests or messages?",
    );
    map.insert("rsvp.attending_label", "Attending:");
    map.insert("rsvp.age_category", "Age Category:");
    map.insert("rsvp.adult", "Adult");
    map.insert("rsvp.child_under_3", "< 3 years");
    map.insert("rsvp.child_under_10", "< 10 years");
    map.insert("rsvp.dietary_restrictions_label", "Dietary Restrictions:");
    map.insert("rsvp.guest_not_found", "Guest not found");

    // Admin
    map.insert("admin.title", "Admin Dashboard");
    map.insert("admin.login", "Login");
    map.insert("admin.logout", "Logout");
    map.insert("admin.dashboard", "Dashboard");
    map.insert("admin.guests", "Guests");
    map.insert("admin.rsvps", "RSVPs");

    // Common
    map.insert("common.loading", "Loading...");
    map.insert("common.save", "Save");
    map.insert("common.saving", "Saving...");
    map.insert("common.cancel", "Cancel");
    map.insert("common.delete", "Delete");
    map.insert("common.edit", "Edit");
    map.insert("common.add", "Add");
    map.insert("common.search", "Search");
    map.insert("common.filter", "Filter");
    map.insert("common.export", "Export");
    map.insert("common.back", "Back");
    map.insert("common.next", "Next");
    map.insert("common.previous", "Previous");

    // Error messages
    map.insert(
        "error.generic",
        "An unexpected error occurred. Please try again.",
    );
    map.insert(
        "error.network",
        "Network error. Please check your connection and try again.",
    );
    map.insert(
        "error.auth",
        "Authentication failed. Please check your credentials.",
    );
    map.insert("error.not_found", "The requested resource was not found.");
    map.insert("error.validation", "Please check your input and try again.");
    map.insert("error.server", "Server error. Please try again later.");
    map.insert("error.storage", "File upload error. Please try again.");
    map.insert(
        "error.session_expired",
        "Your session has expired. Please log in again.",
    );

    // Footer
    map.insert(
        "footer.copyright",
        "© 2026 - Made with ❤️ for our special day",
    );

    map
}
