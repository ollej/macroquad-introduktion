document.addEventListener("DOMContentLoaded", (_) => {
  document.querySelectorAll("code").forEach((el) => {
    const highlightPattern = /\[hl(\s\d*(\-\d*)?)*\]/;

    if (highlightPattern.test(el.className)) {
      const htmlLines = el.innerHTML.split("\n");
      const className = el.className.match(highlightPattern)[0];
      const linesStr = className.replace("[", "").replace("]", "").split(" ");
      linesStr.shift();
      
      const lines = linesStr
        .map((x) => {
          if (x.includes("-")) {
            const [lower, upper] = x.split("-").map((x) => parseInt(x));
            return Array.from(
              new Array(upper - lower + 1),
              (_, i) => i + lower - 1
            );
          }
          return parseInt(x) - 1;
        })
        .flat();
      
      const newHtmlLines = htmlLines.map((x, i) =>
        !lines.includes(i) ? `<span style="opacity: 0.3;">${x}</span>` : x
      );

      el.innerHTML = newHtmlLines.join("\n");
    }
  });
});
