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
	/**
	* @param {string} input
	* @returns {string}
	*/
	export function urldecode(input: string): string;
	/**
	* @param {string} input
	* @returns {string}
	*/
	export function urlencode(input: string): string;
	/**
	* @param {number} input
	* @returns {string}
	*/
	export function int_to_ip(input: number): string;
	/**
	* @param {string} input
	* @returns {number}
	*/
	export function ip_to_int(input: string): number;
	/**
	* @returns {any}
	*/
	export function wasm_memory(): any;
	/**
	*/
	export enum Cell {
	  Dead,
	  Alive,
	}
	/**
	*/
	export class RandGame {
	  free(): void;
	/**
	* @returns {number}
	*/
	  width(): number;
	/**
	* @returns {number}
	*/
	  height(): number;
	/**
	* @returns {number}
	*/
	  count(): number;
	/**
	*/
	  tick(): void;
	/**
	* @param {number} cnt
	* @returns {RandGame}
	*/
	  static new(cnt: number): RandGame;
	/**
	* @returns {string}
	*/
	  render(): string;
	}
	/**
	*/
	export class RgCell {
	  free(): void;
	/**
	* @param {number} width
	* @param {number} height
	*/
	  tick(width: number, height: number): void;
	}
	/**
	*/
	export class Universe {
	  free(): void;
	/**
	* @returns {number}
	*/
	  width(): number;
	/**
	* @returns {number}
	*/
	  height(): number;
	/**
	* @returns {number}
	*/
	  cells(): number;
	/**
	*/
	  tick(): void;
	/**
	* @returns {Universe}
	*/
	  static new(): Universe;
	/**
	* @returns {string}
	*/
	  render(): string;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_rgcell_free: (a: number) => void;
  readonly rgcell_tick: (a: number, b: number, c: number) => void;
  readonly __wbg_randgame_free: (a: number) => void;
  readonly randgame_width: (a: number) => number;
  readonly randgame_height: (a: number) => number;
  readonly randgame_count: (a: number) => number;
  readonly randgame_tick: (a: number) => void;
  readonly randgame_new: (a: number) => number;
  readonly randgame_render: (a: number, b: number) => void;
  readonly hello_world: (a: number) => void;
  readonly reverse: (a: number, b: number, c: number) => void;
  readonly json_print: (a: number, b: number, c: number) => void;
  readonly unicode_to: (a: number, b: number, c: number) => void;
  readonly to_unicode: (a: number, b: number, c: number) => void;
  readonly get_timestamp: (a: number) => void;
  readonly get_time_from_unix: (a: number, b: number, c: number) => void;
  readonly urldecode: (a: number, b: number, c: number) => void;
  readonly urlencode: (a: number, b: number, c: number) => void;
  readonly int_to_ip: (a: number, b: number) => void;
  readonly ip_to_int: (a: number, b: number) => number;
  readonly wasm_memory: () => number;
  readonly __wbg_universe_free: (a: number) => void;
  readonly universe_width: (a: number) => number;
  readonly universe_height: (a: number) => number;
  readonly universe_cells: (a: number) => number;
  readonly universe_tick: (a: number) => void;
  readonly universe_new: () => number;
  readonly universe_render: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
