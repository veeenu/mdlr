(function() {
  let md = document.body.attributes['data-page'].value
  let div = document.querySelector("#content")

  setInterval(async function() {
    let md_text = await (await fetch(`/render/${md}`)).text()
    console.dir(md_text)

    div.innerHTML = md_text
  }, 2000);
})();
