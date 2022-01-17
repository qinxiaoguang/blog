declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* @returns {string}
	*/
	export function hello_world(): string;
	/**
	* @param {string} s
	* @returns {string}
	*/
	export function reverse(s: string): string;
	/**
	* @param {string} input
	* @returns {string}
	*/
	export function json_print(input: string): string;
	/**
	* @param {string} input
	* @returns {string}
	*/
	export function unicode_to(input: string): string;
	/**
	* @param {string} input
	* @returns {string}
	*/
	export function to_unicode(input: string): string;
	/**
	* @returns {BigInt}
	*/
	export function get_timestamp(): BigInt;
	/**
	* @param {BigInt} ts
	* @returns {string}
	*/
	export function get_time_from_unix(ts: BigInt): string;
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly hello_world: (a: number) => void;
  readonly reverse: (a: number, b: number, c: number) => void;
  readonly json_print: (a: number, b: number, c: number) => void;
  readonly unicode_to: (a: number, b: number, c: number) => void;
  readonly to_unicode: (a: number, b: number, c: number) => void;
  readonly get_timestamp: (a: number) => void;
  readonly get_time_from_unix: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
