import init, { count_words, count_sentences, count_chars, byte_len } from "./pkg/demo_extension.js";

async function run() {
  const wasmUrl = chrome.runtime.getURL("pkg/demo_extension_bg.wasm");
  const bytes = await fetch(wasmUrl).then(r => r.arrayBuffer());
  await init({ module_or_path: bytes });

  document.getElementById("analyze").addEventListener("click", () => {
    const text = document.getElementById("input").value;

    document.getElementById("sentences").textContent = count_sentences(text);
    document.getElementById("words").textContent = count_words(text);
    document.getElementById("chars").textContent = count_chars(text);
    document.getElementById("bytes").textContent = byte_len(text);

    document.getElementById("results").style.display = "block";
  });
}

run();
