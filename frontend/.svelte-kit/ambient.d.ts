
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const GJS_DEBUG_TOPICS: string;
	export const LESSOPEN: string;
	export const TAURI_ENV_PLATFORM: string;
	export const LC_TIME: string;
	export const RUST_RECURSION_COUNT: string;
	export const USER: string;
	export const npm_config_user_agent: string;
	export const TAURI_CLI_VERBOSITY: string;
	export const GIT_ASKPASS: string;
	export const XDG_SESSION_TYPE: string;
	export const npm_node_execpath: string;
	export const CLUTTER_DISABLE_MIPMAPPED_TEXT: string;
	export const LD_LIBRARY_PATH: string;
	export const RUSTUP_TOOLCHAIN: string;
	export const SHLVL: string;
	export const npm_config_noproxy: string;
	export const CHROME_DESKTOP: string;
	export const HOME: string;
	export const OLDPWD: string;
	export const DESKTOP_SESSION: string;
	export const NVM_BIN: string;
	export const TERM_PROGRAM_VERSION: string;
	export const npm_package_json: string;
	export const GIO_LAUNCHED_DESKTOP_FILE: string;
	export const NVM_INC: string;
	export const GNOME_SHELL_SESSION_MODE: string;
	export const GTK_MODULES: string;
	export const HOMEBREW_PREFIX: string;
	export const OPENAI_API_KEY: string;
	export const VSCODE_GIT_ASKPASS_MAIN: string;
	export const LC_MONETARY: string;
	export const MANAGERPID: string;
	export const SSL_CERT_FILE: string;
	export const VSCODE_GIT_ASKPASS_NODE: string;
	export const npm_config_userconfig: string;
	export const npm_config_local_prefix: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const GSM_SKIP_SSH_AGENT_WORKAROUND: string;
	export const SYSTEMD_EXEC_PID: string;
	export const TAURI_ENV_TARGET_TRIPLE: string;
	export const npm_config_engine_strict: string;
	export const COLORTERM: string;
	export const GIO_LAUNCHED_DESKTOP_FILE_PID: string;
	export const COLOR: string;
	export const DEBUGINFOD_URLS: string;
	export const NVM_DIR: string;
	export const IM_CONFIG_PHASE: string;
	export const INFOPATH: string;
	export const MANDATORY_PATH: string;
	export const LOGNAME: string;
	export const JOURNAL_STREAM: string;
	export const _: string;
	export const npm_config_prefix: string;
	export const npm_config_npm_version: string;
	export const DEFAULTS_PATH: string;
	export const MEMORY_PRESSURE_WATCH: string;
	export const XDG_SESSION_CLASS: string;
	export const TERM: string;
	export const USERNAME: string;
	export const npm_config_cache: string;
	export const GNOME_DESKTOP_SESSION_ID: string;
	export const RUSTUP_HOME: string;
	export const WINDOWPATH: string;
	export const npm_config_node_gyp: string;
	export const PATH: string;
	export const HOMEBREW_CELLAR: string;
	export const INVOCATION_ID: string;
	export const SESSION_MANAGER: string;
	export const TAURI_ENV_PLATFORM_VERSION: string;
	export const NODE: string;
	export const npm_package_name: string;
	export const GDK_BACKEND: string;
	export const LC_ADDRESS: string;
	export const XDG_MENU_PREFIX: string;
	export const XDG_RUNTIME_DIR: string;
	export const DISPLAY: string;
	export const SSL_CERT_DIR: string;
	export const LANG: string;
	export const LC_TELEPHONE: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const TAURI_ENV_ARCH: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const LS_COLORS: string;
	export const TERM_PROGRAM: string;
	export const VSCODE_GIT_IPC_HANDLE: string;
	export const XAUTHORITY: string;
	export const XDG_SESSION_DESKTOP: string;
	export const XMODIFIERS: string;
	export const npm_lifecycle_script: string;
	export const ORIGINAL_XDG_CURRENT_DESKTOP: string;
	export const SSH_AUTH_SOCK: string;
	export const LC_NAME: string;
	export const SHELL: string;
	export const npm_package_version: string;
	export const npm_lifecycle_event: string;
	export const GDMSESSION: string;
	export const QT_ACCESSIBILITY: string;
	export const LESSCLOSE: string;
	export const LC_MEASUREMENT: string;
	export const GJS_DEBUG_OUTPUT: string;
	export const GPG_AGENT_INFO: string;
	export const LC_IDENTIFICATION: string;
	export const CARGO: string;
	export const QT_IM_MODULE: string;
	export const TAURI_ENV_FAMILY: string;
	export const VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
	export const npm_config_globalconfig: string;
	export const npm_config_init_module: string;
	export const PWD: string;
	export const npm_execpath: string;
	export const CARGO_HOME: string;
	export const NVM_CD_FLAGS: string;
	export const XDG_CONFIG_DIRS: string;
	export const XDG_DATA_DIRS: string;
	export const npm_config_global_prefix: string;
	export const HOMEBREW_REPOSITORY: string;
	export const LC_NUMERIC: string;
	export const npm_command: string;
	export const LC_PAPER: string;
	export const MEMORY_PRESSURE_WRITE: string;
	export const INIT_CWD: string;
	export const EDITOR: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		GJS_DEBUG_TOPICS: string;
		LESSOPEN: string;
		TAURI_ENV_PLATFORM: string;
		LC_TIME: string;
		RUST_RECURSION_COUNT: string;
		USER: string;
		npm_config_user_agent: string;
		TAURI_CLI_VERBOSITY: string;
		GIT_ASKPASS: string;
		XDG_SESSION_TYPE: string;
		npm_node_execpath: string;
		CLUTTER_DISABLE_MIPMAPPED_TEXT: string;
		LD_LIBRARY_PATH: string;
		RUSTUP_TOOLCHAIN: string;
		SHLVL: string;
		npm_config_noproxy: string;
		CHROME_DESKTOP: string;
		HOME: string;
		OLDPWD: string;
		DESKTOP_SESSION: string;
		NVM_BIN: string;
		TERM_PROGRAM_VERSION: string;
		npm_package_json: string;
		GIO_LAUNCHED_DESKTOP_FILE: string;
		NVM_INC: string;
		GNOME_SHELL_SESSION_MODE: string;
		GTK_MODULES: string;
		HOMEBREW_PREFIX: string;
		OPENAI_API_KEY: string;
		VSCODE_GIT_ASKPASS_MAIN: string;
		LC_MONETARY: string;
		MANAGERPID: string;
		SSL_CERT_FILE: string;
		VSCODE_GIT_ASKPASS_NODE: string;
		npm_config_userconfig: string;
		npm_config_local_prefix: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		GSM_SKIP_SSH_AGENT_WORKAROUND: string;
		SYSTEMD_EXEC_PID: string;
		TAURI_ENV_TARGET_TRIPLE: string;
		npm_config_engine_strict: string;
		COLORTERM: string;
		GIO_LAUNCHED_DESKTOP_FILE_PID: string;
		COLOR: string;
		DEBUGINFOD_URLS: string;
		NVM_DIR: string;
		IM_CONFIG_PHASE: string;
		INFOPATH: string;
		MANDATORY_PATH: string;
		LOGNAME: string;
		JOURNAL_STREAM: string;
		_: string;
		npm_config_prefix: string;
		npm_config_npm_version: string;
		DEFAULTS_PATH: string;
		MEMORY_PRESSURE_WATCH: string;
		XDG_SESSION_CLASS: string;
		TERM: string;
		USERNAME: string;
		npm_config_cache: string;
		GNOME_DESKTOP_SESSION_ID: string;
		RUSTUP_HOME: string;
		WINDOWPATH: string;
		npm_config_node_gyp: string;
		PATH: string;
		HOMEBREW_CELLAR: string;
		INVOCATION_ID: string;
		SESSION_MANAGER: string;
		TAURI_ENV_PLATFORM_VERSION: string;
		NODE: string;
		npm_package_name: string;
		GDK_BACKEND: string;
		LC_ADDRESS: string;
		XDG_MENU_PREFIX: string;
		XDG_RUNTIME_DIR: string;
		DISPLAY: string;
		SSL_CERT_DIR: string;
		LANG: string;
		LC_TELEPHONE: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		TAURI_ENV_ARCH: string;
		XDG_CURRENT_DESKTOP: string;
		LS_COLORS: string;
		TERM_PROGRAM: string;
		VSCODE_GIT_IPC_HANDLE: string;
		XAUTHORITY: string;
		XDG_SESSION_DESKTOP: string;
		XMODIFIERS: string;
		npm_lifecycle_script: string;
		ORIGINAL_XDG_CURRENT_DESKTOP: string;
		SSH_AUTH_SOCK: string;
		LC_NAME: string;
		SHELL: string;
		npm_package_version: string;
		npm_lifecycle_event: string;
		GDMSESSION: string;
		QT_ACCESSIBILITY: string;
		LESSCLOSE: string;
		LC_MEASUREMENT: string;
		GJS_DEBUG_OUTPUT: string;
		GPG_AGENT_INFO: string;
		LC_IDENTIFICATION: string;
		CARGO: string;
		QT_IM_MODULE: string;
		TAURI_ENV_FAMILY: string;
		VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
		npm_config_globalconfig: string;
		npm_config_init_module: string;
		PWD: string;
		npm_execpath: string;
		CARGO_HOME: string;
		NVM_CD_FLAGS: string;
		XDG_CONFIG_DIRS: string;
		XDG_DATA_DIRS: string;
		npm_config_global_prefix: string;
		HOMEBREW_REPOSITORY: string;
		LC_NUMERIC: string;
		npm_command: string;
		LC_PAPER: string;
		MEMORY_PRESSURE_WRITE: string;
		INIT_CWD: string;
		EDITOR: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
