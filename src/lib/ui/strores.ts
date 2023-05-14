import { get_current_component, onDestroy } from "svelte/internal";
import { type Readable, type Writable, writable } from "svelte/store";
import { is_client } from "svelte/internal";

export const defaultWindow = is_client ? window : undefined;
export const defaultDocument = is_client ? window.document : undefined;



export interface ConfigurableWindow {
	window?: Window;
}

export function tryOnDestroy(fn: () => void) {
	try {
		get_current_component();
		onDestroy(fn);
	} catch {
		// fail silently
	}
}


export function writableToReadable<T>({ subscribe }: Writable<T>): Readable<T> {
	return { subscribe: subscribe };
}



export function elementVisibilityStore(
	target: Element | null,
	{ window = defaultWindow }: ConfigurableWindow = {}
) {
	const store = writable(false);

	function stop() {
		window?.removeEventListener("scroll", check);
	}

	function check() {
		if (!window) return;
		if (target === null) return;

		const document = window.document;
		const rect = target.getBoundingClientRect();

		const test =
			rect.top <= (window.innerHeight || document.documentElement.clientHeight) &&
			rect.left <= (window.innerWidth || document.documentElement.clientWidth) &&
			rect.bottom >= 0 &&
			rect.right >= 0;
		store.set(test);
	}

	function start() {
		stop();
		window?.addEventListener("scroll", check, { capture: false, passive: true });
		check();
	}

	start();

	tryOnDestroy(stop);

	return { isVisible: writableToReadable(store), stop };
}