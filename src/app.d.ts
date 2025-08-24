// See https://svelte.dev/docs/kit/types#app.d.ts

import type { Payload } from "./hooks.server";

// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: Payload | null;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
