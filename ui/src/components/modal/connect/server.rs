//! Server CRUD: async wrappers, form submission handling, and server-side views.

use iced::widget::{
    Column, Row, TextInput, button, column, pick_list, scrollable,
    space::{horizontal as horizontal_space, vertical as vertical_space},
    text,
};
use iced::{Length, Pixels, Task};
use log::warn;
use smudgy_i18n::{t, ts};

use crate::theme::Element;
use crate::theme::builtins;

use smudgy_core::models::server::{Server, ServerConfig, ServerEncoding};

use super::{
    Message, ServerCrudAction, ServerFormField, State, server_host_input_id, server_name_input_id,
    server_port_input_id,
};

// --- Server CRUD Async Wrappers ---

pub(super) async fn create_server_async(
    name: String,
    config: ServerConfig,
) -> Result<Server, String> {
    smudgy_core::models::server::create_server(&name, config) // Pass name explicitly
        .map_err(|e| e.to_string())
}

pub(super) async fn update_server_async(
    name: String,
    config: ServerConfig,
) -> Result<Server, String> {
    smudgy_core::models::server::update_server(&name, config).map_err(|e| e.to_string())
}

pub(super) async fn delete_server_async(name: String) -> Result<String, String> {
    smudgy_core::models::server::delete_server(&name)
        .map(|_| name) // Return the name on success for state update
        .map_err(|e| e.to_string())
}

// --- Update Logic ---

/// Helper function to handle server form submission.
pub(super) fn handle_submit_server_form(state: &mut State) -> Task<Message> {
    state.server_crud_error = None; // Clear previous error

    match state.server_action.clone() {
        // Clone needed for async task
        Some(ServerCrudAction::Create) => {
            let port = match state.server_form_data.port.trim().parse::<u16>() {
                Ok(p) if p > 0 => p,
                Err(_) => {
                    state.server_crud_error = Some(t!("server-error-port"));
                    return Task::none();
                }
                Ok(_) => {
                    state.server_crud_error = Some(t!("server-error-port"));
                    return Task::none();
                }
            };
            let host = state.server_form_data.host.trim();
            if host.is_empty() {
                state.server_crud_error = Some(t!("server-error-host-empty"));
                return Task::none();
            }
            let mut config = ServerConfig::new(host.to_string(), port);
            config.encoding = state.server_form_data.encoding;
            let name = state.server_form_data.name.trim().to_string();
            if name.is_empty() {
                state.server_crud_error = Some(t!("server-error-name-empty"));
                return Task::none();
            }
            if name.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != '-') {
                state.server_crud_error = Some(t!("server-error-name-format"));
                return Task::none();
            }
            Task::perform(create_server_async(name, config), Message::ServerCreated)
        }
        Some(ServerCrudAction::Edit(original_name)) => {
            let port = match state.server_form_data.port.trim().parse::<u16>() {
                Ok(p) if p > 0 => p,
                Err(_) => {
                    state.server_crud_error = Some(t!("server-error-port"));
                    return Task::none();
                }
                Ok(_) => {
                    state.server_crud_error = Some(t!("server-error-port"));
                    return Task::none();
                }
            };
            // Carry everything the form doesn't edit (the link-trust grants)
            // forward from the existing config, so an address edit never
            // silently revokes them.
            let mut config = state
                .servers
                .iter()
                .find(|s| s.name == *original_name)
                .map_or_else(|| ServerConfig::new(String::new(), 0), |s| s.config.clone());
            let host = state.server_form_data.host.trim();
            if host.is_empty() {
                state.server_crud_error = Some(t!("server-error-host-empty"));
                return Task::none();
            }
            config.host = host.to_string();
            config.port = port;
            config.encoding = state.server_form_data.encoding;
            Task::perform(
                update_server_async(original_name.clone(), config),
                Message::ServerUpdated,
            )
        }
        Some(ServerCrudAction::ConfirmDelete(_)) => {
            warn!("Error: SubmitServerForm called during ConfirmDelete state.");
            state.server_crud_error = Some(t!("server-error-action-delete"));
            Task::none()
        }
        None => {
            warn!("Error: SubmitServerForm called without a ServerCrudAction set.");
            state.server_crud_error = Some(t!("server-error-action-missing"));
            Task::none()
        }
    }
}

// --- View Logic ---

/// Renders the left rail: a small `Servers` header, the server list, and a
/// persistent `+ New Server` button pinned at the bottom.
pub(super) fn view_server_list(state: &State) -> Element<'_, Message> {
    let server_list_content: Element<Message> = if state.servers.is_empty() {
        if state.is_loading_servers {
            column![text(t!("servers-loading")).style(builtins::text::muted)]
        } else {
            // The no-servers welcome/CTA lives in the right pane; keep the
            // rail itself quiet.
            column![text(t!("servers-empty")).style(builtins::text::muted)]
        }
        .into()
    } else {
        state
            .servers
            .iter()
            .fold(Column::new().spacing(2), |col, server| {
                let is_selected = state.selected_server.as_ref() == Some(&server.name);

                let mut server_button = button(text(&server.name))
                    .width(Length::Fill)
                    .padding([6, 10]);
                server_button = if is_selected {
                    server_button.style(builtins::button::list_item_selected)
                } else {
                    server_button.style(builtins::button::list_item)
                };

                // While a profile form is open the rail selection is inert — the
                // user is mid-edit and switching servers would discard that
                // context. (`+ New Server` below stays live and resets the form.)
                if state.profile_action.is_none() {
                    server_button =
                        server_button.on_press(Message::SelectServer(server.name.clone()));
                }

                col.push(server_button)
            })
            .into()
    };

    column![
        text(t!("servers-title")).size(12).style(builtins::text::muted),
        scrollable(server_list_content).height(Length::Fill),
        button(text(t!("servers-new")))
            .width(Length::Fill)
            .padding([6, 10])
            .style(builtins::button::secondary)
            .on_press(Message::RequestCreateServer),
    ]
    .width(Length::Fixed(200.0))
    .spacing(10)
    .padding(15)
    .into()
}

/// Renders the server create/edit form.
pub(super) fn view_server_form<'a>(
    state: &'a State,
    action: &'a ServerCrudAction,
) -> Element<'a, Message> {
    match action {
        ServerCrudAction::Create => {
            // --- Create Form ---
            let name_field = column![
                text(t!("server-name")).size(13).style(builtins::text::muted),
                TextInput::new(ts!("server-name-placeholder"), &state.server_form_data.name)
                    .id(server_name_input_id())
                    .on_input(|val| Message::UpdateServerFormField(ServerFormField::Name, val))
                    .on_submit(Message::SubmitServerForm),
            ]
            .spacing(4);

            let host_field = column![
                text(t!("server-host")).size(13).style(builtins::text::muted),
                TextInput::new("mud.example.com", &state.server_form_data.host)
                    .id(server_host_input_id())
                    .on_input(|val| Message::UpdateServerFormField(ServerFormField::Host, val))
                    .on_submit(Message::SubmitServerForm),
            ]
            .spacing(4);

            let port_field = column![
                text(t!("server-port")).size(13).style(builtins::text::muted),
                TextInput::new("", &state.server_form_data.port)
                    .id(server_port_input_id())
                    .width(Length::Fixed(120.0))
                    .on_input(|val| Message::UpdateServerFormField(ServerFormField::Port, val))
                    .on_submit(Message::SubmitServerForm),
                text(t!("server-port-help")).size(12).style(builtins::text::muted),
            ]
            .spacing(4);

            let save_button = button(text(t!("server-save-add-profile")))
                .style(builtins::button::primary)
                .padding([8, 18])
                .on_press(Message::SubmitServerForm);
            let cancel_button = button(text(t!("action-cancel")))
                .style(builtins::button::secondary)
                .padding([8, 18])
                .on_press(Message::CancelServerForm);

            Column::new()
                .push(text(t!("server-add")).size(Pixels(22.0)))
                .push(name_field)
                .push(host_field)
                .push(port_field)
                .push(encoding_field(state))
                .push(server_error(state))
                .push(Row::new().push(save_button).push(cancel_button).spacing(10))
                .spacing(15)
                .into()
        }
        ServerCrudAction::Edit(name) => {
            // --- Edit Form — name is the key and stays read-only ---
            let name_field = column![
                text(t!("server-name")).size(13).style(builtins::text::muted),
                text(name).size(Pixels(16.0)),
            ]
            .spacing(4);

            let host_field = column![
                text(t!("server-host")).size(13).style(builtins::text::muted),
                TextInput::new("mud.example.com", &state.server_form_data.host)
                    .id(server_host_input_id())
                    .on_input(|val| Message::UpdateServerFormField(ServerFormField::Host, val))
                    .on_submit(Message::SubmitServerForm),
            ]
            .spacing(4);

            let port_field = column![
                text(t!("server-port")).size(13).style(builtins::text::muted),
                TextInput::new("4000", &state.server_form_data.port)
                    .id(server_port_input_id())
                    .width(Length::Fixed(120.0))
                    .on_input(|val| Message::UpdateServerFormField(ServerFormField::Port, val))
                    .on_submit(Message::SubmitServerForm),
                text(t!("server-port-help")).size(12).style(builtins::text::muted),
            ]
            .spacing(4);

            let save_button = button(text(t!("action-save")))
                .style(builtins::button::primary)
                .padding([8, 18])
                .on_press(Message::SubmitServerForm);
            let cancel_button = button(text(t!("action-cancel")))
                .style(builtins::button::secondary)
                .padding([8, 18])
                .on_press(Message::CancelServerForm);
            let delete_button = button(text(t!("server-delete")))
                .style(builtins::button::link)
                .on_press(Message::RequestConfirmDeleteServer(name.clone()));

            Column::new()
                .push(text(t!("server-edit")).size(Pixels(22.0)))
                .push(name_field)
                .push(host_field)
                .push(port_field)
                .push(encoding_field(state))
                .push(server_error(state))
                .push(Row::new().push(save_button).push(cancel_button).spacing(10))
                .push(vertical_space().height(Pixels(10.0)))
                .push(delete_button)
                .spacing(15)
                .into()
        }
        ServerCrudAction::ConfirmDelete(name) => {
            // --- Delete Confirmation ---
            let confirmation_text = text(t!("server-confirm-delete", "name" => name))
            .size(Pixels(16.0));

            let confirm_delete_button = button(text(t!("server-confirm-delete-action")))
                .style(builtins::button::secondary)
                .padding([8, 18])
                .on_press(Message::ConfirmDeleteServer(name.clone()));
            let cancel_delete_button = button(text(t!("action-cancel")))
                .style(builtins::button::secondary)
                .padding([8, 18])
                .on_press(Message::CancelServerForm);

            Column::new()
                .push(text(t!("server-delete")).size(Pixels(22.0)))
                .push(confirmation_text)
                .push(server_error(state))
                .push(
                    Row::new()
                        .push(confirm_delete_button)
                        .push(cancel_delete_button)
                        .spacing(10),
                )
                .spacing(15)
                .into()
        }
    }
}

fn encoding_field(state: &State) -> Element<'_, Message> {
    column![
        text(t!("encoding-label"))
            .size(13)
            .style(builtins::text::muted),
        pick_list(
            ServerEncoding::ALL.to_vec(),
            Some(state.server_form_data.encoding),
            Message::UpdateServerEncoding,
        )
        .width(Length::Fixed(360.0)),
        text(t!("encoding-help"))
            .size(12)
            .style(builtins::text::muted),
    ]
    .spacing(4)
    .into()
}

/// Renders the current server-form error (if any) as danger text, or an empty
/// spacer so the button row doesn't jump when an error appears/clears.
fn server_error(state: &State) -> Element<'_, Message> {
    match &state.server_crud_error {
        Some(error) => text(error).style(builtins::text::danger).into(),
        None => horizontal_space().into(),
    }
}
