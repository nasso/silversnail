const PLUGIN_API = {
  add: (x, y) => x + y,
  sub: (x, y) => x - y,
  mul: (x, y) => x * y,
  div: (x, y) => x / y,
};

const PLUGIN_PATHS = [
  "plug_add.wasm",
  "plug_mul.wasm",
];

const PLUGINS = [];

async function main() {
  const run_btn = document.getElementById("button_run");

  for (let plug of PLUGIN_PATHS) {
    const response = await fetch("plugins/" + plug);
    const buffer = await response.arrayBuffer();
    const obj = await WebAssembly.instantiate(buffer, { env: PLUGIN_API });
    PLUGINS.push({ name: plug, obj: obj })
  }

  run_btn.addEventListener("click", () => {
    const x = document.getElementById("input_x").value;
    const y = document.getElementById("input_y").value;
    const results = document.getElementById("results");
    results.textContent = "";

    for (let plug of PLUGINS) {
      let result = plug.obj.instance.exports.process(x, y);
      let li = document.createElement("li");

      li.textContent = "Result of " + plug.name + ": " + result;
      results.appendChild(li);
    }
  });
}

window.onload = main;
