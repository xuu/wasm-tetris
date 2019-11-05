
const __exports = {};
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function __wbg_error_05bc9ed1374fb872(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    console.error(varg0);
}

__exports.__wbg_error_05bc9ed1374fb872 = __wbg_error_05bc9ed1374fb872;

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* @param {number} rows
* @param {number} cols
* @param {number} block_width
* @returns {any}
*/
export function make_tetris(rows, cols, block_width) {
    return takeObject(wasm.make_tetris(rows, cols, block_width));
}

__exports.make_tetris = make_tetris;

function __widl_instanceof_CanvasRenderingContext2D(idx) { return getObject(idx) instanceof CanvasRenderingContext2D ? 1 : 0; }

__exports.__widl_instanceof_CanvasRenderingContext2D = __widl_instanceof_CanvasRenderingContext2D;

function __widl_f_set_fill_style_CanvasRenderingContext2D(arg0, arg1) {
    getObject(arg0).fillStyle = getObject(arg1);
}

__exports.__widl_f_set_fill_style_CanvasRenderingContext2D = __widl_f_set_fill_style_CanvasRenderingContext2D;

function __widl_f_clear_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
}

__exports.__widl_f_clear_rect_CanvasRenderingContext2D = __widl_f_clear_rect_CanvasRenderingContext2D;

function __widl_f_fill_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
}

__exports.__widl_f_fill_rect_CanvasRenderingContext2D = __widl_f_fill_rect_CanvasRenderingContext2D;

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

function __widl_f_fill_text_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).fillText(varg1, arg3, arg4);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_fill_text_CanvasRenderingContext2D = __widl_f_fill_text_CanvasRenderingContext2D;

function __widl_f_set_font_CanvasRenderingContext2D(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).font = varg1;
}

__exports.__widl_f_set_font_CanvasRenderingContext2D = __widl_f_set_font_CanvasRenderingContext2D;

function __widl_f_set_text_align_CanvasRenderingContext2D(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).textAlign = varg1;
}

__exports.__widl_f_set_text_align_CanvasRenderingContext2D = __widl_f_set_text_align_CanvasRenderingContext2D;

function __widl_f_create_element_Document(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(getObject(arg0).createElement(varg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_create_element_Document = __widl_f_create_element_Document;

function __widl_f_set_attribute_Element(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setAttribute(varg1, varg3);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_set_attribute_Element = __widl_f_set_attribute_Element;

function __widl_f_prevent_default_Event(arg0) {
    getObject(arg0).preventDefault();
}

__exports.__widl_f_prevent_default_Event = __widl_f_prevent_default_Event;

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {

        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let writeOffset = 0;
        while (true) {
            const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
            const { read, written } = cachedTextEncoder.encodeInto(arg, view);
            writeOffset += written;
            if (read === arg.length) {
                break;
            }
            arg = arg.substring(read);
            ptr = wasm.__wbindgen_realloc(ptr, size, size += arg.length * 3);
        }
        WASM_VECTOR_LEN = writeOffset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    };
}

function __widl_f_type_Event(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).type);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

__exports.__widl_f_type_Event = __widl_f_type_Event;

function __widl_f_add_event_listener_with_event_listener_EventTarget(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).addEventListener(varg1, getObject(arg3));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_add_event_listener_with_event_listener_EventTarget = __widl_f_add_event_listener_with_event_listener_EventTarget;

function __widl_instanceof_HTMLCanvasElement(idx) { return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0; }

__exports.__widl_instanceof_HTMLCanvasElement = __widl_instanceof_HTMLCanvasElement;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function __widl_f_get_context_HTMLCanvasElement(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = getObject(arg0).getContext(varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_get_context_HTMLCanvasElement = __widl_f_get_context_HTMLCanvasElement;

function __widl_f_set_width_HTMLCanvasElement(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
}

__exports.__widl_f_set_width_HTMLCanvasElement = __widl_f_set_width_HTMLCanvasElement;

function __widl_f_set_height_HTMLCanvasElement(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
}

__exports.__widl_f_set_height_HTMLCanvasElement = __widl_f_set_height_HTMLCanvasElement;

function __widl_f_key_KeyboardEvent(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).key);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

__exports.__widl_f_key_KeyboardEvent = __widl_f_key_KeyboardEvent;

function __widl_instanceof_Window(idx) { return getObject(idx) instanceof Window ? 1 : 0; }

__exports.__widl_instanceof_Window = __widl_instanceof_Window;

function __widl_f_cancel_animation_frame_Window(arg0, arg1, exnptr) {
    try {
        getObject(arg0).cancelAnimationFrame(arg1);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_cancel_animation_frame_Window = __widl_f_cancel_animation_frame_Window;

function __widl_f_request_animation_frame_Window(arg0, arg1, exnptr) {
    try {
        return getObject(arg0).requestAnimationFrame(getObject(arg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_request_animation_frame_Window = __widl_f_request_animation_frame_Window;

function __widl_f_document_Window(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_document_Window = __widl_f_document_Window;

function __wbg_newnoargs_9fab447a311888a5(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

__exports.__wbg_newnoargs_9fab447a311888a5 = __wbg_newnoargs_9fab447a311888a5;

function __wbg_call_001e26aeb2fdef67(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__wbg_call_001e26aeb2fdef67 = __wbg_call_001e26aeb2fdef67;

function __wbg_floor_41da4fbcf96b4410(arg0) {
    return Math.floor(arg0);
}

__exports.__wbg_floor_41da4fbcf96b4410 = __wbg_floor_41da4fbcf96b4410;

function __wbg_random_e432e84e9f46b36b() {
    return Math.random();
}

__exports.__wbg_random_e432e84e9f46b36b = __wbg_random_e432e84e9f46b36b;

function __wbindgen_string_new(p, l) { return addHeapObject(getStringFromWasm(p, l)); }

__exports.__wbindgen_string_new = __wbindgen_string_new;

function __wbindgen_debug_string(i, len_ptr) {
    const debug_str =
    val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}

__exports.__wbindgen_debug_string = __wbindgen_debug_string;

function __wbindgen_cb_drop(i) {
    const obj = takeObject(i).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}

__exports.__wbindgen_cb_drop = __wbindgen_cb_drop;

const __wbindgen_cb_forget = dropObject;

__exports.__wbindgen_cb_forget = __wbindgen_cb_forget;

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

__exports.__wbindgen_throw = __wbindgen_throw;

function __wbindgen_closure_wrapper68(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(22);
    const d = wasm.__wbg_function_table.get(23);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper68 = __wbindgen_closure_wrapper68;

function __wbindgen_closure_wrapper70(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(26);
    const d = wasm.__wbg_function_table.get(23);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, arg0);

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper70 = __wbindgen_closure_wrapper70;

function __wbindgen_closure_wrapper72(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(22);
    const d = wasm.__wbg_function_table.get(23);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper72 = __wbindgen_closure_wrapper72;

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module) {
    let result;
    const imports = { './wasm_tetris': __exports };
    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;

