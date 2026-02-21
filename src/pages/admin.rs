use crate::components::admin::{AdminDashboard, GuestManagement, RsvpManagement};
use crate::contexts::AdminContext;
use crate::styles::*;
use leptos::*;
use leptos_router::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AdminTab {
    Dashboard,
    Guests,
    Rsvps,
}

#[component]
pub fn AdminPage() -> impl IntoView {
    // Provide admin context
    let admin_context = AdminContext::new();
    provide_context(admin_context);

    let (active_tab, set_active_tab) = create_signal(AdminTab::Dashboard);

    // Authentication state
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (logging_in, set_logging_in) = create_signal(false);

    // Check authentication on mount
    let admin_context_for_verify = admin_context;
    create_effect(move |_| {
        let admin_ctx = admin_context_for_verify;
        spawn_local(async move {
            admin_ctx.verify_session().await;
        });
    });

    let admin_ctx_clone = admin_context;

    view! {
        {move || {
            let admin_ctx = admin_ctx_clone;
            if admin_ctx.is_admin_authenticated() {
                // Authenticated view with layout
                view! {
                    <div class="min-h-screen flex flex-col bg-background">
                        <AdminHeader
                            admin_context=admin_ctx
                        />

                        <main class="flex-grow px-2 sm:px-4 py-4 sm:py-8">
                            <div class="max-w-5xl mx-auto">
                                {/* Tab Navigation */}
                                <div class="bg-white rounded-lg shadow-md mb-6 p-2">
                                    <div class="flex flex-wrap gap-2">
                                        <TabButton
                                            label="Dashboard"
                                            icon="📊"
                                            is_active=move || active_tab.get() == AdminTab::Dashboard
                                            on_click=move |_| set_active_tab.set(AdminTab::Dashboard)
                                        />
                                        <TabButton
                                            label="Guests"
                                            icon="👥"
                                            is_active=move || active_tab.get() == AdminTab::Guests
                                            on_click=move |_| set_active_tab.set(AdminTab::Guests)
                                        />
                                        <TabButton
                                            label="RSVPs"
                                            icon="📋"
                                            is_active=move || active_tab.get() == AdminTab::Rsvps
                                            on_click=move |_| set_active_tab.set(AdminTab::Rsvps)
                                        />
                                    </div>
                                </div>

                                {/* Tab Content */}
                                {move || match active_tab.get() {
                                    AdminTab::Dashboard => view! { <AdminDashboard /> }.into_view(),
                                    AdminTab::Guests => view! { <GuestManagement /> }.into_view(),
                                    AdminTab::Rsvps => view! { <RsvpManagement /> }.into_view(),
                                }}
                            </div>
                        </main>

                        <AdminFooter />
                    </div>
                }.into_view()
            } else {
                // Login page with same styling as invitation page
                let handle_login = {
                    let admin_context = admin_context;
                    move |_| {
                        if email.get().is_empty() || password.get().is_empty() {
                            set_error.set(Some("Please enter both email and password".to_string()));
                            return;
                        }

                        set_logging_in.set(true);
                        set_error.set(None);

                        let admin_context = admin_context;
                        let email_val = email.get();
                        let password_val = password.get();

                        spawn_local(async move {
                            match admin_context.sign_in(email_val, password_val).await {
                                Ok(_) => {
                                    set_logging_in.set(false);
                                    set_error.set(None);
                                }
                                Err(e) => {
                                    set_error.set(Some(e));
                                    set_logging_in.set(false);
                                }
                            }
                        });
                    }
                };

                view! {
                    <div class="min-h-screen flex flex-col bg-gradient-to-br from-primary-50 via-secondary-50 to-accent-50">
                        <LoginHeader />

                        <main class="flex-grow flex items-center justify-center px-4 py-12">
                            <div class="w-full max-w-3xl">
                                <div class="bg-white rounded-lg shadow-xl p-8 md:p-10 animate-fade-in">
                                    <div class="text-center mb-8">
                                        <div class="text-6xl mb-4">"🔐"</div>
                                        <h1 class="text-3xl md:text-4xl font-serif font-bold text-primary-600 mb-2">
                                            "Admin Panel"
                                        </h1>
                                        <p class="text-gray-600">
                                            "Sign in to manage your wedding"
                                        </p>
                                    </div>

                                    {move || error.get().map(|err| view! {
                                        <div class="mb-6 bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg text-sm">
                                            {err}
                                        </div>
                                    })}

                                    <form on:submit=|e| e.prevent_default() class="space-y-6">
                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                                "Email"
                                            </label>
                                            <input
                                                type="email"
                                                placeholder="admin@example.com"
                                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                                                prop:value=move || email.get()
                                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                                disabled=move || logging_in.get()
                                            />
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                                "Password"
                                            </label>
                                            <input
                                                type="password"
                                                placeholder="Enter your password"
                                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                                                prop:value=move || password.get()
                                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                                disabled=move || logging_in.get()
                                            />
                                        </div>

                                        <button
                                            type="submit"
                                            on:click=handle_login
                                            class=BUTTON_PRIMARY
                                            disabled=move || logging_in.get() || email.get().is_empty() || password.get().is_empty()
                                        >
                                            {move || if logging_in.get() {
                                                "Signing in..."
                                            } else {
                                                "Sign In"
                                            }}
                                        </button>
                                    </form>

                                    <div class="mt-8 pt-6 border-t border-gray-200 text-center">
                                        <p class="text-sm text-gray-600 flex items-center justify-center gap-2">
                                            <span class="text-lg">"🔐"</span>
                                            "Secured with Supabase Auth"
                                        </p>
                                    </div>
                                </div>

                                <div class="mt-6 text-center">
                                    <A href="/" class="text-primary-600 hover:text-primary-700 text-sm font-medium transition-colors">
                                        "← Back to Wedding Site"
                                    </A>
                                </div>
                            </div>
                        </main>

                        <LoginFooter />
                    </div>
                }.into_view()
            }
        }}
    }
}

#[component]
fn AdminHeader(admin_context: AdminContext) -> impl IntoView {
    let admin_ctx_for_email = admin_context;
    let admin_ctx_for_logout = admin_context;

    view! {
        <header class="bg-white shadow-md sticky top-0 z-50">
            <nav class="container mx-auto max-w-5xl px-4 py-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                        <span class="text-3xl">"💍"</span>
                        <div>
                            <h1 class="text-xl font-serif font-bold text-primary-600">
                                "Wedding Admin"
                            </h1>
                            <p class="text-xs text-gray-500">
                                "Management Dashboard"
                            </p>
                        </div>
                    </div>

                    <div class="flex items-center space-x-4">
                        <div class="hidden md:block text-right">
                            <p class="text-sm text-gray-600">
                                {move || admin_ctx_for_email.get_email().unwrap_or_default()}
                            </p>
                            <p class="text-xs text-gray-500">"Administrator"</p>
                        </div>
                        <button
                            on:click=move |_| {
                                let admin_context = admin_ctx_for_logout;
                                spawn_local(async move {
                                    admin_context.sign_out().await;
                                });
                            }
                            class="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-colors duration-200 text-sm font-medium"
                        >
                            "Logout"
                        </button>
                    </div>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn AdminFooter() -> impl IntoView {
    view! {
        <footer class="bg-white border-t border-gray-200 mt-12">
            <div class="container mx-auto max-w-5xl px-4 py-6">
                <div class="text-center text-gray-600">
                    <p class="text-sm">
                        "Wedding Admin Panel • Powered by Supabase"
                    </p>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn LoginHeader() -> impl IntoView {
    view! {
        <header class="bg-white/80 backdrop-blur-sm shadow-md">
            <nav class="container mx-auto max-w-5xl px-4 py-4">
                <div class="flex items-center justify-center">
                    <div class="flex items-center space-x-3">
                        <span class="text-3xl">"💍"</span>
                        <h1 class="text-2xl font-serif font-bold text-primary-600">
                            "Our Wedding"
                        </h1>
                    </div>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn LoginFooter() -> impl IntoView {
    view! {
        <footer class="bg-white/80 backdrop-blur-sm border-t border-gray-200">
            <div class="container mx-auto max-w-5xl px-4 py-6">
                <div class="text-center text-gray-600">
                    <p class="text-sm">
                        "© 2026 • Made with ❤️"
                    </p>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn TabButton(
    label: &'static str,
    icon: &'static str,
    is_active: impl Fn() -> bool + 'static,
    on_click: impl Fn(leptos::ev::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <button
            on:click=on_click
            class=move || {
                let base = "flex items-center space-x-2 px-4 py-2 rounded-lg font-medium transition-all duration-200 ";
                if is_active() {
                    format!("{}bg-primary-500 text-white shadow-md transform scale-105", base)
                } else {
                    format!("{}text-gray-700 hover:bg-primary-50 hover:text-primary-600", base)
                }
            }
        >
            <span class="text-xl">{icon}</span>
            <span>{label}</span>
        </button>
    }
}
