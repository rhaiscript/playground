import("./playground").catch(console.error).then(module => {
    document.getElementById("loading").remove();
    document.getElementById("topContainer").classList.remove("hidden");
    module.initEditor();
})
