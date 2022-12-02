import mermaid from "https://cdn.skypack.dev/mermaid@8.14.0";
mermaid.initialize({ 
  startOnLoad: false,
  theme: (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) ?
    "dark" : "default"
})

window.addEventListener('load', () => {
  document.querySelectorAll("pre > code.language-mermaid").forEach(el => {
    el.parentElement.classList.add('mermaid')
    el.parentElement.innerHTML = el.textContent
  })

  mermaid.init()
})
