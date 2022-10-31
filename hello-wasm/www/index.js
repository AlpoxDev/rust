let module;

import("../pkg/index.js").then((r) => {
  module = r;

  r.greet("Alpox");
});

window.onAlert = (str) => {
  module?.greet(str);
};
