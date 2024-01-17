fetch("http://localhost/spec/v1.json", {
    mode: "no-cors"
}).then(body => body.json()).then(spec => {
    const script = document.createElement("script");
    script.type = spec.type;
    script.src = spec.src;
    document.head.append(script);
});