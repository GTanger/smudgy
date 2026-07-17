# Locale names
locale-system = Use system language
locale-english = English (United States)
locale-traditional-chinese = Traditional Chinese (Taiwan)

# Shared actions and states
action-apply = Apply
action-back = Back
action-cancel = Cancel
action-close = Close
action-confirm = Confirm
action-connect = Connect
action-continue = Continue
action-create = Create
action-delete = Delete
action-dismiss = Dismiss
action-done = Done
action-edit = Edit
action-install = Install
action-open = Open
action-refresh = Refresh
action-remove = Remove
action-reset = Reset
action-save = Save
action-send = Send
action-sign-in = Sign in
action-sign-out = Sign out
action-update = Update
state-disabled = Disabled
state-enabled = Enabled
state-loading = Loading…
state-none = None

# Foundation and test fixture
language = Language
language-description = Choose the language used by Smudgy. Server text is not translated.
welcome-name = Welcome, { $name }.

# Settings window navigation
nav-account = Account
nav-preferences = Preferences
nav-security = Security
nav-friends = Friends
nav-licenses = Licenses

# Account and authentication
account-title = Account
account-checking = Checking account status…
account-verified = verified
account-not-verified = not verified
account-signed-in-as = Signed in as { $handle }
account-signed-in-no-nickname = Signed in (no nickname yet)
account-signed-in = Signed in.
account-verify-description = Verify your email to use cloud features (friends, sharing, sync).
account-code-placeholder = code from the email
account-email-placeholder = email
account-verify = Verify
account-email-code = Email me a code
account-recheck = Re-check
account-sign-out-everywhere = also sign out everywhere (all devices)
account-choose-nickname = Choose your nickname
account-nickname-description = This is your public handle — others find and friend you by it.
account-nickname-placeholder = nickname (3-24 letters, digits, - or _)
account-claim-handle = Claim handle
account-nickname-change-warning = Changing your nickname changes how others find you.
account-change-nickname = Change nickname
account-sign-in-description = We'll email you a one-time code — there is no password. New to Smudgy? Just enter your email; your account is created automatically.
account-have-code = I already have a code
account-check-email = Check your email
account-code-sent = We emailed a code to { $email }. Paste it below — codes expire after 15 minutes.
account-resend-code = Resend code
account-error-invalid-email = Enter a valid email address.
account-busy-emailing-code = Emailing you a code…
account-error-paste-code = Paste the code from the email.
account-error-email-for-code = Enter your email first so we know which account the code is for.
account-busy-verifying = Verifying…
account-notice-signed-in-needs-nickname = Signed in! Choose a nickname to claim your handle.
account-notice-signed-in-ready = Signed in — you're all set.
account-error-invalid-code = That code is invalid or expired — request a new one.
account-error-email-for-resend = Enter your email in the form first so we know where to send it.
account-busy-sending = Sending…
account-notice-code-resent = If that address has an account, a fresh code is on its way (it replaces any earlier one).
account-busy-saving-nickname = Saving nickname…
account-notice-nickname-changed = You are now { $handle }.
account-notice-signed-out = Signed out.
account-error-nickname-empty = Enter a nickname.
account-error-nickname-format = Nickname must be 3-24 characters: letters, digits, '-' or '_'.

# Preferences
preferences-title = Preferences
preferences-appearance = Appearance
preferences-terminal-font = Terminal font
preferences-font-size = Font size
preferences-press-enter = Press Enter to apply.
preferences-line-length = Line length
preferences-wrap-window = wrap to window
preferences-line-length-help = Maximum characters per line before wrapping; empty wraps to the window width. Press Enter to apply.
preferences-theme = Theme
preferences-scrollback = Scrollback
preferences-scrollback-help = Lines kept per session. Press Enter to apply.
preferences-hide-pane-headers = Hide panel headers unless the main menu is active
preferences-hide-pane-headers-help = Session and pane title bars show only while a window's toolbar is expanded (headers are also the drag handles for rearranging panes; dividers still resize either way). Scripts can pin a pane's header on.
preferences-input = Input
preferences-command-separator = Command separator
preferences-command-separator-help = Splits one input line into multiple commands.
preferences-raw-prefix = Raw line prefix
preferences-raw-prefix-help = Lines starting with this are sent exactly as typed, minus the prefix. Bypasses aliases and the command separator above.
preferences-command-input = Command input
preferences-command-input-help = What happens to the input box and its text after you press Enter
preferences-input-select-clear = Select all on send, clear when unfocused
preferences-input-select = Select all on send
preferences-input-clear = Clear on send
preferences-logging = Logging
preferences-logging-plain = Write session logs (plain text)
preferences-logging-raw = Also write raw logs (includes ANSI color codes)
preferences-logging-raw-help = Raw logs start with the next connection.
preferences-advanced = Advanced
preferences-advanced-scripting = Enable advanced scripting features
preferences-advanced-scripting-help = Unlocks “Remove sandbox” (run an installed package with full access, as if you wrote it) and the script inspector in the Automations window. Reopen Automations after changing this.
preferences-updates = Updates
preferences-auto-updates = Automatically check for updates
preferences-invalid-value = invalid value

# Theme editor
theme-adjust = Adjust
theme-colors = Colors
theme-override = Override: { $slot }
theme-clear-override = Clear override
theme-reset-colors = Reset all colors
theme-background = Background
theme-brightness = Brightness
theme-contrast = Contrast
theme-saturation = Saturation
theme-adjust-help = Background moves surfaces only; Contrast expands text away from the background.
theme-reset-adjustments = Reset adjustments
theme-preview = The quick brown fox

# Security and sessions
security-title = Security
security-api-keys = API keys
security-new-key = New API key — copy it now, it will not be shown again:
security-key-copied = Done — I copied it
security-no-api-keys = No API keys.
security-key-created = created { $date }
security-last-used = last used { $date }
security-never-used = never used
security-revoke = Revoke
security-create-key = Create API key
security-create-key-help = Creating a key requires a logged-in session; keys are shown once at creation.
security-create-key-login-error = Creating API keys requires being logged in (not just an API key).
security-sessions = Sessions
security-no-sessions = No active sessions.
security-session-created = created { $date }
security-session-expires = expires { $date }
security-session-unused = unused
security-revoke-current-warning = Revoking the session you're currently using signs you out.

# Friends and licenses
friends-title = Friends
friends-verify-email = Verify your email to add friends and share maps
friends-go-account = Go to Account

# Client validation labels
validation-invalid-value = invalid value

# Server text encoding
encoding-label = Server text encoding
encoding-utf8 = UTF-8 (recommended)
encoding-big5 = Big5 (Traditional Chinese legacy servers)
encoding-help = Choose Big5 only when this MUD sends legacy Traditional Chinese text. Telnet, GMCP, and MSDP control data are not transcoded.
connection-connecting = Connecting to { $address }…
connection-connected = Connected.
connection-disconnected = Disconnected.
connection-lost = Connection lost
connection-failed = Connection failed
connection-decode-warning = Some server text was not valid { $encoding } and was replaced.
connection-encode-warning = Some typed characters cannot be represented in { $encoding }; numeric character references were sent instead.

# Server and profile connection manager
servers-title = Servers
servers-loading = Loading servers…
servers-empty = No servers yet.
servers-new = + New Server
servers-get-started = Add a server to get started
servers-get-started-help = Add a server, then a profile to log in with.
servers-add-first = Add your first server
servers-select = Select a server from the list.
server-name = Name
server-name-placeholder = e.g. ArcticMud
server-host = Host
server-port = Port
server-port-help = Usually 23 or 4000.
server-add = Add server
server-edit = Edit server
server-delete = Delete server
server-save-add-profile = Save & add profile
server-confirm-delete = Are you sure you want to delete the server '{ $name }'? This cannot be undone.
server-confirm-delete-action = Yes, delete this server
server-error-port = Invalid port number. Must be between 1 and 65535.
server-error-name-empty = Server name cannot be empty.
server-error-config = Configuration error: { $error }
server-error-create = Failed to create server: { $error }
server-error-update = Failed to update server: { $error }
server-error-delete = Failed to delete server: { $error }
server-error-action-delete = Unexpected error: Cannot submit while confirming delete.
server-error-action-missing = Unexpected error: No action in progress.
server-edit-short = Edit
server-details-missing = Server details not found.

profiles-title = Profiles
profiles-saved-help = Saved logins for this server.
profiles-load-error = Couldn't load profiles for this server.
profiles-error-load = Error loading profiles for '{ $server }': { $error }
profiles-new = + New Profile
profiles-empty = Add a profile to connect
profile-name = Profile name
profile-name-placeholder = Gandalf
profile-description = Description (optional)
profile-description-placeholder = Magician
profile-add = Add profile
profile-create = Create profile
profile-edit = Edit profile
profile-delete = Delete profile
profile-confirm-delete = Are you sure you want to delete profile '{ $name }'?
profile-confirm-delete-action = Yes, delete this profile
profile-connect = Connect
profile-offline = Offline
profile-on-connect = On connect, send (optional)
profile-on-connect-placeholder = Gandalf\n$PASSWORD
profile-plaintext-notice = Auto-login text is stored in plain text on this device. Use $PASSWORD to insert a password securely.
profile-keychain-notice = Auto-login text is stored in plain text; the $PASSWORD value is kept in your OS keychain.
profile-password-saved = Password saved
profile-password-change = Change
profile-password-clear = Clear
profile-password-label = Password for $PASSWORD
profile-password-placeholder = Password
profile-error-no-server = Error: No server selected.
profile-error-password-required = Enter a password for $PASSWORD, or remove $PASSWORD from the auto-login text.
profile-error-name-empty = Profile name cannot be empty.
profile-error-config = Configuration error: { $error }
profile-error-action-delete = Unexpected error: Cannot submit while confirming delete.
profile-error-action-missing = Unexpected error: No action in progress.
profile-error-delete-no-server = Error: No server selected for deletion confirmation.
profile-error-password-clear = Failed to clear stored password: { $error }
profile-error-create = Failed to create profile: { $error }
profile-error-update = Failed to update profile: { $error }
profile-error-delete = Failed to delete profile: { $error }
