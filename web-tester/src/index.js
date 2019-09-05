import "./index.scss";

const WIDTH = 720;
const HEIGHT = 480;

async function startRendering(renderer) {
    function renderLoop() {
        renderer.render_frame(WIDTH, HEIGHT);

        requestAnimationFrame(renderLoop);
    }

    renderLoop();
}

async function main() {
    const silversnail = await import("../../silversnail/pkg");

    // const project = new silversnail.Project();

    const renderer = new silversnail.Renderer({
        canvas: document.querySelector("#cvs"),
        width: WIDTH,
        height: HEIGHT,
    });

    startRendering(renderer);
}

window.onload = async () => {
    try {
        await main();
    } catch(e) {
        console.error(e);
    }
}
