class Plugin {
  constructor(uri, obj) {
    this.uri = uri;
    this.obj = obj;
  }

  static async load(uri) {
    const response = await fetch("plugins/" + uri);
    const buffer = await response.arrayBuffer();
    const obj = await WebAssembly.instantiate(buffer, {});

    return new Plugin(uri, obj);
  }

  process(img_data) {
    let fb_data_ptr = this.obj.instance.exports.alloc_framebuffer(img_data.width, img_data.height);

    {
      let mem = new Uint8Array(this.obj.instance.exports.memory.buffer);

      for (let i = 0; i < img_data.width * img_data.height * 4; i++)
        mem[fb_data_ptr + i] = img_data.data[i];
    }

    this.obj.instance.exports.process(img_data.width, img_data.height, fb_data_ptr, img_data.width * img_data.height * 4);

    {
      let mem = new Uint8Array(this.obj.instance.exports.memory.buffer);

      for (let i = 0; i < img_data.width * img_data.height * 4; i++)
        img_data.data[i] = mem[fb_data_ptr + i];
    }

    this.obj.instance.exports.free_framebuffer(fb_data_ptr, img_data.width, img_data.height);
  }
}

async function run() {
  const cvs = document.getElementById("cvs");
  const plugs_field = document.getElementById("input_plugs");
  let plugins = [];

  for (let plug of plugs_field.value.split("\n")) {
    let plugin = await Plugin.load(plug);

    if (!plugin) {
      alert(plug + " couldn't be loaded: " + response.statusText);
    } else {
      plugins.push(plugin)
      console.log("Loaded " + plugin.uri);
    }
  }

  console.log(plugins);

  let ctx = cvs.getContext("2d");
  ctx.clearRect(0, 0, cvs.width, cvs.height);
  let img_data = ctx.getImageData(0, 0, cvs.width, cvs.height);

  for (let plug of plugins) {
    plug.process(img_data);
  }

  ctx.putImageData(img_data, 0, 0);
}

function main() {
  const run_btn = document.getElementById("button_run");

  run_btn.addEventListener("click", run);
}

window.onload = main;
