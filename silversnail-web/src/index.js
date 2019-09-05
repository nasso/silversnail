import "./index.scss";

async function startRendering(renderer) {
    function renderLoop() {
        renderer.render();

        requestAnimationFrame(renderLoop);
    }

    renderLoop();
}

async function main() {
    const silversnail = await import("../crate/pkg");

    // const project = new silversnail.Project();

    const renderer = new silversnail.Renderer({
        canvas: document.querySelector("#cvs"),
        width: 720,
        height: 480,
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
