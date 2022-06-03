import init, { 
    gfx_create_instance, 
    gfx_start,
    gfx_create_window,
    gfx_set_clear_color,
    gfx_test_struct,
    gfx_create_shader,
} from './pkg/rust_graphx.js';

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
var GFX = 0;

/**
 * Callback called once per frame
 * when graphx is ready to render.
 */
function onRender() {
    // console.log("On render called");
}

/**
 * Callback called once per frame 
 * when graphix is ready to update.
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
    
    gfx_set_clear_color(GFX, c, s, (Math.cos(time / 1000.0 + 42.0) + 1.0) / 2.0, 1.0);
}

/**
 * Callback called when GraphX must
 * initialize.
 */
function onGraphXInitialize() {
    let window_ptr = gfx_create_window();

    // Create new instance of GraphX. This
    // will return a pointer to the GraphX
    // instance created.
    GFX = gfx_create_instance(
        onUpdate,
        onRender,
        onResize,
        window_ptr,
    );

    gfx_test_struct({ name: "Hello world !"})
    gfx_create_shader("this is my shader !");
    
    // This will start the engine. And it 
    // will automatically free the pointer (
    // so C/C++ dev don't worry about memory
    // leaks 😌)
    gfx_start(GFX, window_ptr);
}

// Initialize the wasm module and then initialize GraphX...
init()
    .then(onGraphXInitialize)
    .catch(err => console.error("Failed to initialize GraphX", err));