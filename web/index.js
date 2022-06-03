import init, { 
    ils_create_window,
    ils_create_instance,
    ils_run_instance,
    ils_set_clear_color,
} from './lib/illusion.js';

/**
 * Callback called when the canvas is resized.
 * 
 * @param {number} physical_width   The width (in pixel) of the canvas with DPI.
 * @param {number} physical_height  The height (in pixel) of the canvas with DPI.
 * @param {number} logical_width    The width (in pixel) of the canvas without DPI.
 * @param {number} logical_height   The height (in pixel) of the canvas without DPI.
 */
 function onResize(physical_width, physical_height, logical_width, logical_height) {
    console.log(`
    On resize called !
    new physical width: ${physical_width}, new physical height: ${physical_height}
    new logical width: ${logical_width}, new logical height: ${logical_height}
    `);

}

var time = 0;
var lastTime = performance.now();
var deltaTime = 0;
var acc = 0;
var fps = 0; 
var ILS_INSTANCE = 0;

/**
 * Callback called once per frame
 * when Illusion is ready to render.
 */
function onRender() {
    // console.log("On render called");
}

/**
 * Callback called once per frame 
 * when Illusion is ready to update.
 */
function onUpdate() {
    time      = performance.now();
    deltaTime = (time - lastTime) / 1000;
    lastTime  = time;
    acc += deltaTime;
    fps += 1;

    if (acc >= 1) {
        console.log(`Render time: ${fps}FPS ${deltaTime * 1000}ms`);
        acc = 0;
        fps = 0;
    }

    let c = (Math.cos(time / 1000.0 + 298) + 1.0) / 2.0;
    let s = (Math.sin(time / 1000.0 + 0.5) + 1.0) / 2.0;
    
    ils_set_clear_color(ILS_INSTANCE, c, s, (Math.cos(time / 1000.0 + 42.0) + 1.0) / 2.0, 1.0);
}

/**
 * Callback called when Illusion must
 * initialize.
 */
function onIllusionInitialize() {
    let window_ptr = ils_create_window(800, 600);

    // Create new instance of Illusion. This
    // will return a pointer to the Illusion
    // instance created.
    ILS_INSTANCE = ils_create_instance(window_ptr, {
        on_update: onUpdate,
        on_resize: onResize,
        on_render: onRender,
    });

    // This will start the engine. And it 
    // will automatically free the pointer (
    // so C/C++ dev don't worry about memory
    // leaks 😌)
    ils_run_instance(window_ptr, ILS_INSTANCE);
}

// Initialize the wasm module and then initialize Illusion...
init()
    .then(onIllusionInitialize)
    .catch(err => console.error("Failed to initialize Illusion", err));